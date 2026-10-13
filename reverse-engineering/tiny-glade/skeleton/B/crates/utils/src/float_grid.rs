// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/float_grid.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 76daf6afbee3be9a397fa0e39e23158d98240d3343e35f62fdd99fd803a5f3e4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: float_grid.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 84, 535, 539, 1406, 1471, 1514, 1560, 2602, 2742, 2743, 3648, 3686, 3700, 3704, 3976, 4433, 4436, 4490, 4514
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":51308,"rva":2456480,"va":5371165600}
// 编译实例 RVAs: 0x257ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b54c0
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: float_grid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6674421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371165600,"raw_hex":"e8f5d76500","size":5,"soft_fail":false,"target_va":5377840320,"va":5371165894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10716773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371165600,"raw_hex":"e86586a300","size":5,"soft_fail":false,"target_va":5381883088,"va":5371166310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t10716690","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371165600,"raw_hex":"e91286a300","size":5,"soft_fail":false,"target_va":5381883088,"va":5371166393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":84,"record_offset":52148,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":84,"record_offset":52644,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":51996,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10961,"source_line":984,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":52388,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":51908,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":51656,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":52504,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":51744,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":51564,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":52056,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":51820,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":84,"record_offset":52028,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10963,"source_line":311,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":84,"record_offset":52588,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10969,"source_line":165,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":84,"record_offset":52260,"parent_record_offset":51308,"ancestor_record_offset":51308,"inlinee":10965,"source_line":12,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":84,"record_offset":52612,"parent_record_offset":52588,"ancestor_record_offset":51308,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":84,"record_offset":51936,"parent_record_offset":51908,"ancestor_record_offset":51308,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":84,"record_offset":51684,"parent_record_offset":51656,"ancestor_record_offset":51308,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":51628,"parent_record_offset":51564,"ancestor_record_offset":51308,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":51768,"parent_record_offset":51744,"ancestor_record_offset":51308,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":51604,"parent_record_offset":51564,"ancestor_record_offset":51308,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":84,"record_offset":51848,"parent_record_offset":51820,"ancestor_record_offset":51308,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":84,"record_offset":52412,"parent_record_offset":52388,"ancestor_record_offset":51308,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$120::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":84,"record_offset":52668,"parent_record_offset":52644,"ancestor_record_offset":51308,"inlinee":10971,"source_line":1574,"archive":"_pdb/B/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
// 记录形式: compiled_procedure; instances=8
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":6848,"rva":10571248,"va":5379280368}
// 编译实例 RVAs: 0xa14df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2601952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379280368,"raw_hex":"e8e0b32700","size":5,"soft_fail":false,"target_va":5381883088,"va":5379281131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::lerp
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t2601885","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379280368,"raw_hex":"e99db32700","size":5,"soft_fail":false,"target_va":5381883088,"va":5379281198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":7700,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":539,"record_offset":8344,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7552,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10961,"source_line":984,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":8088,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7464,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7220,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":8204,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7308,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7100,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7612,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":7384,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":539,"record_offset":7584,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10963,"source_line":311,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":539,"record_offset":8288,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10969,"source_line":165,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=64
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":539,"record_offset":7812,"parent_record_offset":6848,"ancestor_record_offset":6848,"inlinee":10965,"source_line":12,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":6848,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":6848,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":7248,"parent_record_offset":7220,"ancestor_record_offset":6848,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":7164,"parent_record_offset":7100,"ancestor_record_offset":6848,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":6848,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":7140,"parent_record_offset":7100,"ancestor_record_offset":6848,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":6848,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$4::load_index_unchecked
//     owner=private_source_ownership_candidate; count=72
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":539,"record_offset":7188,"parent_record_offset":7100,"ancestor_record_offset":6848,"inlinee":30964,"source_line":583,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":6848,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$120::from
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":539,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":6848,"inlinee":10971,"source_line":1574,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4514,"record_offset":264,"rva":39973056,"va":5408682176}
// 编译实例 RVAs: 0x261f0c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ebe80
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: float_grid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29569902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408682176,"raw_hex":"e892cc3cfe","size":5,"soft_fail":false,"target_va":5379112576,"va":5408682473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26799804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408682176,"raw_hex":"e8441167fe","size":5,"soft_fail":false,"target_va":5381883088,"va":5408682887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-26799887","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5408682176,"raw_hex":"e9f11067fe","size":5,"soft_fail":false,"target_va":5381883088,"va":5408682970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4514,"record_offset":1108,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":4514,"record_offset":1604,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":956,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10961,"source_line":984,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":1348,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":868,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":616,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":1464,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":704,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":524,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":1016,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":780,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":4514,"record_offset":988,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10963,"source_line":311,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":4514,"record_offset":1548,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10969,"source_line":165,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":4514,"record_offset":1220,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10965,"source_line":12,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4514,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":264,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4514,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4514,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":264,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4514,"record_offset":588,"parent_record_offset":524,"ancestor_record_offset":264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4514,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4514,"record_offset":564,"parent_record_offset":524,"ancestor_record_offset":264,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":4514,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":264,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":4514,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$120::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":4514,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":264,"inlinee":10971,"source_line":1574,"archive":"_pdb/B/modules/04514.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::FloatGrid::load_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=179
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AD5
// 首条位置: {"module":84,"inlinee":10965}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::terrain_height::impl$7::load_index_unchecked
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":535,"record_offset":3116,"parent_record_offset":3080,"ancestor_record_offset":3080,"inlinee":27262,"source_line":347,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$4::load_index_unchecked
//     owner=private_source_ownership_candidate; count=64
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":539,"record_offset":7844,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":30964,"source_line":583,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":3348,"parent_record_offset":3236,"ancestor_record_offset":3080,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":3292,"parent_record_offset":3236,"ancestor_record_offset":3080,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":3264,"parent_record_offset":3236,"ancestor_record_offset":3080,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":535,"record_offset":3152,"parent_record_offset":3116,"ancestor_record_offset":3080,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":535,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":3080,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":535,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":3080,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":535,"record_offset":3236,"parent_record_offset":3116,"ancestor_record_offset":3080,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::impl$0::bilinear_sample_at_world_xz
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AE0
// 首条位置: {"module":535,"inlinee":10976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":2968,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":535,"record_offset":4372,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2816,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10961,"source_line":984,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":4116,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2728,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2488,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":4232,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2576,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2408,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2876,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":2652,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":535,"record_offset":2848,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10963,"source_line":311,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":535,"record_offset":4316,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10969,"source_line":165,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=72
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":535,"record_offset":3080,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":10965,"source_line":12,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":535,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":2264,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":2756,"parent_record_offset":2728,"ancestor_record_offset":2264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":535,"record_offset":2516,"parent_record_offset":2488,"ancestor_record_offset":2264,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":2460,"parent_record_offset":2408,"ancestor_record_offset":2264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":2264,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":2264,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":535,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":2264,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":535,"record_offset":3152,"parent_record_offset":3116,"ancestor_record_offset":2264,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":535,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":535,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2264,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$7::load_index_unchecked
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":535,"record_offset":3116,"parent_record_offset":3080,"ancestor_record_offset":2264,"inlinee":27262,"source_line":347,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":535,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":2264,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$120::from
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/u32/uvec2.rs
//     evidence={"module":535,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":2264,"inlinee":10971,"source_line":1574,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":535,"record_offset":3236,"parent_record_offset":3116,"ancestor_record_offset":2264,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00535.jsonl.gz"}
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
// 完整结构及原始类型记录: float_grid.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::float_grid::FloatGrid
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: float_grid.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/three-version-path-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/utils/src/float_grid.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: float_grid.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: utils::float_grid::FloatGrid::load_unchecked
// 信息 ID: a451dd4316349d5a2e079f33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 160 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52260; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52292; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52324; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52356; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52816; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52852; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52884; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=84, inline_record=52916; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz
// 信息 ID: f99e764e3b45c780ab9f3864
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=535, inline_record=2264; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=1471, inline_record=584; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=2602, inline_record=572; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=2743, inline_record=576; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=3686, inline_record=580; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=3704, inline_record=564; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=3976, inline_record=568; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=4436, inline_record=572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
// 信息 ID: 40b295ca3d6922418fab1ef0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] utils::float_grid::impl$0::bilinear_sample_at_world_xz -> ["country_core::resources::terrain_height::TerrainHeightFloatGrid"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 44 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xa4a1 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1988d -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::VisualTerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::VisualTerrainHeightSsbo> @ 0x257989 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$1<false> > > @ 0x2bb004 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> @ 0x498c12 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> @ 0x498ca7 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x49efb5 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x49f04c -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x7a0c40 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79fdfb -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::find_stair_placement_candidate @ 0x7a8032 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::construct_stair_steps @ 0x7da55d -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f20a3 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f221c -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f2300 -> 0x257ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f234f -> 0x257ba0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
// 信息 ID: aae3b16d271274ce9a2baa15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::RasterBufferRef -> ["512"]; 字段布局 = unknown。
// [类型关系] utils::float_grid::impl$0::bilinear_sample_at_world_xz -> ["country_core::resources::world_raster::RasterBufferRef<512>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 109 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cd71 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cd99 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cdc1 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cde8 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cf27 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cf47 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cf6b -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cf8b -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cfc7 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74cfe7 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74d006 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74d025 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74d9ca -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74d9f4 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74da22 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x74da50 -> 0xa14df0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// 信息 ID: 31862d2c266bf65db667afcf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::float_grid::impl$0::bilinear_sample_at_world_xz -> ["country_core::systems::audio::water_ambience_audio::WaterAudioGrid"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x2313459 -> 0x261f0c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x23134d0 -> 0x261f0c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
