// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/utils/visual_entity_builder.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6196bd6d7c0f8c806d35c3ff2287ab640978b77d1c9f04e8ce5242a10322294a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: visual_entity_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1274, 1287, 2747, 2756, 3835, 3842, 3850, 3854, 3856, 3858, 3859, 3860, 3862, 3863, 3864, 3865, 3866, 3870
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15EFC
// 首条位置: {"module":2756,"inlinee":89852}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2756,"record_offset":10508,"parent_record_offset":10436,"ancestor_record_offset":10436,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2756,"record_offset":10768,"parent_record_offset":10740,"ancestor_record_offset":10436,"inlinee":89858,"source_line":110,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2756,"record_offset":10636,"parent_record_offset":10608,"ancestor_record_offset":10436,"inlinee":89854,"source_line":117,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2756,"record_offset":10700,"parent_record_offset":10672,"ancestor_record_offset":10436,"inlinee":89856,"source_line":128,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$68::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":2756,"record_offset":10836,"parent_record_offset":10804,"ancestor_record_offset":10436,"inlinee":89860,"source_line":270,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B45C
// 首条位置: {"module":3835,"inlinee":111708}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3835,"record_offset":1068,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":968,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":1032,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3835,"record_offset":740,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":89735,"source_line":4,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":3835,"record_offset":1996,"parent_record_offset":1956,"ancestor_record_offset":644,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":3835,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":644,"inlinee":4537,"source_line":138,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$33::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3835,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":644,"inlinee":4541,"source_line":1471,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3835,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":644,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmpeq_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":644,"inlinee":4600,"source_line":478,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":996,"parent_record_offset":968,"ancestor_record_offset":644,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":644,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":644,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":644,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3835,"record_offset":1096,"parent_record_offset":1068,"ancestor_record_offset":644,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3835,"record_offset":2208,"parent_record_offset":2176,"ancestor_record_offset":644,"inlinee":111716,"source_line":110,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3835,"record_offset":1656,"parent_record_offset":1620,"ancestor_record_offset":644,"inlinee":111712,"source_line":117,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$30::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3835,"record_offset":1956,"parent_record_offset":1928,"ancestor_record_offset":644,"inlinee":111714,"source_line":128,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$71::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3835,"record_offset":2488,"parent_record_offset":2460,"ancestor_record_offset":644,"inlinee":111718,"source_line":270,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::texture_presets::impl$8::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/texture_presets.rs
//     evidence={"module":3835,"record_offset":1292,"parent_record_offset":1260,"ancestor_record_offset":644,"inlinee":111710,"source_line":13,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$12::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3835,"record_offset":808,"parent_record_offset":740,"ancestor_record_offset":644,"inlinee":59777,"source_line":897,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmpeq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3835,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":644,"inlinee":4598,"source_line":310,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$2::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3835,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":644,"inlinee":4596,"source_line":914,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3835,"record_offset":1736,"parent_record_offset":1656,"ancestor_record_offset":644,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":772,"parent_record_offset":740,"ancestor_record_offset":644,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
// 记录形式: inline_instance, inlinee_declaration; instances=57
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB09
// 首条位置: {"module":1274,"inlinee":60169}
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
// 记录形式: inline_instance, inlinee_declaration; instances=36
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB0A
// 首条位置: {"module":1274,"inlinee":60170}
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
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB0B
// 首条位置: {"module":1274,"inlinee":60171}
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
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15ED1
// 首条位置: {"module":3860,"inlinee":89809}
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
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15ED2
// 首条位置: {"module":3859,"inlinee":89810}
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
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15ED5
// 首条位置: {"module":3866,"inlinee":89813}
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
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15ED6
// 首条位置: {"module":3864,"inlinee":89814}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15ED7
// 首条位置: {"module":3859,"inlinee":89815}
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
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15ED8
// 首条位置: {"module":3859,"inlinee":89816}
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
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB0C
// 首条位置: {"module":1274,"inlinee":60172}
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
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3835,"record_offset":2748,"rva":29589808,"va":5398298928}
// 编译实例 RVAs: 0x1c38130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=5
//     target_aliases=1889 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29578764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e8f4a93cfe","size":5,"soft_fail":false,"target_va":5368720672,"va":5398299431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25151244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e8f43880fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398301079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c39f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e855120000","size":5,"soft_fail":false,"target_va":5398306688,"va":5398301990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5293429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e875c55000","size":5,"soft_fail":false,"target_va":5403593344,"va":5398299910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t4616735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e81f724600","size":5,"soft_fail":false,"target_va":5402919072,"va":5398302332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2275452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e87cb82200","size":5,"soft_fail":false,"target_va":5400574960,"va":5398299503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2273853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e83db22200","size":5,"soft_fail":false,"target_va":5400573936,"va":5398300078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-21053643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e835bfbefe","size":5,"soft_fail":false,"target_va":5377246528,"va":5398300166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4676282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e8ba5a4700","size":5,"soft_fail":false,"target_va":5402978912,"va":5398302625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4676498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e8925b4700","size":5,"soft_fail":false,"target_va":5402979152,"va":5398302649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23731855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e871e195fe","size":5,"soft_fail":false,"target_va":5374568096,"va":5398299946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22607552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e84009a7fe","size":5,"soft_fail":false,"target_va":5375692416,"va":5398299963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-21352996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398298928,"raw_hex":"e8dc2dbafe","size":5,"soft_fail":false,"target_va":5376946416,"va":5398299407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":4788,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3835,"record_offset":4136,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":4752,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":8944,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3835,"record_offset":4020,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3835,"record_offset":3112,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":3020,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":7876,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":3932,"parent_record_offset":2748,"ancestor_record_offset":2748,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":2748,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":8220,"parent_record_offset":8184,"ancestor_record_offset":2748,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":8352,"parent_record_offset":8328,"ancestor_record_offset":2748,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":8184,"parent_record_offset":8152,"ancestor_record_offset":2748,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":8328,"parent_record_offset":8104,"ancestor_record_offset":2748,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":8300,"parent_record_offset":8104,"ancestor_record_offset":2748,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":8152,"parent_record_offset":8104,"ancestor_record_offset":2748,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":8468,"parent_record_offset":8104,"ancestor_record_offset":2748,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":2748,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":4864,"parent_record_offset":4836,"ancestor_record_offset":2748,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":2748,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3835,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":2748,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3835,"record_offset":8104,"parent_record_offset":8060,"ancestor_record_offset":2748,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3835,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":2748,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3835,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2748,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":3636,"parent_record_offset":3220,"ancestor_record_offset":2748,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":3544,"parent_record_offset":3488,"ancestor_record_offset":2748,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":2748,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":3348,"parent_record_offset":3264,"ancestor_record_offset":2748,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":3408,"parent_record_offset":3220,"ancestor_record_offset":2748,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3835,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":2748,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3835,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":2748,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3835,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":2748,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":2748,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3835,"record_offset":8384,"parent_record_offset":8104,"ancestor_record_offset":2748,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3835,"record_offset":8408,"parent_record_offset":8384,"ancestor_record_offset":2748,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2748,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2748,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":3835,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":2748,"inlinee":21456,"source_line":40,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3835,"record_offset":3220,"parent_record_offset":3112,"ancestor_record_offset":2748,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3835,"record_offset":3664,"parent_record_offset":3220,"ancestor_record_offset":2748,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3835,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2748,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3835,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2748,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2748,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3835,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":2748,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3835,"record_offset":3264,"parent_record_offset":3220,"ancestor_record_offset":2748,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3835,"record_offset":3076,"parent_record_offset":3044,"ancestor_record_offset":2748,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3835,"record_offset":9144,"rva":29593536,"va":5398302656}
// 编译实例 RVAs: 0x1c38fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=5
//     target_aliases=1889 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29582492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8649b3cfe","size":5,"soft_fail":false,"target_va":5368720672,"va":5398303159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25154972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8642a80fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398304807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c3a1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e815060000","size":5,"soft_fail":false,"target_va":5398307280,"va":5398305718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: visual_entity_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5289701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8e5b65000","size":5,"soft_fail":false,"target_va":5403593344,"va":5398303638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t4613007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e88f634600","size":5,"soft_fail":false,"target_va":5402919072,"va":5398306060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2271724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8eca92200","size":5,"soft_fail":false,"target_va":5400574960,"va":5398303231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2270125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8ada32200","size":5,"soft_fail":false,"target_va":5400573936,"va":5398303806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-21057371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8a5b0befe","size":5,"soft_fail":false,"target_va":5377246528,"va":5398303894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4672554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e82a4c4700","size":5,"soft_fail":false,"target_va":5402978912,"va":5398306353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4672770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8024d4700","size":5,"soft_fail":false,"target_va":5402979152,"va":5398306377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23735583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8e1d295fe","size":5,"soft_fail":false,"target_va":5374568096,"va":5398303674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22611280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e8b0faa6fe","size":5,"soft_fail":false,"target_va":5375692416,"va":5398303691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-21356724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398302656,"raw_hex":"e84c1fbafe","size":5,"soft_fail":false,"target_va":5376946416,"va":5398303135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":11184,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3835,"record_offset":10532,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":11148,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":15340,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3835,"record_offset":10416,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3835,"record_offset":9508,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":9416,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":14272,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":10328,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":14644,"parent_record_offset":14616,"ancestor_record_offset":9144,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":14616,"parent_record_offset":14580,"ancestor_record_offset":9144,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":9144,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3835,"record_offset":14580,"parent_record_offset":14548,"ancestor_record_offset":9144,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":14724,"parent_record_offset":14500,"ancestor_record_offset":9144,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":14696,"parent_record_offset":14500,"ancestor_record_offset":9144,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":14548,"parent_record_offset":14500,"ancestor_record_offset":9144,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3835,"record_offset":14864,"parent_record_offset":14500,"ancestor_record_offset":9144,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":11232,"parent_record_offset":11208,"ancestor_record_offset":9144,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":11260,"parent_record_offset":11232,"ancestor_record_offset":9144,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3835,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":9144,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3835,"record_offset":11288,"parent_record_offset":11260,"ancestor_record_offset":9144,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3835,"record_offset":14500,"parent_record_offset":14456,"ancestor_record_offset":9144,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3835,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":9144,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3835,"record_offset":9532,"parent_record_offset":9508,"ancestor_record_offset":9144,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":10032,"parent_record_offset":9616,"ancestor_record_offset":9144,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":9940,"parent_record_offset":9884,"ancestor_record_offset":9144,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":9144,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":9744,"parent_record_offset":9660,"ancestor_record_offset":9144,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3835,"record_offset":9804,"parent_record_offset":9616,"ancestor_record_offset":9144,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3835,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":9144,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3835,"record_offset":14300,"parent_record_offset":14272,"ancestor_record_offset":9144,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3835,"record_offset":10356,"parent_record_offset":10328,"ancestor_record_offset":9144,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3835,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":9144,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3835,"record_offset":14780,"parent_record_offset":14500,"ancestor_record_offset":9144,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3835,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":9144,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":9144,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3835,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":9144,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":3835,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":9144,"inlinee":21456,"source_line":40,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3835,"record_offset":9616,"parent_record_offset":9508,"ancestor_record_offset":9144,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3835,"record_offset":10060,"parent_record_offset":9616,"ancestor_record_offset":9144,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3835,"record_offset":9580,"parent_record_offset":9556,"ancestor_record_offset":9144,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3835,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":9144,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3835,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":9144,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3835,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":9144,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3835,"record_offset":9660,"parent_record_offset":9616,"ancestor_record_offset":9144,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03835.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3835,"record_offset":9472,"parent_record_offset":9440,"ancestor_record_offset":9144,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03835.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 19; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$10::clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>", "constructor": "alloc::vec::impl$10::clone", "arguments": ["system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: visual_entity_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
// 信息 ID: d1d55b93798163f4449cbfc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=3859, inline_record=22808; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=17460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorPersistentVisualState>
// 信息 ID: b4b375b227d11e1fbf8fa052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build -> ["system_decorator::decorator_visual_state::DecoratorPersistentVisualState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::blueprint::instantiate_blueprints @ 0x17a40ea -> 0x1c38130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
// 信息 ID: 29d9f0874b311228fae88abd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build -> ["system_decorator::decorator_visual_state::DecoratorTransientVisualState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x1c6aca0 -> 0x1c38fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::clothline @ 0x1c73370 -> 0x1c38fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::clothline
// 信息 ID: b524dbb917cca563d7844b19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals; module=3866, inline_record=19396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
// 信息 ID: ba09d38105cff960d286d44d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=18784; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=13812; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=17676; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=21140; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=3862, inline_record=12512; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=3863, inline_record=24552; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=3863, inline_record=25568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
// 信息 ID: 9a51b24028708f8a253dd75b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=13784; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=17648; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::generate_lanterns; module=3860, inline_record=21112; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=3862, inline_record=12456; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=3863, inline_record=24496; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::add_lintels; module=3863, inline_record=25512; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals; module=3866, inline_record=19368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
// 信息 ID: 3f18a4533d466c4988aaa08e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=3859, inline_record=12228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
// 信息 ID: 30dd5db5e324b0713921cea8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=3859, inline_record=19456; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=3859, inline_record=24296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
// 信息 ID: df5d6755063b6a1f661af255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=18756; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=10452; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=13700; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14000; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14196; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14420; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14644; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=3850, inline_record=15416; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
// 信息 ID: 9d710074c9aba7f7a153a383
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=18976; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=13728; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14448; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=3856, inline_record=16224; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=3858, inline_record=14468; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=3863, inline_record=15512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
// 信息 ID: c74fc7d2bed080141f5b1f82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 42 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=18728; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=18948; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=10424; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=13672; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=13972; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14168; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14392; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=14616; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
// 信息 ID: 39c510076d1023f8220ce37f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_roof::generate_roof_chimneys; module=3864, inline_record=8228; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=15132; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=16764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::impl$1::clone
// 信息 ID: 606bd75313adb70158d1646c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>; module=2756, inline_record=10436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::visual_entity_builder::impl$3::eq
// 信息 ID: c715acf400334d5cf530aa49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder>; module=3835, inline_record=644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
