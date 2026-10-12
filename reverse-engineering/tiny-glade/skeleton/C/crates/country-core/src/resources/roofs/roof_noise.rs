// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/roofs/roof_noise.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 666a4fecc675aa72119135f67d55e80a956881f33a8bd102c9adb09b673b9b8a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_noise.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 584, 1664
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::apply_roof_noise
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":584,"record_offset":13440,"rva":12483264,"va":5381192384}
// 编译实例 RVAs: 0xbe7ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_noise::gpunoise::noise
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12419579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192384,"raw_hex":"e8fb81bd00","size":5,"soft_fail":false,"target_va":5393612048,"va":5381192464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":584,"record_offset":13648,"parent_record_offset":13440,"ancestor_record_offset":13440,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":584,"record_offset":13588,"parent_record_offset":13440,"ancestor_record_offset":13440,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":584,"record_offset":13616,"parent_record_offset":13588,"ancestor_record_offset":13440,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":584,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":13440,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::revert_roof_noise
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9160
// 首条位置: {"module":584,"inlinee":37216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":584,"record_offset":27720,"parent_record_offset":27688,"ancestor_record_offset":27688,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":584,"record_offset":27744,"parent_record_offset":27720,"ancestor_record_offset":27688,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141A2
// 首条位置: {"module":1664,"inlinee":82338}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1332,"parent_record_offset":1288,"ancestor_record_offset":1288,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141A3
// 首条位置: {"module":1664,"inlinee":82339}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1480,"parent_record_offset":1420,"ancestor_record_offset":1420,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1452,"parent_record_offset":1420,"ancestor_record_offset":1420,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141A1
// 首条位置: {"module":1664,"inlinee":82337}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":1288,"parent_record_offset":1244,"ancestor_record_offset":1244,"inlinee":82338,"source_line":24,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":1420,"parent_record_offset":1244,"ancestor_record_offset":1244,"inlinee":82339,"source_line":34,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1332,"parent_record_offset":1288,"ancestor_record_offset":1244,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1452,"parent_record_offset":1420,"ancestor_record_offset":1244,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141A6
// 首条位置: {"module":1664,"inlinee":82342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::noise
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1664,"record_offset":856,"rva":24902928,"va":5393612048}
// 编译实例 RVAs: 0x17bfd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: roof_noise.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23445704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393612048,"raw_hex":"e8c8c06501","size":5,"soft_fail":false,"target_va":5417057792,"va":5393612083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":1244,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":82337,"source_line":50,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":2512,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":82343,"source_line":79,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":2448,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":82342,"source_line":54,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::fract_gl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1664,"record_offset":1140,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":67229,"source_line":736,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1664,"record_offset":1836,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1664,"record_offset":1924,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":79708,"source_line":1359,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$62::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1664,"record_offset":2012,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":82341,"source_line":1629,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1664,"record_offset":1028,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1664,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":856,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1332,"parent_record_offset":1288,"ancestor_record_offset":856,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1664,"record_offset":1452,"parent_record_offset":1420,"ancestor_record_offset":856,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1664,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":856,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1664,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":856,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::gpunoise::hash1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":1288,"parent_record_offset":1244,"ancestor_record_offset":856,"inlinee":82338,"source_line":24,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":1664,"record_offset":1420,"parent_record_offset":1244,"ancestor_record_offset":856,"inlinee":82339,"source_line":34,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1664,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":856,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141A7
// 首条位置: {"module":1664,"inlinee":82343}
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
// 完整结构及原始类型记录: roof_noise.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_noise.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_noise.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::apply_roof_noise
// 信息 ID: 6f6159ec4467655a39bbf8a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2824916 -> 0xbe7ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x293f0b2 -> 0xbe7ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash1
// 信息 ID: 0dc892e9f4ad9561b4e8a379
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=1288; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=1592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash2
// 信息 ID: d142537ec2b70d1d6dd094ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=1244; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=1544; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2188; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
// 信息 ID: e3f1cb3fc5ca8b9d04b6261d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=1420; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=1724; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2216; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::lerp
// 信息 ID: 718e3fcd57b0715406eb7972
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2512; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::noise
// 信息 ID: 849b5d7a1fd3a4e8fba9a9dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_noise::apply_roof_noise @ 0xbe7b10 -> 0x17bfd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped @ 0xbe911e -> 0x17bfd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
// 信息 ID: 1116af4993a6a39e01bb3756
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2448; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=1664, inline_record=2480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::revert_roof_noise
// 信息 ID: 64330cf9c14422d057304da1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped; module=584, inline_record=27688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
