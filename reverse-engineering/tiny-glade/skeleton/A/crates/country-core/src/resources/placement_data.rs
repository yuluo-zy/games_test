// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/placement_data.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3d6a9dbd5629007c4728d3d244311c4b387e88a40e98f0147fd4f9a1d7f46267
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: placement_data.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 401, 405, 424, 497, 1306, 1363, 1371, 1403
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6027
// 首条位置: {"module":1306,"inlinee":24615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":424,"record_offset":30668,"rva":5956304,"va":5374665424}
// 编译实例 RVAs: 0x5ae2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: placement_data.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28927835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665424,"raw_hex":"e85b67b901","size":5,"soft_fail":false,"target_va":5403593344,"va":5374665504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":424,"record_offset":30860,"parent_record_offset":30668,"ancestor_record_offset":30668,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":424,"record_offset":30888,"parent_record_offset":30860,"ancestor_record_offset":30668,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":424,"record_offset":30916,"parent_record_offset":30888,"ancestor_record_offset":30668,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":424,"record_offset":30944,"parent_record_offset":30916,"ancestor_record_offset":30668,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFDD
// 首条位置: {"module":1306,"inlinee":61405}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":294676,"parent_record_offset":294652,"ancestor_record_offset":294652,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":294812,"parent_record_offset":294700,"ancestor_record_offset":294652,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294728,"parent_record_offset":294700,"ancestor_record_offset":294652,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294700,"parent_record_offset":294676,"ancestor_record_offset":294652,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":294752,"parent_record_offset":294728,"ancestor_record_offset":294652,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":294776,"parent_record_offset":294752,"ancestor_record_offset":294652,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A4B
// 首条位置: {"module":424,"inlinee":23115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFDB
// 首条位置: {"module":1306,"inlinee":61403}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1306,"record_offset":294900,"parent_record_offset":293896,"ancestor_record_offset":293896,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":298532,"parent_record_offset":293896,"ancestor_record_offset":293896,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::default_anim_t
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1306,"record_offset":298788,"parent_record_offset":293896,"ancestor_record_offset":293896,"inlinee":23099,"source_line":41,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":298816,"parent_record_offset":293896,"ancestor_record_offset":293896,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":293952,"parent_record_offset":293896,"ancestor_record_offset":293896,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":294928,"parent_record_offset":293896,"ancestor_record_offset":293896,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":294812,"parent_record_offset":294700,"ancestor_record_offset":293896,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":294856,"parent_record_offset":294000,"ancestor_record_offset":293896,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::placement_data::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1306,"record_offset":294652,"parent_record_offset":294628,"ancestor_record_offset":293896,"inlinee":61405,"source_line":8,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":298888,"parent_record_offset":298864,"ancestor_record_offset":293896,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":298976,"parent_record_offset":298952,"ancestor_record_offset":293896,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":299200,"parent_record_offset":299176,"ancestor_record_offset":293896,"inlinee":25217,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":298864,"parent_record_offset":298840,"ancestor_record_offset":293896,"inlinee":6861,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":294628,"parent_record_offset":294000,"ancestor_record_offset":293896,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":299176,"parent_record_offset":299152,"ancestor_record_offset":293896,"inlinee":10265,"source_line":134,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":298952,"parent_record_offset":298928,"ancestor_record_offset":293896,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":298492,"parent_record_offset":298468,"ancestor_record_offset":293896,"inlinee":10271,"source_line":1338,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":295356,"parent_record_offset":295332,"ancestor_record_offset":293896,"inlinee":6860,"source_line":1338,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294236,"parent_record_offset":294084,"ancestor_record_offset":293896,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294988,"parent_record_offset":294956,"ancestor_record_offset":293896,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294084,"parent_record_offset":294000,"ancestor_record_offset":293896,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294124,"parent_record_offset":294084,"ancestor_record_offset":293896,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294000,"parent_record_offset":293952,"ancestor_record_offset":293896,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294956,"parent_record_offset":294928,"ancestor_record_offset":293896,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":294700,"parent_record_offset":294676,"ancestor_record_offset":293896,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":294676,"parent_record_offset":294652,"ancestor_record_offset":293896,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":294260,"parent_record_offset":294236,"ancestor_record_offset":293896,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":294156,"parent_record_offset":294124,"ancestor_record_offset":293896,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":294284,"parent_record_offset":294260,"ancestor_record_offset":293896,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":294188,"parent_record_offset":294156,"ancestor_record_offset":293896,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFDE
// 首条位置: {"module":1306,"inlinee":61406}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":299392,"parent_record_offset":299352,"ancestor_record_offset":299352,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::default_anim_t
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1306,"record_offset":299624,"parent_record_offset":299352,"ancestor_record_offset":299352,"inlinee":23099,"source_line":41,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":299420,"parent_record_offset":299352,"ancestor_record_offset":299352,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::default_anim_t
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A3B
// 首条位置: {"module":1306,"inlinee":23099}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A3D
// 首条位置: {"module":1371,"inlinee":23101}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::PlacementDataSsbo::to_transform
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A15
// 首条位置: {"module":401,"inlinee":23061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":401,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":3220,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":401,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":3220,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":497,"record_offset":11968,"parent_record_offset":11792,"ancestor_record_offset":11760,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":3364,"parent_record_offset":3248,"ancestor_record_offset":3220,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":3220,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":3220,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::PlacementDataSsbo::to_affine3a
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A3E
// 首条位置: {"module":401,"record_offset":3060,"rva":5570512,"va":5374279632}
// 编译实例 RVAs: 0x54ffd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: placement_data.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29314272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374279632,"raw_hex":"e8e04cbf01","size":5,"soft_fail":false,"target_va":5403594000,"va":5374279723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: placement_data.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29314619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374279632,"raw_hex":"e83b4ebf01","size":5,"soft_fail":false,"target_va":5403594336,"va":5374279712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":401,"record_offset":3220,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":23061,"source_line":73,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":401,"record_offset":3428,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":401,"record_offset":3892,"parent_record_offset":3452,"ancestor_record_offset":3060,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":401,"record_offset":3692,"parent_record_offset":3640,"ancestor_record_offset":3060,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":401,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":3060,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":401,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":3060,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":401,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":3060,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":401,"record_offset":3492,"parent_record_offset":3452,"ancestor_record_offset":3060,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":401,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":3060,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":401,"record_offset":3548,"parent_record_offset":3492,"ancestor_record_offset":3060,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":401,"record_offset":3640,"parent_record_offset":3452,"ancestor_record_offset":3060,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":401,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3060,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":3364,"parent_record_offset":3248,"ancestor_record_offset":3060,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":3060,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":3060,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::placement_data::PlacementDataSsbo::position
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":401,"record_offset":3988,"rva":5570896,"va":5374280016}
// 编译实例 RVAs: 0x550150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":401,"record_offset":4140,"parent_record_offset":3988,"ancestor_record_offset":3988,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":4180,"parent_record_offset":4160,"ancestor_record_offset":3988,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":4160,"parent_record_offset":4140,"ancestor_record_offset":3988,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/00401.jsonl.gz"}
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
// 完整结构及原始类型记录: placement_data.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::placement_data::PlacementDataSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 35
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::asset_libraries::Handle", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>", "constructor": "tuple$", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>", "usize"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::NaniInstanceData", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global> >", "constructor": "serde_json::de::from_str", "arguments": ["alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "serde_json::de::impl$9::next_element_seed", "arguments": ["serde_json::read::StrRead", "core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::StrRead", "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>", "country_core::startup::clearing::init_clearing::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["country_core::resources::tree_storage::ForestTree", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "tuple$", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::nani::gpu_world::NaniInstanceData<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$11::extend::closure_env$1", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once::closure$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "std::sync::once::impl$2::call_once::closure$0", "arguments": ["country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once::closure_env$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "std::sync::once::impl$2::call_once::closure_env$0", "arguments": ["country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once::closure_env$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >,tuple$<ref$<std::sync::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::once::impl$2::call_once::closure_env$0<country_core::render::nani::gpu_world::impl$11::extend::closure_env$1<country_core::resources::placement_data::PlacementDataSsbo> >", "tuple$<ref$<std::sync::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>", "constructor": "country_core::render::nani::gpu_world::GpuWorld::create_instances", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::GpuWorld::update_instance_data", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::type_name<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::type_name", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::as_any<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::as_any", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::as_any_mut<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::as_any_mut", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::gpu_flush", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$14::copy<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$14::copy", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::placement_data::PlacementDataSsbo,serde_json::de::SeqAccess<serde_json::read::StrRead> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo", "serde_json::de::SeqAccess<serde_json::read::StrRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo> >", "constructor": "serde::de::impl$1::fmt", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::placement_data::PlacementDataSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: placement_data.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: placement_data.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(ref_mut$<bevy_ecs::world::World>)
// 候选参数类型: ["ref_mut$<bevy_ecs::world::World>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::PlacementDataSsbo::position
// 信息 ID: 85367edf7d06be46d4f99d44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x143fc77 -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::tree_move_bend::tree_move_bend @ 0x1a8fadd -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::recalc_tree_position_on_terrain_height_change::recalc_tree_position_on_terrain_height_change @ 0x1a914f8 -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x1a927f5 -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x1a92c9a -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::convert_preplaced_trees::convert_preplaced_trees @ 0x1a93f14 -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::convert_preplaced_trees::convert_preplaced_trees @ 0x1a94281 -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x1ad3db0 -> 0x550150 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x1ad3df8 -> 0x550150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::PlacementDataSsbo::to_affine3a
// 信息 ID: 3f06703b80acd4268480489c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren::update_wrens @ 0x1b3d2f4 -> 0x54ffd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x1b41e5c -> 0x54ffd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x1b40add -> 0x54ffd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > > @ 0x1df9cc4 -> 0x54ffd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::Itertools::multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion> >,system_critters::wren_boids::sim_wren_boids::closure_env$2>,tuple$<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> > > @ 0x1df9eca -> 0x54ffd0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::prefab::create_prefab_rt; module=405, inline_record=13028; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::prefab::update_prefabs; module=405, inline_record=19200; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::PlacementDataSsbo::to_transform
// 信息 ID: 188585cd8fab127fe69c45fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::placement_data::PlacementDataSsbo::to_affine3a; module=401, inline_record=3220; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::prefab::create_prefab_rt; module=405, inline_record=13052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::prefab::update_prefabs; module=405, inline_record=19224; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::clearing::init_clearing; module=497, inline_record=11760; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; module=1363, inline_record=84408; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::glade_themes::olden::startup_olden; module=1403, inline_record=5052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize
// 信息 ID: 8d3dd4bc60e0c1aa2da76fe4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::placement_data::PlacementDataSsbo> >; module=1306, inline_record=91264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: e79e13f53dd1d0cb3f4a2357
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::placement_data::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0xd63e70 -> 0x5ae2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 93c917a8936c9b3004d000ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=294652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 2d648bd4a2bef2f9532c6bdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=424, inline_record=8540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: fad29e676cf0d0af650a9ff8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=293896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 215fcaf5ed8910813eb3d345
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=299352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::default_anim_t
// 信息 ID: 1a4ca83ffa50139acdfb4f63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=298788; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=299624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::placement_data::impl$1::default
// 信息 ID: 865322bcbebca2315df62605
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::resources::placement_data::PlacementDataSsbo>; module=1371, inline_record=55328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
