// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/geometry/aabb.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a5240108ca0552a23d00b5188658460ddaca9feb391c43cae7f112ac268e7db6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: aabb.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 493, 533, 537, 768, 1407, 1413, 1463, 1607, 1623, 3752, 3761, 3762
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
// 函数符号: utils::geometry::aabb::intersection_aabb_segment2
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1463,"record_offset":8376,"rva":19139376,"va":5387848496}
// 编译实例 RVAs: 0x1240b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9312,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":30660,"source_line":291,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9256,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":71152,"source_line":313,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9056,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9144,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":30569,"source_line":229,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":8952,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9200,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":45021,"source_line":220,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9368,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":8688,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":9424,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":8548,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":8376,"inlinee":27520,"source_line":1792,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":9280,"parent_record_offset":9256,"ancestor_record_offset":8376,"inlinee":10039,"source_line":1786,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":9084,"parent_record_offset":9056,"ancestor_record_offset":8376,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":8988,"parent_record_offset":8952,"ancestor_record_offset":8376,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":8376,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":8376,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":8716,"parent_record_offset":8688,"ancestor_record_offset":8376,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":8376,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::intersection_aabb_segment3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115ED
// 首条位置: {"module":1463,"inlinee":71149}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec3.rs
//     evidence={"module":1463,"record_offset":6716,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":30001,"source_line":72,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6744,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":71142,"source_line":341,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6660,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":71143,"source_line":363,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6424,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":13669,"source_line":243,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6548,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":9336,"source_line":279,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6236,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":71080,"source_line":230,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6604,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":26276,"source_line":270,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6828,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5872,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":71151,"source_line":1053,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6916,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5672,"parent_record_offset":5584,"ancestor_record_offset":5584,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":5584,"inlinee":27520,"source_line":1792,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":5584,"inlinee":10039,"source_line":1786,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":6460,"parent_record_offset":6424,"ancestor_record_offset":5584,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":6292,"parent_record_offset":6236,"ancestor_record_offset":5584,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5584,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":5584,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":5584,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5584,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":252,"rva":19136448,"va":5387845568}
// 编译实例 RVAs: 0x123ffc0
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
// IPI IDs: 0xAFD6
// 首条位置: {"module":768,"inlinee":45014}
// 编译实例 RVAs: 0x123fff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23696324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387845616,"raw_hex":"e8c4936901","size":5,"soft_fail":false,"target_va":5411542064,"va":5387845735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35372,"parent_record_offset":35344,"ancestor_record_offset":35344,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":36488,"parent_record_offset":36364,"ancestor_record_offset":36364,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35460,"parent_record_offset":35344,"ancestor_record_offset":35344,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":35484,"parent_record_offset":35460,"ancestor_record_offset":35344,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":36364,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":35396,"parent_record_offset":35372,"ancestor_record_offset":35344,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":768,"record_offset":35424,"parent_record_offset":35396,"ancestor_record_offset":35344,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00768.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":720,"rva":19136624,"va":5387845744}
// 编译实例 RVAs: 0x1240070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23696251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387845744,"raw_hex":"e87b936901","size":5,"soft_fail":false,"target_va":5411542064,"va":5387845808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":852,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":872,"parent_record_offset":852,"ancestor_record_offset":720,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::from_points
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76FE
// 首条位置: {"module":533,"inlinee":30462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":533,"record_offset":64760,"parent_record_offset":64716,"ancestor_record_offset":64716,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":533,"record_offset":64900,"parent_record_offset":64716,"ancestor_record_offset":64716,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":533,"record_offset":64964,"parent_record_offset":64716,"ancestor_record_offset":64716,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":533,"record_offset":64928,"parent_record_offset":64900,"ancestor_record_offset":64716,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":533,"record_offset":64988,"parent_record_offset":64964,"ancestor_record_offset":64716,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":533,"record_offset":64788,"parent_record_offset":64760,"ancestor_record_offset":64716,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1413,"record_offset":3096,"parent_record_offset":2980,"ancestor_record_offset":2900,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":533,"record_offset":64856,"parent_record_offset":64816,"ancestor_record_offset":64716,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":533,"record_offset":64816,"parent_record_offset":64788,"ancestor_record_offset":64716,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 101, 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFEC
// 首条位置: {"module":768,"inlinee":45036}
// 编译实例 RVAs: 0x12400c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48604,"parent_record_offset":48528,"ancestor_record_offset":48528,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":48552,"parent_record_offset":48528,"ancestor_record_offset":48528,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":48576,"parent_record_offset":48552,"ancestor_record_offset":48528,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":48628,"parent_record_offset":48604,"ancestor_record_offset":48528,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00768.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":1184,"rva":19136752,"va":5387845872}
// 编译实例 RVAs: 0x12400f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":1388,"parent_record_offset":1184,"ancestor_record_offset":1184,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":1312,"parent_record_offset":1184,"ancestor_record_offset":1184,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":1184,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":1184,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":1468,"rva":19136800,"va":5387845920}
// 编译实例 RVAs: 0x1240120
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
// 首条位置: {"module":1463,"record_offset":1580,"rva":19136832,"va":5387845952}
// 编译实例 RVAs: 0x1240140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1463,"record_offset":1704,"parent_record_offset":1580,"ancestor_record_offset":1580,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":1760,"parent_record_offset":1732,"ancestor_record_offset":1580,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":1780,"parent_record_offset":1760,"ancestor_record_offset":1580,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1463,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":1580,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::intersects
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFDA
// 首条位置: {"module":768,"inlinee":45018}
// 编译实例 RVAs: 0x1240170
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
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFDC
// 首条位置: {"module":768,"inlinee":45020}
// 编译实例 RVAs: 0x12401c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40908,"parent_record_offset":40552,"ancestor_record_offset":40552,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":41000,"parent_record_offset":40552,"ancestor_record_offset":40552,"inlinee":30569,"source_line":229,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40820,"parent_record_offset":40552,"ancestor_record_offset":40552,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":41056,"parent_record_offset":40552,"ancestor_record_offset":40552,"inlinee":45021,"source_line":220,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40652,"parent_record_offset":40552,"ancestor_record_offset":40552,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":768,"record_offset":40596,"parent_record_offset":40552,"ancestor_record_offset":40552,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":40940,"parent_record_offset":40908,"ancestor_record_offset":40552,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":768,"record_offset":40848,"parent_record_offset":40820,"ancestor_record_offset":40552,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":40676,"parent_record_offset":40652,"ancestor_record_offset":40552,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":768,"record_offset":40620,"parent_record_offset":40596,"ancestor_record_offset":40552,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00768.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::overlaps_segment
// 记录形式: inlinee_declaration; instances=1
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFDB
// 首条位置: {"module":768,"inlinee":45019}
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
// 首条位置: {"module":1463,"record_offset":2804,"rva":19137200,"va":5387846320}
// 编译实例 RVAs: 0x12402b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":3008,"parent_record_offset":2804,"ancestor_record_offset":2804,"inlinee":30569,"source_line":229,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":2932,"parent_record_offset":2804,"ancestor_record_offset":2804,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":2804,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2804,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb2::width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 174, 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFEA
// 首条位置: {"module":768,"inlinee":45034}
// 编译实例 RVAs: 0x12402e0
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
// 首条位置: {"module":1463,"record_offset":3180,"rva":19137264,"va":5387846384}
// 编译实例 RVAs: 0x12402f0
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
// 首条位置: {"module":1463,"record_offset":3292,"rva":19137280,"va":5387846400}
// 编译实例 RVAs: 0x1240300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":3420,"parent_record_offset":3292,"ancestor_record_offset":3292,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1463,"record_offset":3468,"parent_record_offset":3292,"ancestor_record_offset":3292,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":3440,"parent_record_offset":3420,"ancestor_record_offset":3292,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":3292,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::from_center_extent
// 记录形式: inlinee_declaration; instances=1
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115E5
// 首条位置: {"module":1463,"inlinee":71141}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::from_points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AD9
// 首条位置: {"module":493,"inlinee":27353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":537,"record_offset":73480,"parent_record_offset":73424,"ancestor_record_offset":73424,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":537,"record_offset":73508,"parent_record_offset":73480,"ancestor_record_offset":73424,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":537,"record_offset":73596,"parent_record_offset":73508,"ancestor_record_offset":73424,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":537,"record_offset":73564,"parent_record_offset":73536,"ancestor_record_offset":73424,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":537,"record_offset":73536,"parent_record_offset":73508,"ancestor_record_offset":73424,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00537.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":3524,"rva":19137328,"va":5387846448}
// 编译实例 RVAs: 0x1240330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":3752,"parent_record_offset":3524,"ancestor_record_offset":3524,"inlinee":13669,"source_line":243,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":3668,"parent_record_offset":3524,"ancestor_record_offset":3524,"inlinee":71080,"source_line":230,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":3524,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":3524,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":3844,"rva":19137472,"va":5387846592}
// 编译实例 RVAs: 0x12403c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":3972,"parent_record_offset":3844,"ancestor_record_offset":3844,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":4108,"parent_record_offset":3844,"ancestor_record_offset":3844,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":4048,"parent_record_offset":3844,"ancestor_record_offset":3844,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3844,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":3844,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":3844,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::center
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 272, 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115E3
// 首条位置: {"module":1463,"inlinee":71139}
// 编译实例 RVAs: 0x1240420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":4368,"parent_record_offset":4164,"ancestor_record_offset":4164,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":4288,"parent_record_offset":4164,"ancestor_record_offset":4164,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":4164,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":4164,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::Aabb3::extent
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115E4
// 首条位置: {"module":1463,"inlinee":71140}
// 编译实例 RVAs: 0x1240460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":4572,"parent_record_offset":4452,"ancestor_record_offset":4452,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":4452,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":4656,"rva":19137680,"va":5387846800}
// 编译实例 RVAs: 0x1240490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":4776,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":13671,"source_line":1193,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4656,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":4888,"rva":19137728,"va":5387846848}
// 编译实例 RVAs: 0x12404c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5012,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":4888,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":5124,"rva":19137776,"va":5387846896}
// 编译实例 RVAs: 0x12404f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec3.rs
//     evidence={"module":1463,"record_offset":5420,"parent_record_offset":5124,"ancestor_record_offset":5124,"inlinee":30001,"source_line":72,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec3::impl$2::bitand
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec3.rs
//     evidence={"module":1463,"record_offset":5392,"parent_record_offset":5124,"ancestor_record_offset":5124,"inlinee":71145,"source_line":130,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5276,"parent_record_offset":5124,"ancestor_record_offset":5124,"inlinee":71138,"source_line":330,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5336,"parent_record_offset":5124,"ancestor_record_offset":5124,"inlinee":71146,"source_line":352,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":5304,"parent_record_offset":5276,"ancestor_record_offset":5124,"inlinee":19940,"source_line":1790,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":5364,"parent_record_offset":5336,"ancestor_record_offset":5124,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":5452,"rva":19137904,"va":5387847024}
// 编译实例 RVAs: 0x1240570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::aabb::intersection_aabb_segment3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1463,"record_offset":5584,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":71149,"source_line":11,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":5452,"inlinee":27520,"source_line":1792,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":5452,"inlinee":10039,"source_line":1786,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":6460,"parent_record_offset":6424,"ancestor_record_offset":5452,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":6292,"parent_record_offset":6236,"ancestor_record_offset":5452,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5452,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":5452,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":5452,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5452,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec3.rs
//     evidence={"module":1463,"record_offset":6716,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":30001,"source_line":72,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6744,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":71142,"source_line":341,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6660,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":71143,"source_line":363,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6424,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":13669,"source_line":243,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6548,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":9336,"source_line":279,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6236,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":71080,"source_line":230,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6604,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":26276,"source_line":270,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6828,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5872,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":71151,"source_line":1053,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":6916,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":5672,"parent_record_offset":5584,"ancestor_record_offset":5452,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01463.jsonl.gz"}
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
// 首条位置: {"module":1463,"record_offset":7012,"rva":19138656,"va":5387847776}
// 编译实例 RVAs: 0x1240860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::bvec3::BVec3::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec3.rs
//     evidence={"module":1463,"record_offset":8132,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":30001,"source_line":72,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmpgt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":8160,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":71142,"source_line":341,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cmplt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":8076,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":71143,"source_line":363,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":7840,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":13669,"source_line":243,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":7964,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":9336,"source_line":279,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":7652,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":71080,"source_line":230,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::min_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":8020,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":26276,"source_line":270,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":7296,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":71151,"source_line":1053,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1463,"record_offset":7184,"parent_record_offset":7012,"ancestor_record_offset":7012,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":7012,"inlinee":27520,"source_line":1792,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1463,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7012,"inlinee":10039,"source_line":1786,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":7876,"parent_record_offset":7840,"ancestor_record_offset":7012,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1463,"record_offset":7708,"parent_record_offset":7652,"ancestor_record_offset":7012,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":7324,"parent_record_offset":7296,"ancestor_record_offset":7012,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1463,"record_offset":7212,"parent_record_offset":7184,"ancestor_record_offset":7012,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::aabb::RayBoxIntersection::is_hit
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 394, 395
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAFDF
// 首条位置: {"module":768,"inlinee":45023}
// 编译实例 RVAs: 0x1240b10
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 18; 过程引用数: 39
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "constructor": "tuple$", "arguments": ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::wall_holes::HoleType", "country_core::resources::walls::wall_holes::HoleOrigin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>,alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>", "alloc::vec::Vec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::ops::range::Range<usize>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["core::ops::range::Range<usize>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::geometry::aabb::Aabb2", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "constructor": "core::slice::iter::Iter", "arguments": ["utils::geometry::aabb::Aabb2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "core::slice::iter::Iter<utils::geometry::aabb::Aabb2>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$8", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >", "constructor": "system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["utils::geometry::aabb::Aabb2", "system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>", "alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::geometry::aabb::Aabb2>", "constructor": "ref$", "arguments": ["utils::geometry::aabb::Aabb2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,ref$<utils::geometry::aabb::Aabb2> >", "constructor": "tuple$", "arguments": ["usize", "ref$<utils::geometry::aabb::Aabb2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >", "constructor": "tuple$", "arguments": ["tuple$<usize,ref$<utils::geometry::aabb::Aabb2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$11"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>,country_core::resources::walls::wall_holes::impl$1::finalize_from::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["utils::geometry::aabb::Aabb2", "alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::aabb::Aabb3
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 21; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "slotmap::basic::impl$9::next<country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle,utils::geometry::aabb::Aabb3>", "constructor": "slotmap::basic::impl$9::next", "arguments": ["country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle", "utils::geometry::aabb::Aabb3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [已有模块证据关联；不自动成为字段事实] evidence/building-fences/B-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::aabb::Aabb2", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-gather/B-spatial-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::aabb::Aabb2", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-integrate/B-consumers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::aabb::Aabb2", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::geometry::aabb::Aabb2", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: aabb.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::area
// 信息 ID: 8e15de0626ace7a67820a745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze @ 0x9e12ca -> 0x1240120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::center
// 信息 ID: f112ecb89547a8486aa37cf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 85 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76c108 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216152 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216175 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121618c -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216248 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121627e -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216297 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12162d3 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12162ea -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216303 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121631a -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216b36 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216b46 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121633a -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121636a -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216383 -> 0x12400c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=768, inline_record=48528; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::contains
// 信息 ID: 9256dd1d15de31404a08073e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0x94b85d -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::is_point_still_valid @ 0x9992a6 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219efb -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a028 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a109 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121c687 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ec04 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x1222125 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x122206c -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x1241445 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x124145c -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x1241478 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap @ 0x124148f -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_auto_hide @ 0x234423b -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_auto_hide @ 0x2344ba9 -> 0x1240140 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::ui_slider::ui_slider @ 0x2349214 -> 0x1240140 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::expanded
// 信息 ID: bdb4ae46d31f12bfc59e1821
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76a2e6 -> 0x1240300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76bb31 -> 0x1240300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_center_dims
// 信息 ID: 6db7b63349635a30719ee84b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 41 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0x9ef2b6 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0x9efadb -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0x9f023b -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedc07b -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd360 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoleStorage::finalize @ 0xf8747c -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoleStorage::finalize @ 0xf875f8 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1215fc4 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216081 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121612d -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12162c4 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12163b0 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12164a9 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216563 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216609 -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219ffe -> 0x123fff0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=35344; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=36364; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=37340; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=768, inline_record=39192; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_center_radius
// 信息 ID: 19799c3d5912a481e36fb6a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0xeebbc7 -> 0x1240070 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d9b1d -> 0x1240070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::from_points
// 信息 ID: bf8a51e8a2b128c196c1129b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=533, inline_record=64716; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1413, inline_record=2900; 不是独立调用边。
// [PDB 内联宿主] system_color::set_wall_color_id::set_wall_color_id; module=3752, inline_record=3744; 不是独立调用边。
// [PDB 内联宿主] system_color::set_halftimber_color_id::set_halftimber_color_id; module=3761, inline_record=3684; 不是独立调用边。
// [PDB 内联宿主] system_color::set_gate_color_id::set_gate_color_id; module=3762, inline_record=3612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::half_dims
// 信息 ID: c8a299e951b30caa7777711a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1226ceb -> 0x12400f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::height
// 信息 ID: f13326915581a13b469a6add
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76c0fc -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216b1d -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216ba9 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216e8e -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216c8f -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216cc0 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219ca4 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219fd9 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a0ba -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ad53 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cab7 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cb70 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1221b4d -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1221c07 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0x1222043 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0x1222d75 -> 0x12402f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersect_ray
// 信息 ID: 2c2333e2dcf5508b82d58d75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xad67de -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad84c7 -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedca3a -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd963 -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251ccf -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254ecf -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x175aee9 -> 0x12401c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=768, inline_record=40552; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::intersects
// 信息 ID: f80500b18b53a060ae9be154
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76a309 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76bb44 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76bc20 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0> @ 0x9ef685 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0x9efea5 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0x9f05da -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12164bf -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121b05a -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1221aa1 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1221b78 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0x1221c32 -> 0x1240170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0x1222dba -> 0x1240170 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=36328; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=768, inline_record=37300; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::max_extent
// 信息 ID: 6bccf3c54777302ea97dc2f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0x123909f -> 0x12402b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::overlaps_segment
// 信息 ID: 77cd760a877fc1e0cd6b44ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb2::width
// 信息 ID: 451304780885f907e18628ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76c0f0 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121625b -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216b04 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216b99 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216d55 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216d77 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216e7e -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121634d -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216505 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x12165ad -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219f6e -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219fc6 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a05c -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a0a7 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ac26 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ca69 -> 0x12402e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=768, inline_record=48344; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::center
// 信息 ID: 382ff2fd35cec6471e417db9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x841887 -> 0x1240420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xa07ac4 -> 0x1240420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::classify_ray_intersection
// 信息 ID: b83d18b3673ec3d083d49522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x9e9ad8 -> 0x1240860 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x12774b9 -> 0x1240860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::expand
// 信息 ID: c4f088d870e2216d8f058208
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b138 -> 0x12403c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b16c -> 0x12403c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::expand_to_contain_point
// 信息 ID: 362cabf5018029970bca66de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::mesh::Mesh::aabb_bounds @ 0xa079e0 -> 0x1240330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::extent
// 信息 ID: ee4c1512c4a0380a3712d15d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x841894 -> 0x1240460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xa07a9e -> 0x1240460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xa07ab1 -> 0x1240460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::from_center_extent
// 信息 ID: 7ded30d76f72f3dc14c4eac5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::from_points
// 信息 ID: 2416931880bf22309d2a3fa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::mesh::Mesh::aabb_bounds; module=537, inline_record=73424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::inclusive_contains_point
// 信息 ID: 1ee6a2da20c2017d4b429196
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b1a4 -> 0x12404f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b1e6 -> 0x12404f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b201 -> 0x12404f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh @ 0xa9b21c -> 0x12404f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::intersect_segment
// 信息 ID: b319bec348311c4406ee7624
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::trigger_audio::trigger_audio_system @ 0x9eb49e -> 0x1240570 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::tree_animation::handplaced_tree_animation @ 0x2279a18 -> 0x1240570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::scaled
// 信息 ID: 8f7b73018a9babacc8ad92dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x840dbb -> 0x1240490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::update_prefabs @ 0x8427fc -> 0x1240490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x84427a -> 0x1240490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs_impl @ 0x12a4925 -> 0x1240490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs_impl @ 0x12a4aed -> 0x1240490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::Aabb3::translated
// 信息 ID: 7e5585677763b6eca7b5dd6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x840de6 -> 0x12404c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::update_prefabs @ 0x84282b -> 0x12404c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x8442ab -> 0x12404c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::RayBoxIntersection::is_hit
// 信息 ID: 10aa901040bf4a885b59067d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xad67fa -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad84da -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0xedca4e -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0xedd977 -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251ceb -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254ef3 -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x175af07 -> 0x1240b10 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects; module=768, inline_record=41116; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::intersection_aabb_segment2
// 信息 ID: b7af80d9da1b1f2578e112da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment @ 0x94cdb5 -> 0x1240b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::intersection_aabb_segment3
// 信息 ID: 4d1f295a64b550bd71bc1711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::aabb::Aabb3::intersect_segment; module=1463, inline_record=5584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::aabb::is_inside_bbx
// 信息 ID: 91209adf1e494acf9c41856d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_spawner::ivy_spawner @ 0x94de73 -> 0x123ffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0x9ef90a -> 0x123ffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0> @ 0x9ef944 -> 0x123ffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0x9f001a -> 0x123ffc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0> @ 0x9f0054 -> 0x123ffc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
