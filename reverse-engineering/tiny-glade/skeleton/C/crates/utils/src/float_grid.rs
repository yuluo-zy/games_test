// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/float_grid.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 76daf6afbee3be9a397fa0e39e23158d98240d3343e35f62fdd99fd803a5f3e4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: float_grid.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 46, 605, 609, 1470, 1481, 1572, 1622, 2745, 2862, 2863, 3882, 3918, 3929, 3932, 4262, 4742, 4744, 4794, 4810, 4834
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":46,"record_offset":3692,"rva":1749904,"va":5370459024}
// 编译实例 RVAs: 0x1ab390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409d88c0
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: float_grid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8573906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370459024,"raw_hex":"e8d2d38200","size":5,"soft_fail":false,"target_va":5379033280,"va":5370459369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12821922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370459024,"raw_hex":"e8a2a5c300","size":5,"soft_fail":false,"target_va":5383281568,"va":5370459641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t12821839","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370459024,"raw_hex":"e94fa5c300","size":5,"soft_fail":false,"target_va":5383281568,"va":5370459724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":46,"record_offset":4428,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4280,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4204,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4164,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4032,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4060,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":3940,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4336,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4112,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":46,"record_offset":4308,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8906,"source_line":383,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":46,"record_offset":4696,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8909,"source_line":172,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":46,"record_offset":4540,"parent_record_offset":3692,"ancestor_record_offset":3692,"inlinee":8908,"source_line":12,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":46,"record_offset":4004,"parent_record_offset":3940,"ancestor_record_offset":3692,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":46,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3692,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":46,"record_offset":3980,"parent_record_offset":3940,"ancestor_record_offset":3692,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":46,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3692,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00046.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":46,"record_offset":4232,"parent_record_offset":4204,"ancestor_record_offset":3692,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00046.jsonl.gz"}
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
// 首条位置: {"module":609,"record_offset":75948,"rva":13158176,"va":5381867296}
// 编译实例 RVAs: 0xc8c720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1413842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381867296,"raw_hex":"e8d2921500","size":5,"soft_fail":false,"target_va":5383281568,"va":5381867721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::lerp
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t1413519","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381867296,"raw_hex":"e98f911500","size":5,"soft_fail":false,"target_va":5383281568,"va":5381868044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":609,"record_offset":76664,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76520,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76444,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76404,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76272,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76300,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76192,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76576,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76352,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":609,"record_offset":76548,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8906,"source_line":383,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":609,"record_offset":77084,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8909,"source_line":172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=64
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":609,"record_offset":76776,"parent_record_offset":75948,"ancestor_record_offset":75948,"inlinee":8908,"source_line":12,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":76244,"parent_record_offset":76192,"ancestor_record_offset":75948,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":76324,"parent_record_offset":76300,"ancestor_record_offset":75948,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":76220,"parent_record_offset":76192,"ancestor_record_offset":75948,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":76376,"parent_record_offset":76352,"ancestor_record_offset":75948,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$5::load_index_unchecked
//     owner=private_source_ownership_candidate; count=64
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":609,"record_offset":76808,"parent_record_offset":76776,"ancestor_record_offset":75948,"inlinee":38835,"source_line":788,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":76472,"parent_record_offset":76444,"ancestor_record_offset":75948,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00609.jsonl.gz"}
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
// 首条位置: {"module":4834,"record_offset":272,"rva":45252768,"va":5413961888}
// 编译实例 RVAs: 0x2b280a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a200
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: float_grid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32235521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413961888,"raw_hex":"e8ff1f14fe","size":5,"soft_fail":false,"target_va":5381726720,"va":5413962236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30680944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413961888,"raw_hex":"e890d82bfe","size":5,"soft_fail":false,"target_va":5383281568,"va":5413962507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-30681027","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5413961888,"raw_hex":"e93dd82bfe","size":5,"soft_fail":false,"target_va":5383281568,"va":5413962590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4834,"record_offset":1012,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":864,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":788,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":748,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":616,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":644,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":524,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":920,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":696,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":4834,"record_offset":892,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8906,"source_line":383,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":4834,"record_offset":1280,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8909,"source_line":172,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":4834,"record_offset":1124,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8908,"source_line":12,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4834,"record_offset":588,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4834,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4834,"record_offset":564,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4834,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4834,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":272,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/04834.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::FloatGrid::load_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=188
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x22CC
// 首条位置: {"module":46,"inlinee":8908}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::terrain_height::impl$7::load_index_unchecked
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":605,"record_offset":54884,"parent_record_offset":54852,"ancestor_record_offset":54852,"inlinee":30770,"source_line":352,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$5::load_index_unchecked
//     owner=private_source_ownership_candidate; count=64
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":609,"record_offset":76808,"parent_record_offset":76776,"ancestor_record_offset":76776,"inlinee":38835,"source_line":788,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":55088,"parent_record_offset":54976,"ancestor_record_offset":54852,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":55032,"parent_record_offset":54976,"ancestor_record_offset":54852,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":55004,"parent_record_offset":54976,"ancestor_record_offset":54852,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":605,"record_offset":54920,"parent_record_offset":54884,"ancestor_record_offset":54852,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":605,"record_offset":54944,"parent_record_offset":54920,"ancestor_record_offset":54852,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":605,"record_offset":54976,"parent_record_offset":54884,"ancestor_record_offset":54852,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::float_grid::impl$0::bilinear_sample_at_world_xz
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x230A
// 首条位置: {"module":605,"inlinee":8970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":605,"record_offset":54740,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54596,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54520,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54480,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54348,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54376,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54268,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54652,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54428,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":605,"record_offset":54624,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8906,"source_line":383,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::min
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/u32/uvec2.rs
//     evidence={"module":605,"record_offset":55944,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8909,"source_line":172,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=80
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":605,"record_offset":54852,"parent_record_offset":54132,"ancestor_record_offset":54132,"inlinee":8908,"source_line":12,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":54320,"parent_record_offset":54268,"ancestor_record_offset":54132,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":54400,"parent_record_offset":54376,"ancestor_record_offset":54132,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":54296,"parent_record_offset":54268,"ancestor_record_offset":54132,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":605,"record_offset":54452,"parent_record_offset":54428,"ancestor_record_offset":54132,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":605,"record_offset":54920,"parent_record_offset":54884,"ancestor_record_offset":54132,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":605,"record_offset":54944,"parent_record_offset":54920,"ancestor_record_offset":54132,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$7::load_index_unchecked
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":605,"record_offset":54884,"parent_record_offset":54852,"ancestor_record_offset":54132,"inlinee":30770,"source_line":352,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":605,"record_offset":54548,"parent_record_offset":54520,"ancestor_record_offset":54132,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":605,"record_offset":54976,"parent_record_offset":54884,"ancestor_record_offset":54132,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00605.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: utils::float_grid::FloatGrid::load_unchecked
// 信息 ID: 1011b34f69ce28e487634598
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 168 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4540; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4572; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4604; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4636; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4844; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4876; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4908; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>; module=46, inline_record=4940; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz
// 信息 ID: bad44a77c39c7a4b477498d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=605, inline_record=54132; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=1470, inline_record=592; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=2745, inline_record=580; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=2863, inline_record=584; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=3918, inline_record=588; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=3932, inline_record=572; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=4262, inline_record=576; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>; module=4744, inline_record=580; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
// 信息 ID: dad001bbead5a1d5d047e605
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] utils::float_grid::impl$0::bilinear_sample_at_world_xz -> ["country_core::resources::terrain_height::TerrainHeightFloatGrid"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 45 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::VisualTerrainHeightSsbo> | country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::VisualTerrainHeightSsbo> @ 0x1aaf59 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$1<false> > > @ 0x2ff3c0 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x3400df -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x34016f -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x344dff -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x344e8f -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x3694ab -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37b3dc -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85bdfb -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85c9e9 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::find_stair_placement_candidate @ 0x864056 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::construct_stone_stair_steps @ 0x8a0474 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::construct_wooden_stair_steps_impl @ 0x8ae272 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs @ 0x8c57d0 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs @ 0x8c594a -> 0x1ab390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs @ 0x8c5a18 -> 0x1ab390 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
// 信息 ID: 7ccf45da5d7cddc0fea183f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::RasterBufferRef -> ["512"]; 字段布局 = unknown。
// [类型关系] utils::float_grid::impl$0::bilinear_sample_at_world_xz -> ["country_core::resources::world_raster::RasterBufferRef<512>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 103 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f838e -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f83b7 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f83e0 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f8408 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f8547 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f8568 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f858c -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f85ad -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f85ea -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f860b -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f862b -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f864b -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f9587 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f95b2 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f95e0 -> 0xc8c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x7f960e -> 0xc8c720 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// 信息 ID: 49dd283b81601c49864ca26d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::float_grid::impl$0::bilinear_sample_at_world_xz -> ["country_core::systems::audio::water_ambience_audio::WaterAudioGrid"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x292f4e0 -> 0x2b280a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x292f555 -> 0x2b280a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
