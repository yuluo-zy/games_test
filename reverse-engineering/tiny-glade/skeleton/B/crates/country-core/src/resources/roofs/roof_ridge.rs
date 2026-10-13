// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/roofs/roof_ridge.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bb54c965635762d1fc8bd4dfc52e579bf8fd061890ddb1074ffe4c64dbcc5ee0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_ridge.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 498, 519, 556
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use emath; // namespace candidate
//   package identities: [{"name":"emath","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":15444,"rva":9316176,"va":5378025296}
// 编译实例 RVAs: 0x8e2750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: roof_ridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3908890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378025296,"raw_hex":"e81aa53b00","size":5,"soft_fail":false,"target_va":5381934528,"va":5378025633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4883996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378025296,"raw_hex":"e81c864a00","size":5,"soft_fail":false,"target_va":5382911104,"va":5378027103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33514726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378025296,"raw_hex":"e8e664ff01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378027173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378025296,"raw_hex":"e895030000","size":5,"soft_fail":false,"target_va":5378027184,"va":5378026262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3894529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378025296,"raw_hex":"e8016d3b00","size":5,"soft_fail":false,"target_va":5381920064,"va":5378025530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":498,"record_offset":19936,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":498,"record_offset":15628,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":27977,"source_line":27,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":498,"record_offset":15684,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":27979,"source_line":118,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":498,"record_offset":16096,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":27982,"source_line":72,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":17540,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":27987,"source_line":241,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":498,"record_offset":15944,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":498,"record_offset":15852,"parent_record_offset":15444,"ancestor_record_offset":15444,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":498,"record_offset":16064,"parent_record_offset":15944,"ancestor_record_offset":15444,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":498,"record_offset":16004,"parent_record_offset":15944,"ancestor_record_offset":15444,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":498,"record_offset":15976,"parent_record_offset":15944,"ancestor_record_offset":15444,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":498,"record_offset":16032,"parent_record_offset":15944,"ancestor_record_offset":15444,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":16344,"parent_record_offset":16316,"ancestor_record_offset":15444,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":498,"record_offset":19764,"parent_record_offset":19728,"ancestor_record_offset":15444,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":15444,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":498,"record_offset":16564,"parent_record_offset":16520,"ancestor_record_offset":15444,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":15444,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":15444,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":15444,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16184,"parent_record_offset":16096,"ancestor_record_offset":15444,"inlinee":27984,"source_line":186,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16124,"parent_record_offset":16096,"ancestor_record_offset":15444,"inlinee":27983,"source_line":172,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15444,"inlinee":27980,"source_line":251,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16220,"parent_record_offset":16184,"ancestor_record_offset":15444,"inlinee":26476,"source_line":177,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16252,"parent_record_offset":16184,"ancestor_record_offset":15444,"inlinee":27986,"source_line":394,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":498,"record_offset":17720,"parent_record_offset":17540,"ancestor_record_offset":15444,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":498,"record_offset":15736,"parent_record_offset":15684,"ancestor_record_offset":15444,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":498,"record_offset":17664,"parent_record_offset":17540,"ancestor_record_offset":15444,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":498,"record_offset":16152,"parent_record_offset":16096,"ancestor_record_offset":15444,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":498,"record_offset":16380,"parent_record_offset":16252,"ancestor_record_offset":15444,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":498,"record_offset":16316,"parent_record_offset":16252,"ancestor_record_offset":15444,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":14528,"rva":558544,"va":5369267664}
// 编译实例 RVAs: 0x885d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D49
// 首条位置: {"module":498,"inlinee":27977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D58
// 首条位置: {"module":498,"inlinee":27992}
// 编译实例 RVAs: 0x8e2690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":498,"record_offset":14820,"parent_record_offset":14672,"ancestor_record_offset":14672,"inlinee":27976,"source_line":42,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":14972,"parent_record_offset":14924,"ancestor_record_offset":14672,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":14672,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":14672,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":498,"record_offset":14864,"parent_record_offset":14820,"ancestor_record_offset":14672,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D48
// 首条位置: {"module":498,"inlinee":27976}
// 编译实例 RVAs: 0x8e2700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":498,"record_offset":14864,"parent_record_offset":14820,"ancestor_record_offset":14820,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":14972,"parent_record_offset":14924,"ancestor_record_offset":14820,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":14820,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":14820,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D4E
// 首条位置: {"module":498,"inlinee":27982}
// 编译实例 RVAs: 0x8e2eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_ridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3907232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378027184,"raw_hex":"e8a09e3b00","size":5,"soft_fail":false,"target_va":5381934528,"va":5378027291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4883424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378027184,"raw_hex":"e8e0834a00","size":5,"soft_fail":false,"target_va":5382911104,"va":5378027675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16184,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":27984,"source_line":186,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16124,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":27983,"source_line":172,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":498,"record_offset":20900,"parent_record_offset":19968,"ancestor_record_offset":19968,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":498,"record_offset":16152,"parent_record_offset":16096,"ancestor_record_offset":16096,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":16344,"parent_record_offset":16316,"ancestor_record_offset":16096,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":498,"record_offset":19764,"parent_record_offset":19728,"ancestor_record_offset":19604,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":16288,"parent_record_offset":16252,"ancestor_record_offset":16096,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":498,"record_offset":16564,"parent_record_offset":16520,"ancestor_record_offset":16096,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":16488,"parent_record_offset":16380,"ancestor_record_offset":16096,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":16432,"parent_record_offset":16380,"ancestor_record_offset":16096,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":16096,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16220,"parent_record_offset":16184,"ancestor_record_offset":16096,"inlinee":26476,"source_line":177,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":16252,"parent_record_offset":16184,"ancestor_record_offset":16096,"inlinee":27986,"source_line":394,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":498,"record_offset":20168,"parent_record_offset":20144,"ancestor_record_offset":19968,"inlinee":27988,"source_line":82,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":498,"record_offset":16380,"parent_record_offset":16252,"ancestor_record_offset":16096,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":498,"record_offset":16316,"parent_record_offset":16252,"ancestor_record_offset":16096,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D4B
// 首条位置: {"module":498,"inlinee":27979}
// 编译实例 RVAs: 0x8e30b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":498,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15684,"inlinee":27980,"source_line":251,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":498,"record_offset":15736,"parent_record_offset":15684,"ancestor_record_offset":15684,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":15684,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":21384,"rva":9318640,"va":5378027760}
// 编译实例 RVAs: 0x8e30f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33513841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378027760,"raw_hex":"e87161ff01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378027930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33514165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378027760,"raw_hex":"e8b562ff01","size":5,"soft_fail":false,"target_va":5411542064,"va":5378027894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":21648,"parent_record_offset":21384,"ancestor_record_offset":21384,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":21684,"parent_record_offset":21384,"ancestor_record_offset":21384,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":498,"record_offset":21712,"parent_record_offset":21384,"ancestor_record_offset":21384,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":498,"record_offset":21612,"parent_record_offset":21384,"ancestor_record_offset":21384,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] emath::inverse_lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/lib.rs
//     evidence={"module":498,"record_offset":21552,"parent_record_offset":21384,"ancestor_record_offset":21384,"inlinee":27989,"source_line":125,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":498,"record_offset":21584,"parent_record_offset":21552,"ancestor_record_offset":21384,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00498.jsonl.gz"}
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
// 完整结构及原始类型记录: roof_ridge.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::roofs::roof_ridge::RoofRidgeDims
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_ridge.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::roofs::roof_ridge::RoofRidgeDims", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_ridge.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
// 信息 ID: 721d3911c24fdd33a85d4d12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1a43039 -> 0x8e2700 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from; module=498, inline_record=14820; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y; module=498, inline_record=27244; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=498, inline_record=37744; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=96740; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom; module=556, inline_record=3104; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
// 信息 ID: 57706d8ca9b4e34436da4389
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> @ 0x19eea1c -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1a38bc9 -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir @ 0x1a39949 -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x21aa1ec -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x21ad86e -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21afc02 -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231db60 -> 0x8e2690 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y; module=498, inline_record=27212; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=498, inline_record=37708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=519, inline_record=96708; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom; module=556, inline_record=3076; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
// 信息 ID: 99abb4730181139398e22c9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 283 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x9b1e -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x18e2a -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x150f33 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151033 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151113 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x1511f1 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x2466e7 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<u64> @ 0x32df7d -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<u64> @ 0x32df8c -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::image<ref$<str$> > @ 0x32e0ec -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x32ecdd -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f368 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x32f918 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset> > @ 0x32fec8 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x3304b8 -> 0x885d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x330a6a -> 0x885d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
// 信息 ID: c323a4a1dec254f517373107
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=15628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
// 信息 ID: 4d565d7b3c6276e6acc6407d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1a38bea -> 0x8e2750 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir @ 0x1a39966 -> 0x8e2750 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x21aa209 -> 0x8e2750 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x21ad88b -> 0x8e2750 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21afc23 -> 0x8e2750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
// 信息 ID: a596ec2acf7af5a3a61e2898
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217e04 -> 0x8e30b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a3616 -> 0x8e30b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21af002 -> 0x8e30b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231db92 -> 0x8e30b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=15684; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
// 信息 ID: 97d222f091e2416f96c3c66f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::roofs::roof_ridge::impl$0::to_world_space", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x8e2b16 -> 0x8e2eb0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=16096; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=16956; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=18160; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=19020; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=498, inline_record=19604; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
// 信息 ID: d7bc5381f0c9f405277b2a5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
