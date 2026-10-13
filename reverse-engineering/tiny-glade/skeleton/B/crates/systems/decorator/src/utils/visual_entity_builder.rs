// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/utils/visual_entity_builder.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 137abdf9cc7421cdb58027905a0b2ecfcf698dffa45fdc15f39af8a5002e0f35
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: visual_entity_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 415, 429, 430, 431, 432, 433, 434, 438, 439, 440, 441, 442, 443, 444, 445, 450, 1487, 1503, 1557
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
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
// 函数符号: system_decorator::utils::visual_entity_builder::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x116EA
// 首条位置: {"module":1503,"inlinee":71402}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1503,"record_offset":9768,"parent_record_offset":9708,"ancestor_record_offset":9708,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1503,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":9708,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1503,"record_offset":10112,"parent_record_offset":10084,"ancestor_record_offset":9708,"inlinee":71408,"source_line":158,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$35::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1503,"record_offset":9884,"parent_record_offset":9856,"ancestor_record_offset":9708,"inlinee":71404,"source_line":166,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$39::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1503,"record_offset":10044,"parent_record_offset":10016,"ancestor_record_offset":9708,"inlinee":71406,"source_line":178,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$83::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1503,"record_offset":10176,"parent_record_offset":10148,"ancestor_record_offset":9708,"inlinee":71410,"source_line":351,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":9824,"parent_record_offset":9796,"ancestor_record_offset":9708,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118ED
// 首条位置: {"module":1557,"inlinee":71917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":1052,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":984,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":1016,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1557,"record_offset":756,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":25831,"source_line":4,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1557,"record_offset":1848,"parent_record_offset":1812,"ancestor_record_offset":652,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1557,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":652,"inlinee":10417,"source_line":137,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":2544,"parent_record_offset":2484,"ancestor_record_offset":652,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":652,"inlinee":10418,"source_line":1742,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":1108,"parent_record_offset":1080,"ancestor_record_offset":652,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":904,"parent_record_offset":872,"ancestor_record_offset":652,"inlinee":10466,"source_line":483,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":652,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":652,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":652,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1557,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":652,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":1557,"record_offset":2132,"parent_record_offset":2100,"ancestor_record_offset":652,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1557,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":652,"inlinee":71925,"source_line":158,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$37::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1557,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":652,"inlinee":71921,"source_line":166,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$41::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1557,"record_offset":1812,"parent_record_offset":1780,"ancestor_record_offset":652,"inlinee":71923,"source_line":178,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$73::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1557,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":652,"inlinee":71927,"source_line":329,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$86::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":1557,"record_offset":2256,"parent_record_offset":2224,"ancestor_record_offset":652,"inlinee":71929,"source_line":351,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::texture_presets::impl$8::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/texture_presets.rs
//     evidence={"module":1557,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":652,"inlinee":71919,"source_line":16,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$12::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1557,"record_offset":824,"parent_record_offset":756,"ancestor_record_offset":652,"inlinee":10916,"source_line":945,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1557,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":652,"inlinee":10464,"source_line":320,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1557,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":652,"inlinee":10462,"source_line":989,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1557,"record_offset":1616,"parent_record_offset":1536,"ancestor_record_offset":652,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":788,"parent_record_offset":756,"ancestor_record_offset":652,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
// 记录形式: inline_instance, inlinee_declaration; instances=61
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60E7
// 首条位置: {"module":415,"inlinee":24807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
// 记录形式: inline_instance, inlinee_declaration; instances=38
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60E8
// 首条位置: {"module":415,"inlinee":24808}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::has_birdspot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6312
// 首条位置: {"module":439,"inlinee":25362}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x620D
// 首条位置: {"module":429,"inlinee":25101}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::lantern_light
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60EB
// 首条位置: {"module":415,"inlinee":24811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60EA
// 首条位置: {"module":415,"inlinee":24810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60E9
// 首条位置: {"module":415,"inlinee":24809}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::clothline
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x631E
// 首条位置: {"module":440,"inlinee":25374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6221
// 首条位置: {"module":431,"inlinee":25121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6308
// 首条位置: {"module":438,"inlinee":25352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x637F
// 首条位置: {"module":450,"inlinee":25471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x620C
// 首条位置: {"module":429,"inlinee":25100}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1557,"record_offset":2592,"rva":19569456,"va":5388278576}
// 编译实例 RVAs: 0x12a9b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=12
//     target_aliases=532 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19325342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8621ed9fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5388279065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408119e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11113804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8b46a56ff","size":5,"soft_fail":false,"target_va":5377169888,"va":5388283687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14082bd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11002316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8341e58ff","size":5,"soft_fail":false,"target_va":5377277280,"va":5388279591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10972365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8339358ff","size":5,"soft_fail":false,"target_va":5377307216,"va":5388279576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23566572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8ec986701","size":5,"soft_fail":false,"target_va":5411847152,"va":5388280575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=19
//     evidence={"assembly":"\tcallq\t23056283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e89bcf5f01","size":5,"soft_fail":false,"target_va":5411340064,"va":5388283776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-245118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e88242fcff","size":5,"soft_fail":false,"target_va":5388034016,"va":5388279129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5855574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8aaa6a6ff","size":5,"soft_fail":false,"target_va":5382424224,"va":5388279793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23257308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8dce06201","size":5,"soft_fail":false,"target_va":5411541808,"va":5388284495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23257264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8b0e06201","size":5,"soft_fail":false,"target_va":5411541776,"va":5388284507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23257767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8a7e26201","size":5,"soft_fail":false,"target_va":5411542064,"va":5388284292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9948934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e8fa3068ff","size":5,"soft_fail":false,"target_va":5378331888,"va":5388280817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9906535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e899d668ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5388280834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6344558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388278576,"raw_hex":"e892309fff","size":5,"soft_fail":false,"target_va":5381934464,"va":5388279017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":4960,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":7584,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1557,"record_offset":4168,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":6676,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":6768,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":4924,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1557,"record_offset":3924,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1557,"record_offset":10640,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":25032,"source_line":166,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1557,"record_offset":2968,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":2876,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":12744,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":3836,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":2592,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":2592,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":2592,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":2592,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1557,"record_offset":5124,"parent_record_offset":5088,"ancestor_record_offset":2592,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":5056,"parent_record_offset":5024,"ancestor_record_offset":2592,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":5088,"parent_record_offset":5056,"ancestor_record_offset":2592,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":4992,"parent_record_offset":4960,"ancestor_record_offset":2592,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":2592,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":2592,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1557,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2592,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":3496,"parent_record_offset":3076,"ancestor_record_offset":2592,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":3404,"parent_record_offset":3348,"ancestor_record_offset":2592,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2592,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":3204,"parent_record_offset":3120,"ancestor_record_offset":2592,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":3268,"parent_record_offset":3076,"ancestor_record_offset":2592,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1557,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":2592,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":2592,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":2592,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":2592,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1557,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":2592,"inlinee":13110,"source_line":43,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1557,"record_offset":3076,"parent_record_offset":2968,"ancestor_record_offset":2592,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1557,"record_offset":3524,"parent_record_offset":3076,"ancestor_record_offset":2592,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1557,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":2592,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1557,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2592,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2592,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1557,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2592,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1557,"record_offset":3120,"parent_record_offset":3076,"ancestor_record_offset":2592,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1557,"record_offset":2932,"parent_record_offset":2900,"ancestor_record_offset":2592,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1557,"record_offset":13404,"rva":19575408,"va":5388284528}
// 编译实例 RVAs: 0x12ab270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=12
//     target_aliases=532 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19331294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e82207d9fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5388285017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140811c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11119164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e8c45556ff","size":5,"soft_fail":false,"target_va":5377170480,"va":5388289639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14082bd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11008268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e8f40658ff","size":5,"soft_fail":false,"target_va":5377277280,"va":5388285543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10978317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e8f37b58ff","size":5,"soft_fail":false,"target_va":5377307216,"va":5388285528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23560620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e8ac816701","size":5,"soft_fail":false,"target_va":5411847152,"va":5388286527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=19
//     evidence={"assembly":"\tcallq\t23050331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e85bb85f01","size":5,"soft_fail":false,"target_va":5411340064,"va":5388289728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-251070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e8422bfcff","size":5,"soft_fail":false,"target_va":5388034016,"va":5388285081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5861526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e86a8fa6ff","size":5,"soft_fail":false,"target_va":5382424224,"va":5388285745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23251356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e89cc96201","size":5,"soft_fail":false,"target_va":5411541808,"va":5388290447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23251312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e870c96201","size":5,"soft_fail":false,"target_va":5411541776,"va":5388290459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23251815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e867cb6201","size":5,"soft_fail":false,"target_va":5411542064,"va":5388290244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9954886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e8ba1968ff","size":5,"soft_fail":false,"target_va":5378331888,"va":5388286769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9912487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e859bf68ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5388286786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6350510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388284528,"raw_hex":"e852199fff","size":5,"soft_fail":false,"target_va":5381934464,"va":5388284969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":15772,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":18396,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1557,"record_offset":14980,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":17488,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":17580,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":15736,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1557,"record_offset":14736,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1557,"record_offset":21452,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":25032,"source_line":166,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1557,"record_offset":13780,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":13688,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":23556,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":14648,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":13404,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":13404,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":16024,"parent_record_offset":15992,"ancestor_record_offset":13404,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1557,"record_offset":15992,"parent_record_offset":15964,"ancestor_record_offset":13404,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1557,"record_offset":15936,"parent_record_offset":15900,"ancestor_record_offset":13404,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":15868,"parent_record_offset":15836,"ancestor_record_offset":13404,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":15900,"parent_record_offset":15868,"ancestor_record_offset":13404,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":15804,"parent_record_offset":15772,"ancestor_record_offset":13404,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1557,"record_offset":15836,"parent_record_offset":15804,"ancestor_record_offset":13404,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":14764,"parent_record_offset":14736,"ancestor_record_offset":13404,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1557,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":13404,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":14308,"parent_record_offset":13888,"ancestor_record_offset":13404,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":14216,"parent_record_offset":14160,"ancestor_record_offset":13404,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":13984,"parent_record_offset":13960,"ancestor_record_offset":13404,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":14016,"parent_record_offset":13932,"ancestor_record_offset":13404,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1557,"record_offset":14080,"parent_record_offset":13888,"ancestor_record_offset":13404,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1557,"record_offset":14676,"parent_record_offset":14648,"ancestor_record_offset":13404,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1557,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":13404,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":14792,"parent_record_offset":14764,"ancestor_record_offset":13404,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1557,"record_offset":14820,"parent_record_offset":14792,"ancestor_record_offset":13404,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1557,"record_offset":15004,"parent_record_offset":14980,"ancestor_record_offset":13404,"inlinee":13110,"source_line":43,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1557,"record_offset":13888,"parent_record_offset":13780,"ancestor_record_offset":13404,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1557,"record_offset":14336,"parent_record_offset":13888,"ancestor_record_offset":13404,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1557,"record_offset":13852,"parent_record_offset":13828,"ancestor_record_offset":13404,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1557,"record_offset":13828,"parent_record_offset":13804,"ancestor_record_offset":13404,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1557,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":13404,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1557,"record_offset":13960,"parent_record_offset":13932,"ancestor_record_offset":13404,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1557,"record_offset":13932,"parent_record_offset":13888,"ancestor_record_offset":13404,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1557,"record_offset":13744,"parent_record_offset":13712,"ancestor_record_offset":13404,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01557.jsonl.gz"}
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
// 完整结构及原始类型记录: visual_entity_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 21; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$11::clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>", "constructor": "alloc::vec::impl$11::clone", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>", "constructor": "core::slice::iter::Iter", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>", "system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<str$>", "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>,system_decorator::blueprint::instantiate_decorator_colliders::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::cmp::impl$4::equal<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>", "constructor": "core::slice::cmp::impl$4::equal", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>", "constructor": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build", "arguments": ["system_decorator::decorator_visual_state::DecoratorPersistentVisualState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>", "constructor": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build", "arguments": ["system_decorator::decorator_visual_state::DecoratorTransientVisualState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: visual_entity_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-holes/B-opening-producers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: visual_entity_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
// 信息 ID: 49d2b3441de20b58a6db27d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19716; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=450, inline_record=22984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>
// 信息 ID: b464bb1ab3b86efc963ccd25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build -> ["system_decorator::decorator_visual_state::DecoratorPersistentVisualState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::blueprint::instantiate_blueprints @ 0x828a30 -> 0x12a9b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
// 信息 ID: bbe650c1c7f9fb256a92195a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build -> ["system_decorator::decorator_visual_state::DecoratorTransientVisualState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::clothline @ 0x7e29e7 -> 0x12ab270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x7e7006 -> 0x12ab270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::clothline
// 信息 ID: 557b8a86d61404d2f87da5cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals; module=440, inline_record=17048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
// 信息 ID: 4b71983c9b8fadd8b48e46e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=15404; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag_connections::clothline; module=438, inline_record=29356; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=442, inline_record=26376; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=442, inline_record=26744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
// 信息 ID: 97f49f3e608ea94491d7189a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19964; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20608; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=21188; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=430, inline_record=12976; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=439, inline_record=10916; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals; module=440, inline_record=17020; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=442, inline_record=26320; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=442, inline_record=26688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
// 信息 ID: 29a963ff81572f062daa45a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag_connections::clothline; module=438, inline_record=29392; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=450, inline_record=21012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
// 信息 ID: 15c4e8cc5c7f2f504c827fb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=450, inline_record=24548; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=450, inline_record=25040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::has_birdspot
// 信息 ID: 6b0e4b71cb303bcc5ec5bbba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::decorator_clutter::generate_decorator_clutter; module=439, inline_record=10944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::lantern_light
// 信息 ID: 67849b51905c4adbb7e9eae2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19992; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20636; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=21216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
// 信息 ID: 03405d96da254bce19b355d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19468; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20360; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20940; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=10336; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=15100; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=15376; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=15388; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=433, inline_record=10264; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
// 信息 ID: 24515189c1e1fe3547bce171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=15128; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=433, inline_record=13432; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=20092; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=441, inline_record=12124; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=442, inline_record=14676; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=445, inline_record=13700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
// 信息 ID: cc3c9778d45105d66de4e47b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 45 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19440; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19688; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=19936; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20332; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20580; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=20912; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=415, inline_record=21160; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=429, inline_record=10308; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
// 信息 ID: fc043443964f26ba322ea0fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_roof::generate_roof_chimneys; module=431, inline_record=9680; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=16856; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=18600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::impl$1::clone
// 信息 ID: ec11c89a3ae4cff31915c285
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>; module=1503, inline_record=9708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::impl$3::eq
// 信息 ID: dd9e76193cb3bda7488c1dcc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>; module=1557, inline_record=652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
