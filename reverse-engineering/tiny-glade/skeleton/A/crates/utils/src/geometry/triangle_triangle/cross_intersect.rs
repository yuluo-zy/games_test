// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 609d18e8a147fe9112d7decf3ea6ed2a99d0bf764f4c21fca29814ca28cd1540
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cross_intersect.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3829
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::cross_intersect
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3829,"record_offset":244,"rva":29572336,"va":5398281456}
// 编译实例 RVAs: 0x1c33cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-13897980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398281456,"raw_hex":"e804ef2bff","size":5,"soft_fail":false,"target_va":5384383728,"va":5398281703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398281456,"raw_hex":"e85a080000","size":5,"soft_fail":false,"target_va":5398284480,"va":5398282337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":3829,"record_offset":1056,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":111681,"source_line":86,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":3829,"record_offset":688,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":111679,"source_line":50,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3829,"record_offset":404,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":89697,"source_line":20,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":1524,"parent_record_offset":1392,"ancestor_record_offset":244,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":2652,"parent_record_offset":2540,"ancestor_record_offset":244,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":1220,"parent_record_offset":1188,"ancestor_record_offset":244,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3829,"record_offset":436,"parent_record_offset":404,"ancestor_record_offset":244,"inlinee":89698,"source_line":117,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3829,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":244,"inlinee":89699,"source_line":74,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1336,"parent_record_offset":1164,"ancestor_record_offset":244,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":4276,"parent_record_offset":1056,"ancestor_record_offset":244,"inlinee":22395,"source_line":469,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":244,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2720,"parent_record_offset":2200,"ancestor_record_offset":244,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":244,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":244,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1368,"parent_record_offset":1164,"ancestor_record_offset":244,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":3168,"parent_record_offset":2820,"ancestor_record_offset":244,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":244,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2540,"parent_record_offset":2200,"ancestor_record_offset":244,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3829,"record_offset":1488,"parent_record_offset":1456,"ancestor_record_offset":244,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::intersect::Triangle::normal
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":3829,"record_offset":1164,"parent_record_offset":1056,"ancestor_record_offset":244,"inlinee":60105,"source_line":57,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::cross_intersect::intersect_plane
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":3829,"record_offset":2200,"parent_record_offset":1056,"ancestor_record_offset":244,"inlinee":111682,"source_line":71,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3829,"record_offset":2016,"parent_record_offset":1056,"ancestor_record_offset":244,"inlinee":89697,"source_line":20,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::utils::permute_tri_right
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3829,"record_offset":736,"parent_record_offset":688,"ancestor_record_offset":244,"inlinee":111680,"source_line":53,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B43F
// 首条位置: {"module":3829,"inlinee":111679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::permute_tri_right
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3829,"record_offset":736,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":111680,"source_line":53,"archive":"_pdb/A/modules/03829.jsonl.gz"}
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
// 首条位置: {"module":3829,"record_offset":4400,"rva":29575360,"va":5398284480}
// 编译实例 RVAs: 0x1c348c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13900919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398284480,"raw_hex":"e889e32bff","size":5,"soft_fail":false,"target_va":5384383728,"va":5398284642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3829,"record_offset":4660,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3829,"record_offset":4568,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":89697,"source_line":20,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3829,"record_offset":4888,"parent_record_offset":4812,"ancestor_record_offset":4400,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3829,"record_offset":5132,"parent_record_offset":4888,"ancestor_record_offset":4400,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3829,"record_offset":4736,"parent_record_offset":4660,"ancestor_record_offset":4400,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3829,"record_offset":4812,"parent_record_offset":4736,"ancestor_record_offset":4400,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3829,"record_offset":4988,"parent_record_offset":4888,"ancestor_record_offset":4400,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3829,"record_offset":4600,"parent_record_offset":4568,"ancestor_record_offset":4400,"inlinee":89698,"source_line":117,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3829,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":4400,"inlinee":89699,"source_line":74,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::intersect_plane
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B442
// 首条位置: {"module":3829,"inlinee":111682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2476,"parent_record_offset":2200,"ancestor_record_offset":2200,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2720,"parent_record_offset":2200,"ancestor_record_offset":2200,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":3168,"parent_record_offset":2820,"ancestor_record_offset":2820,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2276,"parent_record_offset":2200,"ancestor_record_offset":2200,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2540,"parent_record_offset":2200,"ancestor_record_offset":2200,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":2588,"parent_record_offset":2540,"ancestor_record_offset":2200,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":2652,"parent_record_offset":2540,"ancestor_record_offset":2200,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":2304,"parent_record_offset":2276,"ancestor_record_offset":2200,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B441
// 首条位置: {"module":3829,"inlinee":111681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":4276,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":22395,"source_line":469,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::intersect::Triangle::normal
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":3829,"record_offset":1164,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":60105,"source_line":57,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::intersect_plane
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":3829,"record_offset":2200,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":111682,"source_line":71,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":3829,"record_offset":2016,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":89697,"source_line":20,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":1524,"parent_record_offset":1392,"ancestor_record_offset":1056,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":2652,"parent_record_offset":2540,"ancestor_record_offset":1056,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3829,"record_offset":1220,"parent_record_offset":1188,"ancestor_record_offset":1056,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3829,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":1056,"inlinee":89698,"source_line":117,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3829,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1056,"inlinee":89699,"source_line":74,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1336,"parent_record_offset":1164,"ancestor_record_offset":1056,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":1056,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2720,"parent_record_offset":2200,"ancestor_record_offset":1056,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":1056,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":1056,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1368,"parent_record_offset":1164,"ancestor_record_offset":1056,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":3168,"parent_record_offset":2820,"ancestor_record_offset":1056,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":1056,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3829,"record_offset":2540,"parent_record_offset":2200,"ancestor_record_offset":1056,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03829.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3829,"record_offset":1488,"parent_record_offset":1456,"ancestor_record_offset":1056,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03829.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
// 信息 ID: de903dfd869a3d457f4893f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=1056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::cross_intersect
// 信息 ID: 05cd8a0f806cf4af94a4f199
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect @ 0x17866d3 -> 0x1c33cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::intersect_plane
// 信息 ID: 533e9c5b02c84ad28506485c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=2200; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=2764; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=2820; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=3256; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=3312; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=3624; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=3680; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=4016; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
// 信息 ID: ccaa9e07d244f024dddb2f7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=688; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=3829, inline_record=776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
// 信息 ID: 61f05ea851b1389b3fbc5e2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::triangle_triangle::cross_intersect::cross_intersect @ 0x1c34061 -> 0x1c348c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::triangle_triangle::cross_intersect::cross_intersect @ 0x1c3406c -> 0x1c348c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
