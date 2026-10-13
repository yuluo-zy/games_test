// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/geometry/intersect.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3eae9040dbc83119bf15e519f7f23caa7cc0bd9cb27f3be0c82ac53d3848f910
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: intersect.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1456, 1634, 4070
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::impl$1::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12397
// 首条位置: {"module":1634,"inlinee":74647}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::u32::uvec3::impl$133::hash
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec3.rs
//     evidence={"module":1634,"record_offset":3444,"parent_record_offset":3404,"ancestor_record_offset":3404,"inlinee":73707,"source_line":19,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1634,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":3404,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1634,"record_offset":3476,"parent_record_offset":3444,"ancestor_record_offset":3404,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1634,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":3404,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::points_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":252,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":376,"rva":19115376,"va":5387824496}
// 编译实例 RVAs: 0x123ad70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115B1
// 首条位置: {"module":1456,"record_offset":496,"rva":19115424,"va":5387824544}
// 编译实例 RVAs: 0x123ada0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":796,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":832,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":632,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1456,"record_offset":988,"parent_record_offset":856,"ancestor_record_offset":496,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1456,"record_offset":660,"parent_record_offset":632,"ancestor_record_offset":496,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":496,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":496,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":496,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1456,"record_offset":952,"parent_record_offset":920,"ancestor_record_offset":496,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":1056,"rva":19115600,"va":5387824720}
// 编译实例 RVAs: 0x123ae50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":1380,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":13669,"source_line":243,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":1188,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":71080,"source_line":230,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1456,"record_offset":1412,"parent_record_offset":1380,"ancestor_record_offset":1056,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1456,"record_offset":1224,"parent_record_offset":1188,"ancestor_record_offset":1056,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Triangle::intersect_triangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":1568,"rva":19115840,"va":5387824960}
// 编译实例 RVAs: 0x123af40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14306803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387824960,"raw_hex":"e8f34dda00","size":5,"soft_fail":false,"target_va":5402131840,"va":5387825032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::ray_triangle_intersection
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":1700,"rva":19115936,"va":5387825056}
// 编译实例 RVAs: 0x123afa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1456,"record_offset":2192,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":2008,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":2048,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":1852,"parent_record_offset":1700,"ancestor_record_offset":1700,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1456,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1700,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1456,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1700,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1456,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":1700,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1456,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1700,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::Sphere::as_f32x4
// 记录形式: compiled_procedure; instances=1
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":2400,"rva":14420624,"va":5383129744}
// 编译实例 RVAs: 0xdc0a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::SphereIntersection::closest_intersection_distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1456,"record_offset":2520,"rva":19116544,"va":5387825664}
// 编译实例 RVAs: 0x123b200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1456,"record_offset":2684,"parent_record_offset":2520,"ancestor_record_offset":2520,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::intersect::ray_sphere_intersection
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115AA
// 首条位置: {"module":1456,"inlinee":71082}
// 编译实例 RVAs: 0x123b230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1456,"record_offset":2852,"parent_record_offset":2712,"ancestor_record_offset":2712,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01456.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1456,"record_offset":2884,"parent_record_offset":2712,"ancestor_record_offset":2712,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01456.jsonl.gz"}
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
// 完整结构及原始类型记录: intersect.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::Intersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::Sphere
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::intersect::Sphere", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>", "alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > >", "constructor": "itertools::Itertools::multiunzip", "arguments": ["core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>", "tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::SphereIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::intersect::Triangle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "constructor": "core::slice::iter::Iter", "arguments": ["utils::geometry::intersect::Triangle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals::closure_env$0<true> >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals::closure_env$0<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals::closure_env$0<true> >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals::closure_env$0<true> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >", "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>", "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "alloc::alloc::Global", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > >", "constructor": "core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0", "arguments": ["core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > >", "constructor": "core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0", "arguments": ["array$<glam::f32::vec3::Vec3,3>", "tuple$<>", "core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["array$<glam::f32::vec3::Vec3,3>", "core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3", "tuple$<>", "core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >", "alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>", "country_core::systems::collision::world::impl$20::new_unindexed::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$1>,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$1>", "nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> >,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$1>,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> >", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$1>,nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$2>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>", "array$<glam::f32::vec3::Vec3,3>", "country_core::systems::collision::world::impl$20::new_unindexed::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::intersect::Triangle", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>", "array$<glam::f32::vec3::Vec3,3>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["array$<f32,3>", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<utils::geometry::intersect::Triangle>", "array$<glam::f32::vec3::Vec3,3>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<utils::geometry::intersect::Triangle>", "constructor": "slice2$", "arguments": ["utils::geometry::intersect::Triangle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<slice2$<utils::geometry::intersect::Triangle> >", "constructor": "ref$", "arguments": ["slice2$<utils::geometry::intersect::Triangle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > >", "constructor": "tuple$", "arguments": ["ref$<slice2$<utils::geometry::intersect::Triangle> >", "ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >", "constructor": "utils::calculate_hash", "arguments": ["tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["array$<f32,3>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["array$<f32,3>", "alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["array$<f32,3>", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>", "country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl$1::triangles::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::geometry::intersect::Triangle", "core::slice::iter::ChunksExact<u32>", "country_core::render::mesh::impl$0::as_triangles::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$0::as_triangles::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::geometry::intersect::Triangle", "core::slice::iter::Chunks<usize>", "country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<usize>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::Hash::hash_slice<utils::geometry::intersect::Triangle,std::hash::random::DefaultHasher>", "constructor": "core::hash::Hash::hash_slice", "arguments": ["utils::geometry::intersect::Triangle", "std::hash::random::DefaultHasher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::iter::impl$13::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$2> >", "constructor": "alloc::boxed::iter::impl$13::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<utils::geometry::intersect::Triangle,alloc::alloc::Global>,array$<glam::f32::vec3::Vec3,3>,country_core::systems::collision::world::impl$20::new_unindexed::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::geometry::intersect::Triangle", "alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::geometry::intersect::Triangle", "core::ops::range::Range<usize>", "system_color::ui_color_gates::ui_color_gates::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: intersect.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::intersect::Sphere", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: intersect.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Sphere::as_f32x4
// 信息 ID: 2b8e88808fb9e164ad7c080b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::trimesh::TrimeshBuilder::build @ 0x129eb71 -> 0xdc0a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_plaster @ 0x207b642 -> 0xdc0a90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::SphereIntersection::closest_intersection_distance
// 信息 ID: 97ea99f50624232bdf095974
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<ref$<slice2$<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,ref$<country_core::systems::ui::resources::widget_params::WidgetParams> > > >,tuple$<country_core::systems::ui::resources::ui_state::WidgetId,ref$<country_core::systems::ui::resources::widget_params::WidgetParams> >,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$2,tuple$<alloc::vec::Vec<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global>,alloc::vec::Vec<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::alloc::Global> > > > > @ 0xac4195 -> 0x123b200 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<ref$<slice2$<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,ref$<country_core::systems::ui::resources::widget_params::WidgetParams> > > >,tuple$<country_core::systems::ui::resources::ui_state::WidgetId,ref$<country_core::systems::ui::resources::widget_params::WidgetParams> >,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$2,tuple$<alloc::vec::Vec<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global>,alloc::vec::Vec<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::alloc::Global> > > > > @ 0xac41c8 -> 0x123b200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::aabb
// 信息 ID: e2714ba7d6a0d4cda0e845d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b113 -> 0x123ae50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b144 -> 0x123ae50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::intersect_triangle
// 信息 ID: 6d1ff021014aa3a2bca9a94b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b0f7 -> 0x123af40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::new
// 信息 ID: 23f71e0784b9532e8a54f5dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::ui_arch_controls::ui_arch_controls @ 0x7675e4 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_arch_controls::ui_arch_controls @ 0x76765a -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls @ 0x772209 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls @ 0x77228b -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x8d8164 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x8d8274 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris @ 0x8e8875 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris @ 0x8e891e -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris @ 0x8e839b -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris @ 0x8e8456 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo @ 0x8e9660 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo @ 0x8e9726 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo @ 0x8e908e -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo @ 0x8e916e -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute @ 0x8ea0ec -> 0x123ad70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute @ 0x8ea1c6 -> 0x123ad70 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::normal
// 信息 ID: 3371b325141d977c12ea1f33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x8d816f -> 0x123ada0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::construct_arch_geo @ 0x8d827f -> 0x123ada0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b237 -> 0x123ada0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b243 -> 0x123ada0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes @ 0x1455fba -> 0x123ada0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a5022 -> 0x123ada0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=1172; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4070, inline_record=1752; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::Triangle::points_mut
// 信息 ID: 73c6ed35790de9819516e4cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::impl$1::hash
// 信息 ID: 4d082e12dbbc73afcb82265c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::Hash::hash_slice<utils::geometry::intersect::Triangle,std::hash::random::DefaultHasher>; module=1634, inline_record=3404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::ray_sphere_intersection
// 信息 ID: 5fc2df21a6f9877cfa55842e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<ref$<slice2$<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,ref$<country_core::systems::ui::resources::widget_params::WidgetParams> > > >,tuple$<country_core::systems::ui::resources::ui_state::WidgetId,ref$<country_core::systems::ui::resources::widget_params::WidgetParams> >,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$2,tuple$<alloc::vec::Vec<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global>,alloc::vec::Vec<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::alloc::Global> > > > > @ 0xac4178 -> 0x123b230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::intersect::ray_triangle_intersection
// 信息 ID: 5b9aac5e77a5306bc69e821c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
