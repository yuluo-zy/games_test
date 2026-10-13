// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/shapes/shape_placement_signifiers.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2134c205c3e49d3f68827c7bdad60532f166d264d3bf5c986541bfc0c6cc6fbd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_placement_signifiers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 557, 1643, 1682
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// 首条位置: {"module":557,"record_offset":22240,"rva":11475072,"va":5380184192}
// 编译实例 RVAs: 0xaf1880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-4505731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e87d3fbbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380184430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10945625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8a7fb58ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380186228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3139477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e895e72f00","size":5,"soft_fail":false,"target_va":5383324800,"va":5380185318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416171c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11686774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e87653b200","size":5,"soft_fail":false,"target_va":5391872448,"va":5380185669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15185365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8d5b5e700","size":5,"soft_fail":false,"target_va":5395370064,"va":5380184694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196ecd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15192173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e86dd0e700","size":5,"soft_fail":false,"target_va":5395377360,"va":5380185182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196f170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15193508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8a4d5e700","size":5,"soft_fail":false,"target_va":5395378544,"va":5380185031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196f620
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15194049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8c1d7e700","size":5,"soft_fail":false,"target_va":5395379744,"va":5380185690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15200013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e80defe700","size":5,"soft_fail":false,"target_va":5395384752,"va":5380184734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141970c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15200932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8a4f2e700","size":5,"soft_fail":false,"target_va":5395385440,"va":5380184503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141970e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15200979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8d3f2e700","size":5,"soft_fail":false,"target_va":5395385856,"va":5380184872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141971550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15203332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e804fce700","size":5,"soft_fail":false,"target_va":5395387728,"va":5380184391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36586206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8de422e02","size":5,"soft_fail":false,"target_va":5416771568,"va":5380185357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36872887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8b7a23202","size":5,"soft_fail":false,"target_va":5417057840,"va":5380184948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4438603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e84bba4300","size":5,"soft_fail":false,"target_va":5384624080,"va":5380185472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t36436233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e809f92b02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380185970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36435614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e89ef62b02","size":5,"soft_fail":false,"target_va":5416621762,"va":5380186143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3121956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e824a32f00","size":5,"soft_fail":false,"target_va":5383306800,"va":5380184839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3137993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380184192,"raw_hex":"e8c9e12f00","size":5,"soft_fail":false,"target_va":5383322368,"va":5380184370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":557,"record_offset":23760,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":23108,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":557,"record_offset":25260,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":22496,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":557,"record_offset":24108,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":557,"record_offset":25176,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":557,"record_offset":23140,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::expand
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":557,"record_offset":22464,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":33086,"source_line":220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":25344,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":23792,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":23392,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":557,"record_offset":23332,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":557,"record_offset":23168,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":26087,"source_line":191,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":557,"record_offset":23852,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":22708,"parent_record_offset":22684,"ancestor_record_offset":22240,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":22732,"parent_record_offset":22708,"ancestor_record_offset":22240,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":22756,"parent_record_offset":22732,"ancestor_record_offset":22240,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":22684,"parent_record_offset":22592,"ancestor_record_offset":22240,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":22620,"parent_record_offset":22592,"ancestor_record_offset":22240,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":22592,"parent_record_offset":22568,"ancestor_record_offset":22240,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":22568,"parent_record_offset":22544,"ancestor_record_offset":22240,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":23436,"parent_record_offset":23392,"ancestor_record_offset":22240,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":25428,"parent_record_offset":25372,"ancestor_record_offset":22240,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":25284,"parent_record_offset":25260,"ancestor_record_offset":22240,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":23356,"parent_record_offset":23332,"ancestor_record_offset":22240,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":23204,"parent_record_offset":23168,"ancestor_record_offset":22240,"inlinee":26088,"source_line":626,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":23272,"parent_record_offset":23236,"ancestor_record_offset":22240,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":557,"record_offset":25308,"parent_record_offset":25284,"ancestor_record_offset":22240,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":22652,"parent_record_offset":22620,"ancestor_record_offset":22240,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":25920,"parent_record_offset":25888,"ancestor_record_offset":22240,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":25764,"parent_record_offset":25728,"ancestor_record_offset":22240,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":22240,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":557,"record_offset":25224,"parent_record_offset":25200,"ancestor_record_offset":22240,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":557,"record_offset":25460,"parent_record_offset":25428,"ancestor_record_offset":22240,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":557,"record_offset":25492,"parent_record_offset":25460,"ancestor_record_offset":22240,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":557,"record_offset":25200,"parent_record_offset":25176,"ancestor_record_offset":22240,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/shapes/shape_placement_signifiers.rs
//     evidence={"module":557,"record_offset":24140,"parent_record_offset":24108,"ancestor_record_offset":22240,"inlinee":34836,"source_line":169,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":25700,"parent_record_offset":25672,"ancestor_record_offset":22240,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":25728,"parent_record_offset":25700,"ancestor_record_offset":22240,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":25888,"parent_record_offset":25700,"ancestor_record_offset":22240,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":557,"record_offset":23940,"parent_record_offset":23916,"ancestor_record_offset":22240,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":25372,"parent_record_offset":25344,"ancestor_record_offset":22240,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":25400,"parent_record_offset":25372,"ancestor_record_offset":22240,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":25536,"parent_record_offset":25372,"ancestor_record_offset":22240,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":557,"record_offset":23916,"parent_record_offset":23852,"ancestor_record_offset":22240,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":557,"record_offset":25672,"parent_record_offset":25372,"ancestor_record_offset":22240,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00557.jsonl.gz"}
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
// 首条位置: {"module":557,"record_offset":26928,"rva":11477632,"va":5380186752}
// 编译实例 RVAs: 0xaf2280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-4508264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e89835bbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380186947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-4509151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e82132bbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380187850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11464745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e8d70f51ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380186932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6708051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e8ada499ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380187086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1702167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e817f91900","size":5,"soft_fail":false,"target_va":5381890784,"va":5380188612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3137580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e82ce02f00","size":5,"soft_fail":false,"target_va":5383325024,"va":5380187439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13709117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e83d2fd100","size":5,"soft_fail":false,"target_va":5393896944,"va":5380187822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141971340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15199874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e882eee700","size":5,"soft_fail":false,"target_va":5395387200,"va":5380187321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36869357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e8ed943202","size":5,"soft_fail":false,"target_va":5417057056,"va":5380187694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36432400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e810ea2b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380188364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4607940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e8c44f4600","size":5,"soft_fail":false,"target_va":5384794864,"va":5380186919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36433758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e85eef2b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380187031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36433887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e8dfef2b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380187708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36433794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e882ef2b02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380188409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36433317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e8a5ed2b02","size":5,"soft_fail":false,"target_va":5416621762,"va":5380188440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380186752,"raw_hex":"e81df1ffff","size":5,"soft_fail":false,"target_va":5380184192,"va":5380187998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":557,"record_offset":27176,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":557,"record_offset":27260,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":557,"record_offset":31996,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":557,"record_offset":27148,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":557,"record_offset":31008,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":31064,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":557,"record_offset":29308,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":557,"record_offset":28300,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":557,"record_offset":27484,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":557,"record_offset":31036,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":29796,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":29336,"parent_record_offset":26928,"ancestor_record_offset":26928,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":27380,"parent_record_offset":27356,"ancestor_record_offset":26928,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":27404,"parent_record_offset":27380,"ancestor_record_offset":26928,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":31240,"parent_record_offset":31216,"ancestor_record_offset":26928,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":31264,"parent_record_offset":31240,"ancestor_record_offset":26928,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":27428,"parent_record_offset":27404,"ancestor_record_offset":26928,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":31288,"parent_record_offset":31264,"ancestor_record_offset":26928,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":27356,"parent_record_offset":27324,"ancestor_record_offset":26928,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":31216,"parent_record_offset":31160,"ancestor_record_offset":26928,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":27324,"parent_record_offset":27288,"ancestor_record_offset":26928,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":27288,"parent_record_offset":27260,"ancestor_record_offset":26928,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":557,"record_offset":27224,"parent_record_offset":27200,"ancestor_record_offset":26928,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":27720,"parent_record_offset":27692,"ancestor_record_offset":26928,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":31188,"parent_record_offset":31160,"ancestor_record_offset":26928,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":31160,"parent_record_offset":31136,"ancestor_record_offset":26928,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":27784,"parent_record_offset":27748,"ancestor_record_offset":26928,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":26928,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":31136,"parent_record_offset":31112,"ancestor_record_offset":26928,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":27824,"parent_record_offset":27784,"ancestor_record_offset":26928,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":27604,"parent_record_offset":27576,"ancestor_record_offset":26928,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":27632,"parent_record_offset":27604,"ancestor_record_offset":26928,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":27660,"parent_record_offset":27632,"ancestor_record_offset":26928,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":27576,"parent_record_offset":27548,"ancestor_record_offset":26928,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":557,"record_offset":27548,"parent_record_offset":27520,"ancestor_record_offset":26928,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":557,"record_offset":27520,"parent_record_offset":27484,"ancestor_record_offset":26928,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":557,"record_offset":28168,"parent_record_offset":28144,"ancestor_record_offset":26928,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":27692,"parent_record_offset":27660,"ancestor_record_offset":26928,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":29424,"parent_record_offset":29364,"ancestor_record_offset":26928,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":557,"record_offset":27848,"parent_record_offset":27824,"ancestor_record_offset":26928,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":29036,"parent_record_offset":29012,"ancestor_record_offset":26928,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":557,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":26928,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":557,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":26928,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":29740,"parent_record_offset":29716,"ancestor_record_offset":26928,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":557,"record_offset":29104,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":557,"record_offset":27896,"parent_record_offset":27872,"ancestor_record_offset":26928,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":27872,"parent_record_offset":27848,"ancestor_record_offset":26928,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":32424,"parent_record_offset":32396,"ancestor_record_offset":26928,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":557,"record_offset":28836,"parent_record_offset":28808,"ancestor_record_offset":26928,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":557,"record_offset":28808,"parent_record_offset":28780,"ancestor_record_offset":26928,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":29824,"parent_record_offset":29796,"ancestor_record_offset":26928,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":29544,"parent_record_offset":29520,"ancestor_record_offset":26928,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":557,"record_offset":27200,"parent_record_offset":27176,"ancestor_record_offset":26928,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":557,"record_offset":28064,"parent_record_offset":27660,"ancestor_record_offset":26928,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":26928,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":28144,"parent_record_offset":27484,"ancestor_record_offset":26928,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":557,"record_offset":28952,"parent_record_offset":28928,"ancestor_record_offset":26928,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":557,"record_offset":29452,"parent_record_offset":29424,"ancestor_record_offset":26928,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":557,"record_offset":29480,"parent_record_offset":29452,"ancestor_record_offset":26928,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":557,"record_offset":28204,"parent_record_offset":28168,"ancestor_record_offset":26928,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":557,"record_offset":28228,"parent_record_offset":28204,"ancestor_record_offset":26928,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":557,"record_offset":28260,"parent_record_offset":28168,"ancestor_record_offset":26928,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":557,"record_offset":29060,"parent_record_offset":29036,"ancestor_record_offset":26928,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":29684,"parent_record_offset":29652,"ancestor_record_offset":26928,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":26928,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":26928,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":29604,"parent_record_offset":29580,"ancestor_record_offset":26928,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":29520,"parent_record_offset":29364,"ancestor_record_offset":26928,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":557,"record_offset":28748,"parent_record_offset":28720,"ancestor_record_offset":26928,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":557,"record_offset":28780,"parent_record_offset":28748,"ancestor_record_offset":26928,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":557,"record_offset":28720,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":557,"record_offset":28580,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":557,"record_offset":29132,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":557,"record_offset":28664,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":557,"record_offset":28524,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":557,"record_offset":29012,"parent_record_offset":28988,"ancestor_record_offset":26928,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":557,"record_offset":28360,"parent_record_offset":28336,"ancestor_record_offset":26928,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":557,"record_offset":28988,"parent_record_offset":28880,"ancestor_record_offset":26928,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":557,"record_offset":28392,"parent_record_offset":28360,"ancestor_record_offset":26928,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28928,"parent_record_offset":28904,"ancestor_record_offset":26928,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":29224,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28904,"parent_record_offset":28880,"ancestor_record_offset":26928,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28444,"parent_record_offset":28420,"ancestor_record_offset":26928,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28420,"parent_record_offset":28360,"ancestor_record_offset":26928,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28468,"parent_record_offset":28444,"ancestor_record_offset":26928,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28552,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":557,"record_offset":28880,"parent_record_offset":28468,"ancestor_record_offset":26928,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":557,"record_offset":28336,"parent_record_offset":28300,"ancestor_record_offset":26928,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":29716,"parent_record_offset":29684,"ancestor_record_offset":26928,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":29364,"parent_record_offset":29336,"ancestor_record_offset":26928,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":29276,"parent_record_offset":28300,"ancestor_record_offset":26928,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":557,"record_offset":29396,"parent_record_offset":29364,"ancestor_record_offset":26928,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":557,"record_offset":29580,"parent_record_offset":29336,"ancestor_record_offset":26928,"inlinee":27588,"source_line":68,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":34832,"rva":11480128,"va":5380189248}
// 编译实例 RVAs: 0xaf2c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4510627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e85d2cbbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380189310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4511072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e8a02abbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380189771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11467108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e89c0651ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380189295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36867602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e8128e3202","size":5,"soft_fail":false,"target_va":5417057056,"va":5380189449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36431055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e8cfe42b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380189709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36431421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e83de62b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380189368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1250879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189248,"raw_hex":"e8c1e9ecff","size":5,"soft_fail":false,"target_va":5378938736,"va":5380189610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":35864,"parent_record_offset":34832,"ancestor_record_offset":34832,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":35040,"parent_record_offset":34832,"ancestor_record_offset":34832,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":35220,"parent_record_offset":34832,"ancestor_record_offset":34832,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":35948,"parent_record_offset":34832,"ancestor_record_offset":34832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":557,"record_offset":35836,"parent_record_offset":34832,"ancestor_record_offset":34832,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":557,"record_offset":35808,"parent_record_offset":34832,"ancestor_record_offset":34832,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":35124,"parent_record_offset":35100,"ancestor_record_offset":34832,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":35148,"parent_record_offset":35124,"ancestor_record_offset":34832,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36020,"parent_record_offset":35996,"ancestor_record_offset":34832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36044,"parent_record_offset":36020,"ancestor_record_offset":34832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":35172,"parent_record_offset":35148,"ancestor_record_offset":34832,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":34832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":35100,"parent_record_offset":35068,"ancestor_record_offset":34832,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":35996,"parent_record_offset":35972,"ancestor_record_offset":34832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":35068,"parent_record_offset":35040,"ancestor_record_offset":34832,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":35972,"parent_record_offset":35948,"ancestor_record_offset":34832,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":35376,"parent_record_offset":35352,"ancestor_record_offset":34832,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":35432,"parent_record_offset":35400,"ancestor_record_offset":34832,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":35400,"parent_record_offset":35376,"ancestor_record_offset":34832,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":35468,"parent_record_offset":35432,"ancestor_record_offset":34832,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":35888,"parent_record_offset":35864,"ancestor_record_offset":34832,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":35312,"parent_record_offset":35248,"ancestor_record_offset":34832,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":35284,"parent_record_offset":35248,"ancestor_record_offset":34832,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":35912,"parent_record_offset":35888,"ancestor_record_offset":34832,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":35352,"parent_record_offset":35312,"ancestor_record_offset":34832,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":35248,"parent_record_offset":35220,"ancestor_record_offset":34832,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":557,"record_offset":35492,"parent_record_offset":35468,"ancestor_record_offset":34832,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":557,"record_offset":35572,"parent_record_offset":35516,"ancestor_record_offset":34832,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":35516,"parent_record_offset":35492,"ancestor_record_offset":34832,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":35544,"parent_record_offset":35516,"ancestor_record_offset":34832,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":557,"record_offset":35740,"parent_record_offset":35312,"ancestor_record_offset":34832,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":35764,"parent_record_offset":35740,"ancestor_record_offset":34832,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":36120,"rva":11480688,"va":5380189808}
// 编译实例 RVAs: 0xaf2e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4511181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189808,"raw_hex":"e8332abbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380189864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4511530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189808,"raw_hex":"e8d628bbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380190229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11467662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189808,"raw_hex":"e8720451ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380189849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36430608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189808,"raw_hex":"e810e32b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380190156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36430617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189808,"raw_hex":"e819e32b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380190172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1251362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380189808,"raw_hex":"e8dee7ecff","size":5,"soft_fail":false,"target_va":5378938736,"va":5380190093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":37096,"parent_record_offset":36120,"ancestor_record_offset":36120,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":36328,"parent_record_offset":36120,"ancestor_record_offset":36120,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":557,"record_offset":36508,"parent_record_offset":36120,"ancestor_record_offset":36120,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":37180,"parent_record_offset":36120,"ancestor_record_offset":36120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":557,"record_offset":37040,"parent_record_offset":36120,"ancestor_record_offset":36120,"inlinee":28661,"source_line":296,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":557,"record_offset":37068,"parent_record_offset":36120,"ancestor_record_offset":36120,"inlinee":28615,"source_line":306,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36412,"parent_record_offset":36388,"ancestor_record_offset":36120,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36436,"parent_record_offset":36412,"ancestor_record_offset":36120,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":37252,"parent_record_offset":37228,"ancestor_record_offset":36120,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":37276,"parent_record_offset":37252,"ancestor_record_offset":36120,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36460,"parent_record_offset":36436,"ancestor_record_offset":36120,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":37300,"parent_record_offset":37276,"ancestor_record_offset":36120,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":36388,"parent_record_offset":36356,"ancestor_record_offset":36120,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":37228,"parent_record_offset":37204,"ancestor_record_offset":36120,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":36356,"parent_record_offset":36328,"ancestor_record_offset":36120,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":37204,"parent_record_offset":37180,"ancestor_record_offset":36120,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":36740,"parent_record_offset":36712,"ancestor_record_offset":36120,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":36800,"parent_record_offset":36768,"ancestor_record_offset":36120,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":36768,"parent_record_offset":36740,"ancestor_record_offset":36120,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":37120,"parent_record_offset":37096,"ancestor_record_offset":36120,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":36620,"parent_record_offset":36592,"ancestor_record_offset":36120,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":36648,"parent_record_offset":36620,"ancestor_record_offset":36120,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":36676,"parent_record_offset":36648,"ancestor_record_offset":36120,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":36592,"parent_record_offset":36564,"ancestor_record_offset":36120,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":557,"record_offset":36564,"parent_record_offset":36536,"ancestor_record_offset":36120,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":557,"record_offset":36536,"parent_record_offset":36508,"ancestor_record_offset":36120,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":37144,"parent_record_offset":37120,"ancestor_record_offset":36120,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":36712,"parent_record_offset":36676,"ancestor_record_offset":36120,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":557,"record_offset":36956,"parent_record_offset":36676,"ancestor_record_offset":36120,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":36980,"parent_record_offset":36956,"ancestor_record_offset":36120,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8814
// 首条位置: {"module":557,"inlinee":34836}
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
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DEA
// 首条位置: {"module":1643,"inlinee":81386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":771584,"parent_record_offset":771540,"ancestor_record_offset":771540,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1643,"record_offset":771640,"parent_record_offset":771540,"ancestor_record_offset":771540,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":771612,"parent_record_offset":771540,"ancestor_record_offset":771540,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/01643.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line
// 信息 ID: bd4faceedeb814aad5cb9d87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers @ 0xaf275e -> 0xaf1880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers @ 0xc0c7eb -> 0xaf1880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$0
// 信息 ID: 58fc897422d189e0597be250
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=557, inline_record=24140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$1
// 信息 ID: 7f1a15eeef3654cce9249727
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure_env$1> >; module=1643, inline_record=771540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers
// 信息 ID: a6795c6b7888336b479e4eec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e403722793d94bbe12b4c47: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > @ 0x193a541 -> 0xaf2280 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> @ 0x1a2de31 -> 0xaf2280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$1
// 信息 ID: 0831e304f491a83e53bd73d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- core::ptr::drop_in_place<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xaf3893 -> 0xaf2c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$2
// 信息 ID: 7c9c73781f58dba1de68a28b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xaf3910 -> 0xaf2e70 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
