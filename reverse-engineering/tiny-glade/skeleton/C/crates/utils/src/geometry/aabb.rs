// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/geometry/aabb.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a5240108ca0552a23d00b5188658460ddaca9feb391c43cae7f112ac268e7db6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: aabb.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 512, 567, 570, 578, 596, 605, 762, 764, 1459, 1471, 1699, 2719, 4017, 4021, 4031, 4795
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::intersection_aabb_segment2
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":500,"rva":14591872,"va":5383300992}
// 编译实例 RVAs: 0xdea780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1156,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":34247,"source_line":351,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1100,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":34246,"source_line":373,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":960,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":988,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":36888,"source_line":264,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":932,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1044,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":49081,"source_line":251,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1212,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":828,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1268,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":672,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":500,"inlinee":31314,"source_line":1920,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":500,"inlinee":8911,"source_line":1916,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":500,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":500,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":500,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":500,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":500,"inlinee":36890,"source_line":265,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::min_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":500,"inlinee":49083,"source_line":252,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::intersection_aabb_segment3
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":1328,"rva":14592144,"va":5383301264}
// 编译实例 RVAs: 0xdea890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/bvec3.rs
//     evidence={"module":762,"record_offset":2116,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":33014,"source_line":73,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":2144,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":49086,"source_line":437,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":2056,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":33012,"source_line":459,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":1756,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":11811,"source_line":294,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":1944,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":14575,"source_line":340,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":1796,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":42255,"source_line":278,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":2000,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":28828,"source_line":327,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":2232,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":1672,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":49085,"source_line":1309,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":2320,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":1504,"parent_record_offset":1328,"ancestor_record_offset":1328,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":1328,"inlinee":31314,"source_line":1920,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":2084,"parent_record_offset":2056,"ancestor_record_offset":1328,"inlinee":8911,"source_line":1916,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1328,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":1328,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1700,"parent_record_offset":1672,"ancestor_record_offset":1328,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":1540,"parent_record_offset":1504,"ancestor_record_offset":1328,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1328,"inlinee":14577,"source_line":341,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$0::min_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1328,"inlinee":28830,"source_line":328,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::is_inside_bbx
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":260,"rva":14591792,"va":5383300912}
// 编译实例 RVAs: 0xdea730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_center_dims
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFD0
// 首条位置: {"module":762,"record_offset":3276,"rva":14592784,"va":5383301904}
// 编译实例 RVAs: 0xdeab10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33320208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383301904,"raw_hex":"e8106dfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383301995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3416,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3516,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3464,"parent_record_offset":3276,"ancestor_record_offset":3276,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":3488,"parent_record_offset":3464,"ancestor_record_offset":3276,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":3276,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3436,"parent_record_offset":3416,"ancestor_record_offset":3276,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_center_radius
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":3572,"rva":14592896,"va":5383302016}
// 编译实例 RVAs: 0xdeab80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33320123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383302016,"raw_hex":"e8bb6cfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383302080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3704,"parent_record_offset":3572,"ancestor_record_offset":3572,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":3724,"parent_record_offset":3704,"ancestor_record_offset":3572,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_points
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8BF2
// 首条位置: {"module":570,"inlinee":35826}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":605,"record_offset":7352,"parent_record_offset":7300,"ancestor_record_offset":7300,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":570,"record_offset":46196,"parent_record_offset":46136,"ancestor_record_offset":46136,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":570,"record_offset":46432,"parent_record_offset":46136,"ancestor_record_offset":46136,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":570,"record_offset":46396,"parent_record_offset":46136,"ancestor_record_offset":46136,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":570,"record_offset":46236,"parent_record_offset":46196,"ancestor_record_offset":46136,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":605,"record_offset":7500,"parent_record_offset":7352,"ancestor_record_offset":7300,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":46356,"parent_record_offset":46276,"ancestor_record_offset":46136,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":46324,"parent_record_offset":46276,"ancestor_record_offset":46136,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":46276,"parent_record_offset":46236,"ancestor_record_offset":46136,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4795,"record_offset":268,"rva":45189296,"va":5413898416}
// 编译实例 RVAs: 0x2b188b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: aabb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3158584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413898416,"raw_hex":"e838323000","size":5,"soft_fail":false,"target_va":5417057056,"va":5413898467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30596579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413898416,"raw_hex":"e81d222dfe","size":5,"soft_fail":false,"target_va":5383302352,"va":5413898926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":644,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":4680,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4795,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4795,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4795,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4795,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4795,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4795,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4795,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4795,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4795,"record_offset":2728,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4795,"record_offset":3336,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4795,"record_offset":3420,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4795,"record_offset":2980,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4795,"record_offset":3252,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4795,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4795,"record_offset":1076,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":10709,"source_line":2950,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4795,"record_offset":2156,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4795,"record_offset":1044,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4795,"record_offset":1780,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4795,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4795,"record_offset":916,"parent_record_offset":876,"ancestor_record_offset":268,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4795,"record_offset":876,"parent_record_offset":836,"ancestor_record_offset":268,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4795,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4795,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4795,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4795,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":1584,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":1132,"parent_record_offset":1104,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":1104,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":2072,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":1988,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4795,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4795,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4795,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":4795,"record_offset":3764,"parent_record_offset":3724,"ancestor_record_offset":268,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":4795,"record_offset":3808,"parent_record_offset":3764,"ancestor_record_offset":268,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":4795,"record_offset":3904,"parent_record_offset":3852,"ancestor_record_offset":268,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":3668,"parent_record_offset":672,"ancestor_record_offset":268,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":268,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4795,"record_offset":796,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4795,"record_offset":756,"parent_record_offset":672,"ancestor_record_offset":268,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":4795,"record_offset":836,"parent_record_offset":796,"ancestor_record_offset":268,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":4795,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":268,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4795,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":268,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4795,"record_offset":1272,"parent_record_offset":1208,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4795,"record_offset":3576,"parent_record_offset":3548,"ancestor_record_offset":268,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":4795,"record_offset":3988,"parent_record_offset":3852,"ancestor_record_offset":268,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":4060,"parent_record_offset":3668,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":1336,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4795,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":268,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4795,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4795,"record_offset":1952,"parent_record_offset":1920,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4795,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4795,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4795,"record_offset":2924,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4795,"record_offset":1244,"parent_record_offset":1208,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4795,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4795,"record_offset":1920,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4795,"record_offset":3112,"parent_record_offset":3080,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4795,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] system_color::show_color_menu::publish_palette_bounds::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/show_color_menu.rs
//     evidence={"module":4795,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":268,"inlinee":145397,"source_line":1302,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<system_color::radial_menu::RadialMenuRing>,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::show_color_options::closure_env$30> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4795,"record_offset":6712,"rva":45190928,"va":5413900048}
// 编译实例 RVAs: 0x2b18f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] system_color::radial_menu::RadialMenuRing::bounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2523990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413900048,"raw_hex":"e8aa7cd9ff","size":5,"soft_fail":false,"target_va":5411376336,"va":5413900321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":7048,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":8204,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":4795,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":6712,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":4795,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":6712,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":4795,"record_offset":7272,"parent_record_offset":7232,"ancestor_record_offset":6712,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":7116,"parent_record_offset":7076,"ancestor_record_offset":6712,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":7076,"parent_record_offset":7048,"ancestor_record_offset":6712,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4795,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":6712,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4795,"record_offset":7524,"parent_record_offset":7076,"ancestor_record_offset":6712,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":4795,"record_offset":7600,"parent_record_offset":7548,"ancestor_record_offset":6712,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":4795,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":6712,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4795,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":6712,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4795,"record_offset":7148,"parent_record_offset":7116,"ancestor_record_offset":6712,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4795,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":6712,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":4795,"record_offset":7420,"parent_record_offset":7232,"ancestor_record_offset":6712,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":4795,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":6712,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":7492,"parent_record_offset":7116,"ancestor_record_offset":6712,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":7232,"parent_record_offset":7204,"ancestor_record_offset":6712,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4795,"record_offset":7328,"parent_record_offset":7296,"ancestor_record_offset":6712,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4795,"record_offset":7680,"parent_record_offset":7624,"ancestor_record_offset":6712,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4795,"record_offset":7652,"parent_record_offset":7624,"ancestor_record_offset":6712,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4795,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":6712,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4795,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":6712,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4795,"record_offset":7384,"parent_record_offset":7296,"ancestor_record_offset":6712,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4795,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":6712,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] system_color::show_color_menu::show_color_options::closure$30
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/show_color_menu.rs
//     evidence={"module":4795,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":6712,"inlinee":145404,"source_line":1255,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,array$<glam::f32::vec2::Vec2,2>,system_color::radial_menu::icon_bounds::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4795,"record_offset":4712,"rva":45190544,"va":5413899664}
// 编译实例 RVAs: 0x2b18d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":5052,"parent_record_offset":4712,"ancestor_record_offset":4712,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":6680,"parent_record_offset":4712,"ancestor_record_offset":4712,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":6648,"parent_record_offset":4712,"ancestor_record_offset":4712,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":4795,"record_offset":5196,"parent_record_offset":5168,"ancestor_record_offset":4712,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":4795,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":4712,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":4795,"record_offset":5292,"parent_record_offset":5252,"ancestor_record_offset":4712,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":5136,"parent_record_offset":5080,"ancestor_record_offset":4712,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":4795,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":4712,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4795,"record_offset":5824,"parent_record_offset":5796,"ancestor_record_offset":4712,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4795,"record_offset":5796,"parent_record_offset":5080,"ancestor_record_offset":4712,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":4795,"record_offset":5880,"parent_record_offset":5824,"ancestor_record_offset":4712,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":4795,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":4712,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4795,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":4712,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4795,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":4712,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4795,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":4712,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4795,"record_offset":5168,"parent_record_offset":5136,"ancestor_record_offset":4712,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4795,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":4712,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":4795,"record_offset":5440,"parent_record_offset":5252,"ancestor_record_offset":4712,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":4795,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":4712,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":5512,"parent_record_offset":5136,"ancestor_record_offset":4712,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4795,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":4712,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4795,"record_offset":5348,"parent_record_offset":5316,"ancestor_record_offset":4712,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4795,"record_offset":5964,"parent_record_offset":5908,"ancestor_record_offset":4712,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4795,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":4712,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4795,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4712,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4795,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":4712,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4795,"record_offset":5404,"parent_record_offset":5316,"ancestor_record_offset":4712,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4795,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":4712,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$11::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":4712,"inlinee":145401,"source_line":1201,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":4712,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$53::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":6152,"parent_record_offset":6044,"ancestor_record_offset":4712,"inlinee":145403,"source_line":1556,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4795,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":4712,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   [pdb_inline_transitive] system_color::radial_menu::icon_bounds::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/radial_menu/mod.rs
//     evidence={"module":4795,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4712,"inlinee":145399,"source_line":386,"archive":"_pdb/C/modules/04795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 101, 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFD7
// 首条位置: {"module":762,"record_offset":4312,"rva":14593120,"va":5383302240}
// 编译实例 RVAs: 0xdeac60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":4512,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":4436,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":4312,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":4312,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::half_dims
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":5284,"rva":14593280,"va":5383302400}
// 编译实例 RVAs: 0xdead00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":5488,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":5412,"parent_record_offset":5284,"ancestor_record_offset":5284,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":5284,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":5284,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::dims
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":4000,"rva":14593072,"va":5383302192}
// 编译实例 RVAs: 0xdeac30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":4120,"parent_record_offset":4000,"ancestor_record_offset":4000,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":4000,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::area
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":3888,"rva":14593040,"va":5383302160}
// 编译实例 RVAs: 0xdeac10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::contains
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":4704,"rva":14593184,"va":5383302304}
// 编译实例 RVAs: 0xdeaca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":762,"record_offset":4828,"parent_record_offset":4704,"ancestor_record_offset":4704,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":4884,"parent_record_offset":4856,"ancestor_record_offset":4704,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":4904,"parent_record_offset":4884,"ancestor_record_offset":4704,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":762,"record_offset":4856,"parent_record_offset":4828,"ancestor_record_offset":4704,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::intersects
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFCE
// 首条位置: {"module":762,"record_offset":2412,"rva":14592624,"va":5383301744}
// 编译实例 RVAs: 0xdeaa70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::intersects_non_inclusive
// 记录形式: compiled_procedure; instances=1
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":3756,"rva":14592976,"va":5383302096}
// 编译实例 RVAs: 0xdeabd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::intersect_ray
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 148, 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFBF
// 首条位置: {"module":762,"inlinee":49087}
// 编译实例 RVAs: 0xdeaad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3140,"parent_record_offset":2756,"ancestor_record_offset":2756,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3168,"parent_record_offset":2756,"ancestor_record_offset":2756,"inlinee":36888,"source_line":264,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3112,"parent_record_offset":2756,"ancestor_record_offset":2756,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3220,"parent_record_offset":2756,"ancestor_record_offset":2756,"inlinee":49081,"source_line":251,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":2956,"parent_record_offset":2756,"ancestor_record_offset":2756,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":2904,"parent_record_offset":2756,"ancestor_record_offset":2756,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2756,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2756,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2756,"inlinee":36890,"source_line":265,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::min_element::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":2756,"inlinee":49083,"source_line":252,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::overlaps_segment
// 记录形式: inlinee_declaration; instances=1
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFD1
// 首条位置: {"module":764,"inlinee":49105}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::max_extent
// 记录形式: compiled_procedure; instances=1
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":2528,"rva":14592688,"va":5383301808}
// 编译实例 RVAs: 0xdeaab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":2704,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":36888,"source_line":264,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":2656,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":2676,"parent_record_offset":2656,"ancestor_record_offset":2528,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":2724,"parent_record_offset":2704,"ancestor_record_offset":2528,"inlinee":36890,"source_line":265,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 174, 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFD6
// 首条位置: {"module":762,"record_offset":4200,"rva":14593104,"va":5383302224}
// 编译实例 RVAs: 0xdeac50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::height
// 记录形式: compiled_procedure; instances=1
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":4592,"rva":14593168,"va":5383302288}
// 编译实例 RVAs: 0xdeac90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::expanded
// 记录形式: compiled_procedure; instances=1
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":5052,"rva":14593232,"va":5383302352}
// 编译实例 RVAs: 0xdeacd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":5180,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":762,"record_offset":5228,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":5200,"parent_record_offset":5180,"ancestor_record_offset":5052,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":5052,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::impl$10::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":578,"record_offset":105844,"rva":12266160,"va":5380975280}
// 编译实例 RVAs: 0xbb2ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field2_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3845967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380975280,"raw_hex":"e84faf3a00","size":5,"soft_fail":false,"target_va":5384821360,"va":5380975388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::from_center_extent
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFC9
// 首条位置: {"module":762,"inlinee":49097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/bvec3.rs
//     evidence={"module":762,"record_offset":7756,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":33014,"source_line":73,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7700,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":49092,"source_line":426,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7784,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7836,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":7728,"parent_record_offset":7700,"ancestor_record_offset":7660,"inlinee":37230,"source_line":1922,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":7660,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7660,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::from_points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73D4
// 首条位置: {"module":512,"inlinee":29652}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":596,"record_offset":5760,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":596,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5712,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":5864,"parent_record_offset":5784,"ancestor_record_offset":5712,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":5712,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":596,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":5712,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::expand_to_contain_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":5924,"rva":14593424,"va":5383302544}
// 编译实例 RVAs: 0xdead90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6096,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":11811,"source_line":294,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6068,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":42255,"source_line":278,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::expand
// 记录形式: compiled_procedure; instances=1
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":8184,"rva":14594224,"va":5383303344}
// 编译实例 RVAs: 0xdeb0b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":8312,"parent_record_offset":8184,"ancestor_record_offset":8184,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":8412,"parent_record_offset":8184,"ancestor_record_offset":8184,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":8360,"parent_record_offset":8184,"ancestor_record_offset":8184,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":8332,"parent_record_offset":8312,"ancestor_record_offset":8184,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":8184,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":8384,"parent_record_offset":8360,"ancestor_record_offset":8184,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 272, 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFC7
// 首条位置: {"module":762,"inlinee":49095}
// 编译实例 RVAs: 0xdeb070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7480,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7428,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":7400,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":7400,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::extent
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFC8
// 首条位置: {"module":762,"inlinee":49096}
// 编译实例 RVAs: 0xdeb110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7556,"parent_record_offset":7536,"ancestor_record_offset":7536,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7576,"parent_record_offset":7556,"ancestor_record_offset":7536,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::scaled
// 记录形式: compiled_procedure; instances=1
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":8672,"rva":14594368,"va":5383303488}
// 编译实例 RVAs: 0xdeb140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":8792,"parent_record_offset":8672,"ancestor_record_offset":8672,"inlinee":11813,"source_line":1449,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":8672,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::scaled_relative_to_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":7248,"rva":14593968,"va":5383303088}
// 编译实例 RVAs: 0xdeafb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33318931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383303088,"raw_hex":"e81368fc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383303272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7608,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":11813,"source_line":1449,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb3::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":762,"record_offset":7400,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":49095,"source_line":272,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb3::extent
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":762,"record_offset":7536,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":49096,"source_line":276,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb3::from_center_extent
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":762,"record_offset":7660,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":49097,"source_line":233,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":7728,"parent_record_offset":7700,"ancestor_record_offset":7248,"inlinee":37230,"source_line":1922,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":7248,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":7248,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":7576,"parent_record_offset":7556,"ancestor_record_offset":7248,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/bvec3.rs
//     evidence={"module":762,"record_offset":7756,"parent_record_offset":7660,"ancestor_record_offset":7248,"inlinee":33014,"source_line":73,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7700,"parent_record_offset":7660,"ancestor_record_offset":7248,"inlinee":49092,"source_line":426,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7480,"parent_record_offset":7400,"ancestor_record_offset":7248,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7428,"parent_record_offset":7400,"ancestor_record_offset":7248,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7556,"parent_record_offset":7536,"ancestor_record_offset":7248,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::translated
// 记录形式: compiled_procedure; instances=1
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":5568,"rva":14593328,"va":5383302448}
// 编译实例 RVAs: 0xdead30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":5692,"parent_record_offset":5568,"ancestor_record_offset":5568,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5568,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::inclusive_contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":6128,"rva":14593504,"va":5383302624}
// 编译实例 RVAs: 0xdeade0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/bvec3.rs
//     evidence={"module":762,"record_offset":6416,"parent_record_offset":6128,"ancestor_record_offset":6128,"inlinee":33014,"source_line":73,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec3::impl$2::bitand
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/bvec3.rs
//     evidence={"module":762,"record_offset":6388,"parent_record_offset":6128,"ancestor_record_offset":6128,"inlinee":49094,"source_line":131,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6280,"parent_record_offset":6128,"ancestor_record_offset":6128,"inlinee":49092,"source_line":426,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6332,"parent_record_offset":6128,"ancestor_record_offset":6128,"inlinee":49090,"source_line":448,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":6128,"inlinee":37230,"source_line":1922,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":6360,"parent_record_offset":6332,"ancestor_record_offset":6128,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::intersect_segment
// 记录形式: compiled_procedure; instances=1
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":5800,"rva":14593376,"va":5383302496}
// 编译实例 RVAs: 0xdead60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::aabb::intersection_aabb_segment3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383302496,"raw_hex":"e814fbffff","size":5,"soft_fail":false,"target_va":5383301264,"va":5383302519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::classify_ray_intersection
// 记录形式: compiled_procedure; instances=1
// 源码行: 334
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":762,"record_offset":6448,"rva":14593632,"va":5383302752}
// 编译实例 RVAs: 0xdeae60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/bool/bvec3.rs
//     evidence={"module":762,"record_offset":7128,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":33014,"source_line":73,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7156,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":49086,"source_line":437,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7068,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":33012,"source_line":459,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6780,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":11811,"source_line":294,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6956,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":14575,"source_line":340,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6816,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":42255,"source_line":278,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7012,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":28828,"source_line":327,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6700,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":49085,"source_line":1309,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6620,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":7184,"parent_record_offset":7156,"ancestor_record_offset":6448,"inlinee":31314,"source_line":1920,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":762,"record_offset":7096,"parent_record_offset":7068,"ancestor_record_offset":6448,"inlinee":8911,"source_line":1916,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":6448,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":762,"record_offset":6648,"parent_record_offset":6620,"ancestor_record_offset":6448,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":6980,"parent_record_offset":6956,"ancestor_record_offset":6448,"inlinee":14577,"source_line":341,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$0::min_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":762,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":6448,"inlinee":28830,"source_line":328,"archive":"_pdb/C/modules/00762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::RayBoxIntersection::is_hit
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 394, 395
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFC1
// 首条位置: {"module":762,"inlinee":49089}
// 编译实例 RVAs: 0xdea760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: aabb.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::Aabb2
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 18; 过程引用数: 57
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "constructor": "tuple$", "arguments": ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::window_cuts::WindowCutRecord"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "constructor": "tuple$", "arguments": ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::wall_holes::HoleType", "country_core::resources::walls::wall_holes::HoleOrigin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>,alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>", "alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>,alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>", "alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::ops::range::Range<usize>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$7"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "constructor": "core::slice::iter::Iter", "arguments": ["utils::geometry::aabb::Aabb2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<utils::geometry::aabb::Aabb2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >", "alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$13"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_circle_horizontal_wall_boards::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_circle_horizontal_wall_boards::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallspaceRegion,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_circle_horizontal_wall_boards::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallspaceRegion", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_circle_horizontal_wall_boards::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$10"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$10>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::ops::range::Range<usize>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$7"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >", "alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$13"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<usize,f32,f32>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<usize,f32,f32>", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$6> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_top_rim_holes::closure$0::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_top_rim_holes::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_top_rim_holes::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_top_rim_holes::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_wall_cutouts::closure$0::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_wall_cutouts::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_wall_cutouts::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::remapped_wall_cutouts::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >", "country_core::resources::walls::window_cuts::impl$0::records::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>,system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>", "system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>,system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>,system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$7>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >", "constructor": "system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["utils::geometry::aabb::Aabb2", "system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion", "alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,tuple$<>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["utils::geometry::aabb::Aabb2", "system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion", "tuple$<>", "system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<utils::geometry::aabb::Aabb2,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,tuple$<>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > > > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,tuple$<>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::vec::impl$21::extend_trusted::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::PlannedPlankRegion,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::construct_wooden_walls::construct_wall_boards::subdivide_wallspace_regions_into_planks::closure_env$0<system_wall_constructor::construct_wooden_walls::construct_wall_boards::plan_horizontal_wall_boards::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord>,country_core::resources::walls::window_cuts::impl$0::finalize_from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0> >", "constructor": "utils::geometry::aabb::Aabb2::from_points", "arguments": ["core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,array$<glam::f32::vec2::Vec2,2>,system_color::radial_menu::icon_bounds::closure_env$0> >", "constructor": "utils::geometry::aabb::Aabb2::from_points", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,array$<glam::f32::vec2::Vec2,2>,system_color::radial_menu::icon_bounds::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<system_color::radial_menu::RadialMenuRing>,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::show_color_options::closure_env$30> >", "constructor": "utils::geometry::aabb::Aabb2::from_points", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<system_color::radial_menu::RadialMenuRing>,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::show_color_options::closure_env$30>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::Aabb3
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 21; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "slotmap::basic::impl$13::next<country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle,utils::geometry::aabb::Aabb3>", "constructor": "slotmap::basic::impl$13::next", "arguments": ["country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle", "utils::geometry::aabb::Aabb3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::RayBoxIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: aabb.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: aabb.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::area
// 信息 ID: 0a92013ad5f6403c31452b6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::play_bulldoze_sound @ 0xb5d9eb -> 0xdeac10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::center
// 信息 ID: e27f44dc29e1acf22e4cec4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 105 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67ab12 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::perturb_row_ends @ 0x8412cb -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_horizontally @ 0x841513 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a5c6 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> @ 0x13f1a04 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> @ 0x13f1a15 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> @ 0x13f1b4f -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> @ 0x13f1b5f -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> @ 0x13f1bc7 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$13> @ 0x13f1a6e -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436b13 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436b36 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436b4e -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436c08 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436c44 -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436c5e -> 0xdeac60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=764, inline_record=576; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::contains
// 信息 ID: d979ac267fb3fcdc903b7d63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::is_point_still_valid @ 0xa5e4d3 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0xbbf3bd -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143be4c -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143bf7c -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143c05f -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143e6bd -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x1443993 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x1444887 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x14447ec -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x1478e9a -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x1478eb1 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x1478ece -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x1478ee4 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_auto_hide @ 0x296358b -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_auto_hide @ 0x2963b26 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::ui_slider::ui_slider @ 0x296c010 -> 0xdeaca0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::dims
// 信息 ID: bdfa3c51181202c51251c458
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a5d9 -> 0xdeac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a60c -> 0xdeac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a62e -> 0xdeac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x143907d -> 0xdeac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x143913e -> 0xdeac30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::expanded
// 信息 ID: 48209b4cb8c983f8b2a18e55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x6799c5 -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::split_brick_by_holes @ 0x810760 -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::window_cuts::WindowCutStorage::finalize @ 0xb3b80f -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x14493a1 -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144cb4d -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1450411 -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x1453b7d -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0> > @ 0x2b18aae -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0> > @ 0x2b18bef -> 0xdeacd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_center_dims
// 信息 ID: 6b6f3e48d00d44b7a9bb4dbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 47 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::split_horizontally @ 0x841533 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_horizontally @ 0x8415d2 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0xb3e23f -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0xb3eb65 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0xb3f0e4 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0xbe6b3f -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoleStorage::finalize @ 0xc4dea1 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoleStorage::finalize @ 0xc4e009 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a602 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cbdb6 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cc770 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436985 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436a3b -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436aee -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436c84 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436d70 -> 0xdeab10 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=2564; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=26084; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=27040; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=27840; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_center_radius
// 信息 ID: 9f70a830d30880810c88b78b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid::bushes_near_point<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0> @ 0x10d29be -> 0xdeab80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x21c8177 -> 0xdeab80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_points
// 信息 ID: 3f0723be9064d309cc8151a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::play_bulldoze_sound; module=570, inline_record=46136; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_window_cut_sdf::bake_layer; module=605, inline_record=7300; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1471, inline_record=992; 不是独立调用边。
// [PDB 内联宿主] system_color::set_halftimber_color_id::set_halftimber_color_id; module=2719, inline_record=1952; 不是独立调用边。
// [PDB 内联宿主] system_color::set_halftimber_color_id::set_wooden_wall_color_id; module=2719, inline_record=4472; 不是独立调用边。
// [PDB 内联宿主] system_color::set_halftimber_color_id::set_stone_foundation_color_id; module=2719, inline_record=6484; 不是独立调用边。
// [PDB 内联宿主] system_color::set_wall_color_id::set_wall_color_id; module=4017, inline_record=1556; 不是独立调用边。
// [PDB 内联宿主] system_color::set_gate_color_id::set_gate_color_id; module=4021, inline_record=1940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0> >
// 信息 ID: 5106318c53d8c104c0e381dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["system_color::radial_menu::ColorRadialMenu"]; 字段布局 = unknown。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] bevy_ecs::query::iter::QueryIter -> ["ref$<system_color::radial_menu::ColorRadialMenu>", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::flatten::FlatMap -> ["bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "array$<glam::f32::vec2::Vec2,2>", "system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0"]; 字段布局 = unknown。
// [类型关系] utils::geometry::aabb::Aabb2::from_points -> ["core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::publish_palette_bounds::closure$0::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::show_color_menu::publish_palette_bounds @ 0x28aa9e7 -> 0x2b188b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<system_color::radial_menu::RadialMenuRing>,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::show_color_options::closure_env$30> >
// 信息 ID: 4f2666b46cb0be785f261d0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["system_color::radial_menu::RadialMenuRing"]; 字段布局 = unknown。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::flatten::FlatMap -> ["core::slice::iter::Iter<system_color::radial_menu::RadialMenuRing>", "array$<glam::f32::vec2::Vec2,2>", "system_color::show_color_menu::show_color_options::closure_env$30"]; 字段布局 = unknown。
// [类型关系] utils::geometry::aabb::Aabb2::from_points -> ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<system_color::radial_menu::RadialMenuRing>,array$<glam::f32::vec2::Vec2,2>,system_color::show_color_menu::show_color_options::closure_env$30>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x289d908 -> 0x2b18f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_points<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,array$<glam::f32::vec2::Vec2,2>,system_color::radial_menu::icon_bounds::closure_env$0> >
// 信息 ID: 30270f801cabc8fe1081ec6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>"]; 字段布局 = unknown。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::flatten::FlatMap -> ["core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >", "array$<glam::f32::vec2::Vec2,2>", "system_color::radial_menu::icon_bounds::closure_env$0"]; 字段布局 = unknown。
// [类型关系] utils::geometry::aabb::Aabb2::from_points -> ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,array$<glam::f32::vec2::Vec2,2>,system_color::radial_menu::icon_bounds::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::radial_menu::RadialMenuRing::bounds @ 0x28b0ed3 -> 0x2b18d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::radial_menu::ColorRadialMenu::create @ 0x28b13ab -> 0x2b18d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::half_dims
// 信息 ID: ce2620c370da7198522cd417
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x146376d -> 0xdead00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::height
// 信息 ID: f6f56b59d92442db352e795a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 44 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67ab05 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_horizontally @ 0x8414b3 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_horizontally @ 0x8414c4 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x143776a -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x143780c -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14378d5 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1437acb -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1437906 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1439332 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1439405 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143bc40 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143bf2b -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143c00e -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143d000 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143eaad -> 0xdeac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143eb69 -> 0xdeac90 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersect_ray
// 信息 ID: a4b6cfeeb5c9def30bb80c88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xb3fb66 -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xb41dfa -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0xbe6b4c -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cbfd3 -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cc9aa -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x14882f4 -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x148b41d -> 0xdeaad0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=3824; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersects
// 信息 ID: cd72f4593373fbad4d064a45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x6799e8 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::split_brick_by_holes @ 0x81076b -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::split_brick_by_holes @ 0x810824 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0xb3e609 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0xb3ef68 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0xb3f498 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436e89 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1438fda -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143d2f3 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1442ede -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1442fb4 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1443066 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1445679 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x14493c4 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144cb6d -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1450434 -> 0xdeaa70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=27000; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=764, inline_record=27804; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersects_non_inclusive
// 信息 ID: 33b280434c3cbbe2e4719e46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::subtract_wall_cutouts @ 0x14917d7 -> 0xdeabd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::max_extent
// 信息 ID: 587a325bdd3b56708d57cd4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0x1474642 -> 0xdeaab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::overlaps_segment
// 信息 ID: f4eeb4ada4e1800d29d357bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::width
// 信息 ID: c9e81646fffb2bebe9b7a690
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 65 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67aaf8 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_horizontally @ 0x841507 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436c1c -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1437753 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14377fb -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1437983 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1437994 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1437aba -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436d0e -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436ed2 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436f7e -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1439027 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14390ee -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1439318 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_stone_foundation_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14393f4 -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143bebe -> 0xdeac50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=764, inline_record=392; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::center
// 信息 ID: 95f9e1a2b413e7d3b7d32c1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x90ef9b -> 0xdeb070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xc29e74 -> 0xdeb070 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<array$<f32,3> >,array$<f32,3>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<array$<f32,3>,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0> > > > > > @ 0x1829287 -> 0xdeb070 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adefa9 -> 0xdeb070 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::aabb::Aabb3::scaled_relative_to_center; module=762, inline_record=7400; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::classify_ray_intersection
// 信息 ID: 6ed832a1a36def5f7fca7e2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0xc70b18 -> 0xdeae60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x14d12d8 -> 0xdeae60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x2b18388 -> 0xdeae60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::expand
// 信息 ID: 845f9e64899fee0c6644075f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xbeca62 -> 0xdeb0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xbecaa0 -> 0xdeb0b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::expand_to_contain_point
// 信息 ID: c255b889439f1995a72b7fcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_walls::glade_startup_walls @ 0xba2e0e -> 0xdead90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::aabb_bounds @ 0xc29c50 -> 0xdead90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::extent
// 信息 ID: 0026099053f7d71d934505d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x90efa9 -> 0xdeb110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xc29e4e -> 0xdeb110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xc29e61 -> 0xdeb110 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<array$<f32,3> >,array$<f32,3>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<array$<f32,3>,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0> > > > > > @ 0x182929f -> 0xdeb110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adef41 -> 0xdeb110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adef75 -> 0xdeb110 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::aabb::Aabb3::scaled_relative_to_center; module=762, inline_record=7536; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::from_center_extent
// 信息 ID: 2e4e90402ef44c7411146b50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::aabb::Aabb3::scaled_relative_to_center; module=762, inline_record=7660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::from_points
// 信息 ID: 6f193c45ff61968e1072520d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::mesh::Mesh::aabb_bounds; module=596, inline_record=5712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::inclusive_contains_point
// 信息 ID: 1438e18109fd3b3997e14d9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xbecadb -> 0xdeade0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xbecb20 -> 0xdeade0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xbecb3c -> 0xdeade0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xbecb58 -> 0xdeade0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::intersect_segment
// 信息 ID: 8fc9b63b0b78cc6303c0ea36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::trigger_audio::trigger_audio_system @ 0xbb14ef -> 0xdead60 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::tree_animation::handplaced_tree_animation @ 0x286ea5a -> 0xdead60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::scaled
// 信息 ID: bc3d793376f1fba1dfad6506
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x90d0fb -> 0xdeb140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::update_prefabs @ 0x90e1a2 -> 0xdeb140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x90fcf1 -> 0xdeb140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa68d4a -> 0xdeb140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::scaled_relative_to_center
// 信息 ID: b565ddb4dc8cc9cf11a80d7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa68b9e -> 0xdeafb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::translated
// 信息 ID: 854cfeab37ebdd4f0f6f9bf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x90d127 -> 0xdead30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::update_prefabs @ 0x90e1ca -> 0xdead30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x90fd1c -> 0xdead30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::RayBoxIntersection::is_hit
// 信息 ID: 41c550c2651a7cc658708d7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xb3fb83 -> 0xdea760 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xb41e0e -> 0xdea760 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0xbe6b6a -> 0xdea760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cbff3 -> 0xdea760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cc9cb -> 0xdea760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1488315 -> 0xdea760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x148b442 -> 0xdea760 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=764, inline_record=4256; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::impl$10::fmt
// 信息 ID: b6a6f0d2c3dc7c99e7a21d5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::intersection_aabb_segment2
// 信息 ID: 3a0470542403141321f34bc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment @ 0xbc0aa5 -> 0xdea780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::intersection_aabb_segment3
// 信息 ID: 0fb78120ac35e8e1b0bffa1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::aabb::Aabb3::intersect_segment @ 0xdead77 -> 0xdea890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::is_inside_bbx
// 信息 ID: c03805d93fd63784c432f558
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0xb3e04a -> 0xdea730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0xb3e084 -> 0xdea730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0xb3e9ca -> 0xdea730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0xb3ea04 -> 0xdea730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_spawner::ivy_spawner @ 0xbc1ca2 -> 0xdea730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
