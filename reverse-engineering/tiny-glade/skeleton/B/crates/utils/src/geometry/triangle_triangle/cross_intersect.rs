// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a27ff2ed53df71e21069a2cac12d820a40c458e888ab8b6e9155b2758f8af49a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cross_intersect.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 4070
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::cross_intersect
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4070,"record_offset":252,"rva":37847456,"va":5406556576}
// 编译实例 RVAs: 0x24181a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18220611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406556576,"raw_hex":"e8bdf9e9fe","size":5,"soft_fail":false,"target_va":5388336208,"va":5406556814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406556576,"raw_hex":"e827080000","size":5,"soft_fail":false,"target_va":5406559680,"va":5406557588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4070,"record_offset":1056,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":124937,"source_line":86,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4070,"record_offset":696,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":124935,"source_line":50,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4070,"record_offset":412,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":106139,"source_line":20,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4070,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":252,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":2876,"parent_record_offset":2728,"ancestor_record_offset":252,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":1576,"parent_record_offset":1444,"ancestor_record_offset":252,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":1232,"parent_record_offset":1200,"ancestor_record_offset":252,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":4070,"record_offset":444,"parent_record_offset":412,"ancestor_record_offset":252,"inlinee":82955,"source_line":120,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":4070,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":252,"inlinee":106140,"source_line":77,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1388,"parent_record_offset":1172,"ancestor_record_offset":252,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":4388,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":25348,"source_line":484,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1508,"parent_record_offset":1476,"ancestor_record_offset":252,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2960,"parent_record_offset":2304,"ancestor_record_offset":252,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1476,"parent_record_offset":1444,"ancestor_record_offset":252,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":252,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1420,"parent_record_offset":1172,"ancestor_record_offset":252,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2728,"parent_record_offset":2304,"ancestor_record_offset":252,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1200,"parent_record_offset":1172,"ancestor_record_offset":252,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":4070,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":252,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::intersect::Triangle::normal
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":4070,"record_offset":1172,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":71089,"source_line":58,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::cross_intersect::intersect_plane
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4070,"record_offset":2304,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":124938,"source_line":71,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4070,"record_offset":1644,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":106139,"source_line":20,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::utils::permute_tri_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4070,"record_offset":744,"parent_record_offset":696,"ancestor_record_offset":252,"inlinee":124936,"source_line":53,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E807
// 首条位置: {"module":4070,"inlinee":124935}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::permute_tri_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4070,"record_offset":744,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":124936,"source_line":53,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4070,"record_offset":4512,"rva":37850560,"va":5406559680}
// 编译实例 RVAs: 0x2418dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18223639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406559680,"raw_hex":"e8e9ede9fe","size":5,"soft_fail":false,"target_va":5388336208,"va":5406559842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4070,"record_offset":4772,"parent_record_offset":4512,"ancestor_record_offset":4512,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4070,"record_offset":4680,"parent_record_offset":4512,"ancestor_record_offset":4512,"inlinee":106139,"source_line":20,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4070,"record_offset":4848,"parent_record_offset":4772,"ancestor_record_offset":4512,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4070,"record_offset":5328,"parent_record_offset":5084,"ancestor_record_offset":4512,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4070,"record_offset":4924,"parent_record_offset":4848,"ancestor_record_offset":4512,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4070,"record_offset":5004,"parent_record_offset":4924,"ancestor_record_offset":4512,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4070,"record_offset":5084,"parent_record_offset":5004,"ancestor_record_offset":4512,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4070,"record_offset":5184,"parent_record_offset":5084,"ancestor_record_offset":4512,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":4070,"record_offset":4712,"parent_record_offset":4680,"ancestor_record_offset":4512,"inlinee":82955,"source_line":120,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":4070,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4512,"inlinee":106140,"source_line":77,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::intersect_plane
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E80A
// 首条位置: {"module":4070,"inlinee":124938}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2636,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2960,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2728,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2416,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4070,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":2304,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":2876,"parent_record_offset":2728,"ancestor_record_offset":2304,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":2804,"parent_record_offset":2728,"ancestor_record_offset":2304,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":2456,"parent_record_offset":2416,"ancestor_record_offset":2304,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E809
// 首条位置: {"module":4070,"inlinee":124937}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":4388,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":25348,"source_line":484,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::intersect::Triangle::normal
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":4070,"record_offset":1172,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":71089,"source_line":58,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::intersect_plane
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4070,"record_offset":2304,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":124938,"source_line":71,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4070,"record_offset":1644,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":106139,"source_line":20,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4070,"record_offset":2996,"parent_record_offset":2960,"ancestor_record_offset":1056,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":2876,"parent_record_offset":2728,"ancestor_record_offset":1056,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":1576,"parent_record_offset":1444,"ancestor_record_offset":1056,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4070,"record_offset":1232,"parent_record_offset":1200,"ancestor_record_offset":1056,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":4070,"record_offset":1684,"parent_record_offset":1644,"ancestor_record_offset":1056,"inlinee":82955,"source_line":120,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":4070,"record_offset":1712,"parent_record_offset":1684,"ancestor_record_offset":1056,"inlinee":106140,"source_line":77,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1388,"parent_record_offset":1172,"ancestor_record_offset":1056,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1508,"parent_record_offset":1476,"ancestor_record_offset":1056,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2960,"parent_record_offset":2304,"ancestor_record_offset":1056,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1476,"parent_record_offset":1444,"ancestor_record_offset":1056,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":1056,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1420,"parent_record_offset":1172,"ancestor_record_offset":1056,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":2728,"parent_record_offset":2304,"ancestor_record_offset":1056,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":4070,"record_offset":1200,"parent_record_offset":1172,"ancestor_record_offset":1056,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/04070.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":4070,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":1056,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/04070.jsonl.gz"}
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
// 完整结构及原始类型记录: cross_intersect.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cross_intersect.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cross_intersect.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
// 信息 ID: 86f79539b4c9c4b8662b9726
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=1056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::cross_intersect
// 信息 ID: 967dc741a22f8ade93dcdee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect @ 0x1fe0177 -> 0x24181a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::intersect_plane
// 信息 ID: 495ff042e87e3594a565d02e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=2304; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=3044; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=3100; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=3480; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=3536; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=3744; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=3800; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=4172; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
// 信息 ID: cee28317b69b99236a2487ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=696; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
// 信息 ID: 7e60ea09e2a245b34b6582fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::triangle_triangle::cross_intersect::cross_intersect @ 0x2418594 -> 0x2418dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::triangle_triangle::cross_intersect::cross_intersect @ 0x241859f -> 0x2418dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
