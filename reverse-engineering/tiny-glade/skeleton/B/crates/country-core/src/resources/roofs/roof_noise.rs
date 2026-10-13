// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/roofs/roof_noise.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 666a4fecc675aa72119135f67d55e80a956881f33a8bd102c9adb09b673b9b8a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_noise.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 498, 539, 1451
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
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
// 首条位置: {"module":1451,"record_offset":5080,"rva":19089648,"va":5387798768}
// 编译实例 RVAs: 0x12348f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_noise::gpunoise::noise
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8517253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387798768,"raw_hex":"e87b097eff","size":5,"soft_fail":false,"target_va":5379281600,"va":5387798848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1451,"record_offset":5288,"parent_record_offset":5080,"ancestor_record_offset":5080,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1451,"record_offset":5228,"parent_record_offset":5080,"ancestor_record_offset":5080,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":5080,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":5080,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::revert_roof_noise
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D68
// 首条位置: {"module":498,"inlinee":28008}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":498,"record_offset":36428,"parent_record_offset":36396,"ancestor_record_offset":36396,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":498,"record_offset":36452,"parent_record_offset":36428,"ancestor_record_offset":36396,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F8
// 首条位置: {"module":539,"inlinee":30968}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10500,"parent_record_offset":10468,"ancestor_record_offset":10468,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F9
// 首条位置: {"module":539,"inlinee":30969}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10640,"parent_record_offset":10588,"ancestor_record_offset":10588,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":10588,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F7
// 首条位置: {"module":539,"inlinee":30967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":10444,"inlinee":30968,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":10588,"parent_record_offset":10444,"ancestor_record_offset":10444,"inlinee":30969,"source_line":34,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10500,"parent_record_offset":10468,"ancestor_record_offset":10444,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":10444,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78FE
// 首条位置: {"module":539,"inlinee":30974}
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
// 首条位置: {"module":539,"record_offset":10112,"rva":10572480,"va":5379281600}
// 编译实例 RVAs: 0xa152c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: roof_noise.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32566264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379281600,"raw_hex":"e8f8ebf001","size":5,"soft_fail":false,"target_va":5411847904,"va":5379281635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":10444,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":30967,"source_line":50,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":11684,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":30975,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":11620,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":30974,"source_line":54,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::fract_gl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":10340,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":30965,"source_line":656,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":11264,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":11352,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":30971,"source_line":1248,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$62::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":11440,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":30973,"source_line":1518,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":539,"record_offset":10284,"parent_record_offset":10112,"ancestor_record_offset":10112,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10500,"parent_record_offset":10468,"ancestor_record_offset":10112,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":10112,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":11292,"parent_record_offset":11264,"ancestor_record_offset":10112,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":10112,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::gpunoise::hash1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":10112,"inlinee":30968,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":539,"record_offset":10588,"parent_record_offset":10444,"ancestor_record_offset":10112,"inlinee":30969,"source_line":34,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":10112,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78FF
// 首条位置: {"module":539,"inlinee":30975}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::apply_roof_noise
// 信息 ID: 42d60d9201c795e52f739340
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x21a5a86 -> 0x12348f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231df9b -> 0x12348f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash1
// 信息 ID: 641ce32c9f34b75790fe3e33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10468; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=11028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash2
// 信息 ID: 7266dc303721c80e90e2d001
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10444; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10704; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10828; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
// 信息 ID: 888ea1178dd8b82a242dac52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10588; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10732; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=10868; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=11148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::lerp
// 信息 ID: 32846cdbff43cd20dacc342b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=11684; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=11712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::noise
// 信息 ID: b24896aae759e232e524bdff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::get_height_01_unclamped_from_ws @ 0x8e47c8 -> 0xa152c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_noise::apply_roof_noise @ 0x1234940 -> 0xa152c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
// 信息 ID: f763516d777169f988762f09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=11620; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=539, inline_record=11652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::revert_roof_noise
// 信息 ID: 87a78269f70fa519e90b32d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_height_01_unclamped_from_ws; module=498, inline_record=36396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
