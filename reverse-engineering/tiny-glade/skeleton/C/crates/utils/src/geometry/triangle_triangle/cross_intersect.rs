// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a27ff2ed53df71e21069a2cac12d820a40c458e888ab8b6e9155b2758f8af49a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cross_intersect.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4374
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":4374,"record_offset":260,"rva":44184528,"va":5412893648}
// 编译实例 RVAs: 0x2a233d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::determinant
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14436646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412893648,"raw_hex":"e8dab623ff","size":5,"soft_fail":false,"target_va":5398457248,"va":5412893889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4374,"record_offset":1816,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":138962,"source_line":86,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4374,"record_offset":708,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":138959,"source_line":50,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4374,"record_offset":892,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":138961,"source_line":64,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4374,"record_offset":424,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":117856,"source_line":20,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4374,"record_offset":3824,"parent_record_offset":3788,"ancestor_record_offset":260,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4374,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":260,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4374,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":260,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4374,"record_offset":1016,"parent_record_offset":892,"ancestor_record_offset":260,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":3684,"parent_record_offset":3548,"ancestor_record_offset":260,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":2380,"parent_record_offset":2220,"ancestor_record_offset":260,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":2004,"parent_record_offset":1968,"ancestor_record_offset":260,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":260,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":260,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4374,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":260,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":260,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":260,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_short
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1548,"parent_record_offset":1456,"ancestor_record_offset":260,"inlinee":11870,"source_line":1456,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4374,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":260,"inlinee":117857,"source_line":128,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4374,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":260,"inlinee":117858,"source_line":85,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2164,"parent_record_offset":1944,"ancestor_record_offset":260,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":5636,"parent_record_offset":1816,"ancestor_record_offset":260,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2284,"parent_record_offset":2252,"ancestor_record_offset":260,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3788,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":260,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":260,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2196,"parent_record_offset":1944,"ancestor_record_offset":260,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3548,"parent_record_offset":3164,"ancestor_record_offset":260,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":4312,"parent_record_offset":3928,"ancestor_record_offset":260,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":260,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4374,"record_offset":2316,"parent_record_offset":2284,"ancestor_record_offset":260,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::intersect::Triangle::normal
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":4374,"record_offset":1944,"parent_record_offset":1816,"ancestor_record_offset":260,"inlinee":80011,"source_line":58,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::cross_intersect::intersect_plane
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4374,"record_offset":3164,"parent_record_offset":1816,"ancestor_record_offset":260,"inlinee":138963,"source_line":71,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4374,"record_offset":924,"parent_record_offset":892,"ancestor_record_offset":260,"inlinee":117856,"source_line":20,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::triangle_triangle::utils::permute_tri_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4374,"record_offset":756,"parent_record_offset":708,"ancestor_record_offset":260,"inlinee":138960,"source_line":53,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21ECF
// 首条位置: {"module":4374,"inlinee":138959}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::permute_tri_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4374,"record_offset":756,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":138960,"source_line":53,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21ED1
// 首条位置: {"module":4374,"inlinee":138961}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4374,"record_offset":1016,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4374,"record_offset":924,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":117856,"source_line":20,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4374,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":892,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":892,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":892,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4374,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":892,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":892,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":892,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_short
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4374,"record_offset":1548,"parent_record_offset":1456,"ancestor_record_offset":1228,"inlinee":11870,"source_line":1456,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4374,"record_offset":956,"parent_record_offset":924,"ancestor_record_offset":892,"inlinee":117857,"source_line":128,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4374,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":892,"inlinee":117858,"source_line":85,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::intersect_plane
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21ED3
// 首条位置: {"module":4374,"inlinee":138963}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3468,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3788,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3548,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":4312,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3268,"parent_record_offset":3164,"ancestor_record_offset":3164,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4374,"record_offset":3824,"parent_record_offset":3788,"ancestor_record_offset":3164,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":3684,"parent_record_offset":3548,"ancestor_record_offset":3164,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":3620,"parent_record_offset":3548,"ancestor_record_offset":3164,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":3296,"parent_record_offset":3268,"ancestor_record_offset":3164,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21ED2
// 首条位置: {"module":4374,"inlinee":138962}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":5636,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::intersect::Triangle::normal
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/intersect.rs
//     evidence={"module":4374,"record_offset":1944,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":80011,"source_line":58,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::cross_intersect::intersect_plane
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs
//     evidence={"module":4374,"record_offset":3164,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":138963,"source_line":71,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::triangle_triangle::utils::orient3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/triangle_triangle/utils.rs
//     evidence={"module":4374,"record_offset":2968,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":117856,"source_line":20,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4374,"record_offset":3824,"parent_record_offset":3788,"ancestor_record_offset":1816,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4374,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":1816,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":3684,"parent_record_offset":3548,"ancestor_record_offset":1816,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":2380,"parent_record_offset":2220,"ancestor_record_offset":1816,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4374,"record_offset":2004,"parent_record_offset":1968,"ancestor_record_offset":1816,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4374,"record_offset":3004,"parent_record_offset":2968,"ancestor_record_offset":1816,"inlinee":117857,"source_line":128,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4374,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":1816,"inlinee":117858,"source_line":85,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2164,"parent_record_offset":1944,"ancestor_record_offset":1816,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2284,"parent_record_offset":2252,"ancestor_record_offset":1816,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3788,"parent_record_offset":3164,"ancestor_record_offset":1816,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":1816,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1816,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":2196,"parent_record_offset":1944,"ancestor_record_offset":1816,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":3548,"parent_record_offset":3164,"ancestor_record_offset":1816,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":4312,"parent_record_offset":3928,"ancestor_record_offset":1816,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4374,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1816,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04374.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4374,"record_offset":2316,"parent_record_offset":2284,"ancestor_record_offset":1816,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04374.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection
// 信息 ID: e5eb9c0b2aa988cd662ea015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=1816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::cross_intersect
// 信息 ID: 2a2502013db7e038ca739ee6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect @ 0x24eb87a -> 0x2a233d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::intersect_plane
// 信息 ID: 04534ff023a48b1a598a7997
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=3164; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=3872; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=3928; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=4404; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=4460; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=4764; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=4820; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=5116; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone
// 信息 ID: 03b1996b1908ad0d678d6d54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=708; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive
// 信息 ID: 4d56d1924ca1f183c966dcd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=892; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::triangle_triangle::cross_intersect::cross_intersect; module=4374, inline_record=1228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
