// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/clearing.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a181937d29977b1848c82f9d80f509e0896ee25a6cfea7ec145294858fa05065
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clearing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 497, 1306, 1308, 1363
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"PDB_registry_path_only","version":"1.9.0"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::util_add_clearing_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CF4
// 首条位置: {"module":497,"inlinee":27892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":497,"record_offset":21600,"parent_record_offset":21556,"ancestor_record_offset":21556,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":21768,"parent_record_offset":21556,"ancestor_record_offset":21556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":21920,"parent_record_offset":21896,"ancestor_record_offset":21556,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":21896,"parent_record_offset":21840,"ancestor_record_offset":21556,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":497,"record_offset":21648,"parent_record_offset":21624,"ancestor_record_offset":21556,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":21868,"parent_record_offset":21840,"ancestor_record_offset":21556,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":21840,"parent_record_offset":21816,"ancestor_record_offset":21556,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":21624,"parent_record_offset":21600,"ancestor_record_offset":21556,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":21792,"parent_record_offset":21768,"ancestor_record_offset":21556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEF91
// 首条位置: {"module":1306,"inlinee":61329}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1363,"record_offset":92416,"rva":15839648,"va":5384548768}
// 编译实例 RVAs: 0xf1b1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1363,"record_offset":92600,"parent_record_offset":92416,"ancestor_record_offset":92416,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1363,"record_offset":92620,"parent_record_offset":92600,"ancestor_record_offset":92416,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1363,"record_offset":92640,"parent_record_offset":92620,"ancestor_record_offset":92416,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1363,"record_offset":92660,"parent_record_offset":92640,"ancestor_record_offset":92416,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFB5
// 首条位置: {"module":1306,"inlinee":61365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":159016,"parent_record_offset":158992,"ancestor_record_offset":158992,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":159156,"parent_record_offset":159040,"ancestor_record_offset":158992,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":159072,"parent_record_offset":159040,"ancestor_record_offset":158992,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":159040,"parent_record_offset":159016,"ancestor_record_offset":158992,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":159096,"parent_record_offset":159072,"ancestor_record_offset":158992,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":159120,"parent_record_offset":159096,"ancestor_record_offset":158992,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D14
// 首条位置: {"module":1363,"inlinee":27924}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFB3
// 首条位置: {"module":1306,"inlinee":61363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1306,"record_offset":159240,"parent_record_offset":158200,"ancestor_record_offset":158200,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":161412,"parent_record_offset":158200,"ancestor_record_offset":158200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":161284,"parent_record_offset":158200,"ancestor_record_offset":158200,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":161664,"parent_record_offset":158200,"ancestor_record_offset":158200,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":158284,"parent_record_offset":158200,"ancestor_record_offset":158200,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":159268,"parent_record_offset":158200,"ancestor_record_offset":158200,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":161604,"parent_record_offset":161580,"ancestor_record_offset":158200,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":161580,"parent_record_offset":161556,"ancestor_record_offset":158200,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":161988,"parent_record_offset":161960,"ancestor_record_offset":158200,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":161556,"parent_record_offset":161532,"ancestor_record_offset":158200,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":159156,"parent_record_offset":159040,"ancestor_record_offset":158200,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":161436,"parent_record_offset":161412,"ancestor_record_offset":158200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":159200,"parent_record_offset":158336,"ancestor_record_offset":158200,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/clearing.rs
//     evidence={"module":1306,"record_offset":158992,"parent_record_offset":158968,"ancestor_record_offset":158200,"inlinee":61365,"source_line":56,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$56::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"1.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1306,"record_offset":161244,"parent_record_offset":161220,"ancestor_record_offset":158200,"inlinee":26667,"source_line":2081,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":161736,"parent_record_offset":161712,"ancestor_record_offset":158200,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":161824,"parent_record_offset":161800,"ancestor_record_offset":158200,"inlinee":6877,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":161712,"parent_record_offset":161688,"ancestor_record_offset":158200,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":158968,"parent_record_offset":158336,"ancestor_record_offset":158200,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":159732,"parent_record_offset":159708,"ancestor_record_offset":158200,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":158576,"parent_record_offset":158424,"ancestor_record_offset":158200,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":159352,"parent_record_offset":159308,"ancestor_record_offset":158200,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":158424,"parent_record_offset":158336,"ancestor_record_offset":158200,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":158464,"parent_record_offset":158424,"ancestor_record_offset":158200,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":158336,"parent_record_offset":158284,"ancestor_record_offset":158200,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":159308,"parent_record_offset":159268,"ancestor_record_offset":158200,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":159040,"parent_record_offset":159016,"ancestor_record_offset":158200,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":159016,"parent_record_offset":158992,"ancestor_record_offset":158200,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":158600,"parent_record_offset":158576,"ancestor_record_offset":158200,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":158496,"parent_record_offset":158464,"ancestor_record_offset":158200,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":158624,"parent_record_offset":158600,"ancestor_record_offset":158200,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":158528,"parent_record_offset":158496,"ancestor_record_offset":158200,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFB6
// 首条位置: {"module":1306,"inlinee":61366}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":163484,"parent_record_offset":163360,"ancestor_record_offset":163360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":163428,"parent_record_offset":163360,"ancestor_record_offset":163360,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":163684,"parent_record_offset":163360,"ancestor_record_offset":163360,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":163636,"parent_record_offset":163612,"ancestor_record_offset":163360,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":163612,"parent_record_offset":163556,"ancestor_record_offset":163360,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":163584,"parent_record_offset":163556,"ancestor_record_offset":163360,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":163556,"parent_record_offset":163532,"ancestor_record_offset":163360,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":163508,"parent_record_offset":163484,"ancestor_record_offset":163360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":164344,"parent_record_offset":163724,"ancestor_record_offset":163360,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":163940,"parent_record_offset":163800,"ancestor_record_offset":163360,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":163800,"parent_record_offset":163724,"ancestor_record_offset":163360,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":163836,"parent_record_offset":163800,"ancestor_record_offset":163360,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":163724,"parent_record_offset":163684,"ancestor_record_offset":163360,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":163964,"parent_record_offset":163940,"ancestor_record_offset":163360,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":163864,"parent_record_offset":163836,"ancestor_record_offset":163360,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":163988,"parent_record_offset":163964,"ancestor_record_offset":163360,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":163892,"parent_record_offset":163864,"ancestor_record_offset":163360,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":9048,"rva":7110336,"va":5375819456}
// 编译实例 RVAs: 0x6c7ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4169603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e87d60c0ff","size":5,"soft_fail":false,"target_va":5371652976,"va":5375822574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3513274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e84664caff","size":5,"soft_fail":false,"target_va":5372314208,"va":5375827477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3510122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e89670caff","size":5,"soft_fail":false,"target_va":5372320512,"va":5375830629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391380
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3378408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e81873ccff","size":5,"soft_fail":false,"target_va":5372449664,"va":5375828067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3199932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8442ccfff","size":5,"soft_fail":false,"target_va":5372630832,"va":5375830759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3070198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e80a27d1ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375819889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=34
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3071542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8ca21d1ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375821249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b5470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1128907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e835c6eeff","size":5,"soft_fail":false,"target_va":5374694512,"va":5375823414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1118558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8a2eeeeff","size":5,"soft_fail":false,"target_va":5374701184,"va":5375819737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=5
//     target_aliases=28 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-649872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e87015f6ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5375820923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406512d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-490336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8a084f8ff","size":5,"soft_fail":false,"target_va":5375333072,"va":5375823403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140651430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-489644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e85487f8ff","size":5,"soft_fail":false,"target_va":5375333424,"va":5375823063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b23a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e82b90feff","size":5,"soft_fail":false,"target_va":5375730592,"va":5375824752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-88681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e897a5feff","size":5,"soft_fail":false,"target_va":5375731504,"va":5375820180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8a50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t63714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8e2f80000","size":5,"soft_fail":false,"target_va":5375887952,"va":5375824233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t112533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e895b70100","size":5,"soft_fail":false,"target_va":5375936112,"va":5375823574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t162198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e896790200","size":5,"soft_fail":false,"target_va":5375988432,"va":5375826229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t461892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8440c0700","size":5,"soft_fail":false,"target_va":5376286288,"va":5375824391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d324c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6722355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e833936600","size":5,"soft_fail":false,"target_va":5382546624,"va":5375824264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6793372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e89ca86700","size":5,"soft_fail":false,"target_va":5382615664,"va":5375822287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d748c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6994677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8f5ba6a00","size":5,"soft_fail":false,"target_va":5382817984,"va":5375823302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76e80
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7004077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8addf6a00","size":5,"soft_fail":false,"target_va":5382827648,"va":5375823566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f1abc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8723527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8471c8500","size":5,"soft_fail":false,"target_va":5384547264,"va":5375823732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141024e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9814228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8d4c09500","size":5,"soft_fail":false,"target_va":5385637408,"va":5375823175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141086720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10216029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e85de29b00","size":5,"soft_fail":false,"target_va":5386037024,"va":5375820990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27772881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8d1c7a701","size":5,"soft_fail":false,"target_va":5403594000,"va":5375821114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27773228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e82cc9a701","size":5,"soft_fail":false,"target_va":5403594336,"va":5375821103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27152462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e84e509e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375825677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1705283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e843051a00","size":5,"soft_fail":false,"target_va":5377529888,"va":5375824600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27095794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8f2729d01","size":5,"soft_fail":false,"target_va":5402919600,"va":5375823801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1537974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8b6771700","size":5,"soft_fail":false,"target_va":5377364336,"va":5375826357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8544130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8825f8200","size":5,"soft_fail":false,"target_va":5384366112,"va":5375821977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t8555887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e86f8d8200","size":5,"soft_fail":false,"target_va":5384376160,"va":5375820268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1424509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e87dbc1500","size":5,"soft_fail":false,"target_va":5377246528,"va":5375822014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27152813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8ad519e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375826062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t27154652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8dc589e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375825471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-1254035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e86dddecff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375820110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-128321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8bf0afeff","size":5,"soft_fail":false,"target_va":5375692416,"va":5375820732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8745772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e82c738500","size":5,"soft_fail":false,"target_va":5384569328,"va":5375823551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::glade_themes::olden::startup_olden
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9940861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e87daf9700","size":5,"soft_fail":false,"target_va":5385762832,"va":5375821966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1534124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8ac681700","size":5,"soft_fail":false,"target_va":5377353776,"va":5375819647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1583410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e832291800","size":5,"soft_fail":false,"target_va":5377405616,"va":5375822201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t1618856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e8a8b31800","size":5,"soft_fail":false,"target_va":5377439776,"va":5375820915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1124618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375819456,"raw_hex":"e80a291100","size":5,"soft_fail":false,"target_va":5376946416,"va":5375821793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$82::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":497,"record_offset":17256,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":27266,"source_line":2321,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":16360,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":17104,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":18328,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":497,"record_offset":17160,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":14532,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":14592,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":12139,"source_line":622,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":9384,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":497,"record_offset":9412,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":497,"record_offset":18456,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":15860,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":15772,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":9580,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":18244,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":18604,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":497,"record_offset":9468,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":497,"record_offset":21320,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":14680,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":10332,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":27887,"source_line":220,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_olden
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":14468,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":27890,"source_line":215,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":9524,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::InteractableForestPlacements::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":497,"record_offset":16448,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":25775,"source_line":105,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::clearing::util_add_clearing_shader
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/clearing.rs
//     evidence={"module":497,"record_offset":21556,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":27892,"source_line":41,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::glade_themes::flowery::startup_flowery
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/glade_themes/flowery.rs
//     evidence={"module":497,"record_offset":10388,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":27889,"source_line":17,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":497,"record_offset":9656,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":497,"record_offset":9440,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":497,"record_offset":9300,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":497,"record_offset":14964,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":497,"record_offset":14932,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":497,"record_offset":14792,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":497,"record_offset":10304,"parent_record_offset":9048,"ancestor_record_offset":9048,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":9048,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":9048,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":12192,"parent_record_offset":12168,"ancestor_record_offset":9048,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":10056,"parent_record_offset":10028,"ancestor_record_offset":9048,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":12168,"parent_record_offset":12076,"ancestor_record_offset":9048,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":497,"record_offset":21600,"parent_record_offset":21556,"ancestor_record_offset":9048,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":497,"record_offset":21648,"parent_record_offset":21624,"ancestor_record_offset":9048,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":9048,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":19192,"parent_record_offset":19152,"ancestor_record_offset":9048,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":19120,"parent_record_offset":19096,"ancestor_record_offset":9048,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":9048,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":10000,"parent_record_offset":9972,"ancestor_record_offset":9048,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":10176,"parent_record_offset":10000,"ancestor_record_offset":9048,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":9048,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":9936,"parent_record_offset":9908,"ancestor_record_offset":9048,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":9908,"parent_record_offset":9880,"ancestor_record_offset":9048,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":9852,"parent_record_offset":9824,"ancestor_record_offset":9048,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":9880,"parent_record_offset":9852,"ancestor_record_offset":9048,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":9824,"parent_record_offset":9796,"ancestor_record_offset":9048,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":497,"record_offset":9796,"parent_record_offset":9768,"ancestor_record_offset":9048,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":497,"record_offset":9768,"parent_record_offset":9720,"ancestor_record_offset":9048,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":16700,"parent_record_offset":16660,"ancestor_record_offset":9048,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":9048,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":16660,"parent_record_offset":16632,"ancestor_record_offset":9048,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":19152,"parent_record_offset":18804,"ancestor_record_offset":9048,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":16520,"parent_record_offset":16484,"ancestor_record_offset":9048,"inlinee":4126,"source_line":1244,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":16484,"parent_record_offset":16448,"ancestor_record_offset":9048,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":9048,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":19068,"parent_record_offset":18804,"ancestor_record_offset":9048,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":19096,"parent_record_offset":18804,"ancestor_record_offset":9048,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":19224,"parent_record_offset":18804,"ancestor_record_offset":9048,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":9972,"parent_record_offset":9936,"ancestor_record_offset":9048,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":9048,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":497,"record_offset":18780,"parent_record_offset":18728,"ancestor_record_offset":9048,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":9048,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":11336,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":14616,"parent_record_offset":14592,"ancestor_record_offset":9048,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":497,"record_offset":14644,"parent_record_offset":14616,"ancestor_record_offset":9048,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":17312,"parent_record_offset":17284,"ancestor_record_offset":9048,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":497,"record_offset":10236,"parent_record_offset":10212,"ancestor_record_offset":9048,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":497,"record_offset":18848,"parent_record_offset":18804,"ancestor_record_offset":9048,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":497,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":9048,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":9048,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":9048,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":21624,"parent_record_offset":21600,"ancestor_record_offset":9048,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":11272,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":10212,"parent_record_offset":9936,"ancestor_record_offset":9048,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=134
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":9624,"parent_record_offset":9580,"ancestor_record_offset":9048,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":17128,"parent_record_offset":17104,"ancestor_record_offset":9048,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":17224,"parent_record_offset":17160,"ancestor_record_offset":9048,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":16732,"parent_record_offset":16700,"ancestor_record_offset":9048,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":11724,"parent_record_offset":11680,"ancestor_record_offset":9048,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":10132,"parent_record_offset":10104,"ancestor_record_offset":9048,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":21452,"parent_record_offset":21352,"ancestor_record_offset":9048,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":10548,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":497,"record_offset":17340,"parent_record_offset":17312,"ancestor_record_offset":9048,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":497,"record_offset":17368,"parent_record_offset":17340,"ancestor_record_offset":9048,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":21396,"parent_record_offset":21352,"ancestor_record_offset":9048,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":497,"record_offset":18268,"parent_record_offset":18244,"ancestor_record_offset":9048,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":18640,"parent_record_offset":18604,"ancestor_record_offset":9048,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":18876,"parent_record_offset":18848,"ancestor_record_offset":9048,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":11680,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":17284,"parent_record_offset":17256,"ancestor_record_offset":9048,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":9048,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":9048,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":16760,"parent_record_offset":16700,"ancestor_record_offset":9048,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":16792,"parent_record_offset":16660,"ancestor_record_offset":9048,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":497,"record_offset":9720,"parent_record_offset":9656,"ancestor_record_offset":9048,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":497,"record_offset":9692,"parent_record_offset":9656,"ancestor_record_offset":9048,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":497,"record_offset":21352,"parent_record_offset":21320,"ancestor_record_offset":9048,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":497,"record_offset":11760,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":23061,"source_line":73,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":11396,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":9048,"inlinee":21875,"source_line":44,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9048,"inlinee":23012,"source_line":93,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":14496,"parent_record_offset":14468,"ancestor_record_offset":9048,"inlinee":23011,"source_line":85,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":9048,"inlinee":21928,"source_line":109,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":497,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":9048,"inlinee":21873,"source_line":153,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":497,"record_offset":10624,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":497,"record_offset":11824,"parent_record_offset":11792,"ancestor_record_offset":9048,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":497,"record_offset":11792,"parent_record_offset":11760,"ancestor_record_offset":9048,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":497,"record_offset":11968,"parent_record_offset":11792,"ancestor_record_offset":9048,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":11912,"parent_record_offset":11792,"ancestor_record_offset":9048,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":497,"record_offset":21424,"parent_record_offset":21352,"ancestor_record_offset":9048,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":497,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":9048,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":497,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":9048,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":497,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":9048,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":497,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":9048,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":497,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":9048,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":497,"record_offset":11472,"parent_record_offset":11420,"ancestor_record_offset":9048,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":497,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":9048,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":497,"record_offset":11508,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":497,"record_offset":15000,"parent_record_offset":14964,"ancestor_record_offset":9048,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":497,"record_offset":11308,"parent_record_offset":10388,"ancestor_record_offset":9048,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF02F
// 首条位置: {"module":1308,"inlinee":61487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1308,"record_offset":103616,"parent_record_offset":103544,"ancestor_record_offset":103544,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1308,"record_offset":103588,"parent_record_offset":103544,"ancestor_record_offset":103544,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1308,"record_offset":103644,"parent_record_offset":103616,"ancestor_record_offset":103544,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":103680,"parent_record_offset":103644,"ancestor_record_offset":103544,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":103704,"parent_record_offset":103680,"ancestor_record_offset":103544,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF757
// 首条位置: {"module":1363,"inlinee":63319}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1363,"record_offset":84408,"parent_record_offset":84280,"ancestor_record_offset":84280,"inlinee":23061,"source_line":73,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1363,"record_offset":85080,"parent_record_offset":84280,"ancestor_record_offset":84280,"inlinee":26366,"source_line":370,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1363,"record_offset":84316,"parent_record_offset":84280,"ancestor_record_offset":84280,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1363,"record_offset":84616,"parent_record_offset":84280,"ancestor_record_offset":84280,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1363,"record_offset":85220,"parent_record_offset":85120,"ancestor_record_offset":84280,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1363,"record_offset":84872,"parent_record_offset":84820,"ancestor_record_offset":84280,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1363,"record_offset":84844,"parent_record_offset":84820,"ancestor_record_offset":84280,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1363,"record_offset":84576,"parent_record_offset":84552,"ancestor_record_offset":84280,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1363,"record_offset":84640,"parent_record_offset":84616,"ancestor_record_offset":84280,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1363,"record_offset":84464,"parent_record_offset":84436,"ancestor_record_offset":84280,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1363,"record_offset":84672,"parent_record_offset":84640,"ancestor_record_offset":84280,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1363,"record_offset":84436,"parent_record_offset":84408,"ancestor_record_offset":84280,"inlinee":14670,"source_line":137,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1363,"record_offset":84728,"parent_record_offset":84672,"ancestor_record_offset":84280,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1363,"record_offset":84820,"parent_record_offset":84640,"ancestor_record_offset":84280,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1363,"record_offset":84340,"parent_record_offset":84316,"ancestor_record_offset":84280,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1363,"record_offset":84552,"parent_record_offset":84436,"ancestor_record_offset":84280,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1363,"record_offset":84512,"parent_record_offset":84488,"ancestor_record_offset":84280,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1363,"record_offset":84488,"parent_record_offset":84464,"ancestor_record_offset":84280,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1363,"record_offset":84372,"parent_record_offset":84340,"ancestor_record_offset":84280,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01363.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::uninit_clearing
// 记录形式: compiled_procedure; instances=1
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":26112,"rva":7121712,"va":5375830832}
// 编译实例 RVAs: 0x6cab30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4178306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e87e3ec0ff","size":5,"soft_fail":false,"target_va":5371652976,"va":5375831277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3081804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8b4f9d0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375831495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3082084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e89cf8d0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375831791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1162105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e88744eeff","size":5,"soft_fail":false,"target_va":5374669984,"va":5375832084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e63d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-935931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e805b8f1ff","size":5,"soft_fail":false,"target_va":5374895056,"va":5375830982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e5f70
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t110228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e894ae0100","size":5,"soft_fail":false,"target_va":5375942512,"va":5375832279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f21d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t160504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8f8720200","size":5,"soft_fail":false,"target_va":5375992272,"va":5375831763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df4390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7509931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8ab977200","size":5,"soft_fail":false,"target_va":5383340944,"va":5375831008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df47b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7511083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e82b9c7200","size":5,"soft_fail":false,"target_va":5383342000,"va":5375830912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df4ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7512399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e84fa17200","size":5,"soft_fail":false,"target_va":5383343328,"va":5375830924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27762219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e82b9ea701","size":5,"soft_fail":false,"target_va":5403593392,"va":5375831168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27146206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8de379e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375831933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1532011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e86b601700","size":5,"soft_fail":false,"target_va":5377364336,"va":5375832320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27148655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e86f419e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375831468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1155654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8ba5deeff","size":5,"soft_fail":false,"target_va":5374675760,"va":5375831409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6770633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375830832,"raw_hex":"e8c94f6700","size":5,"soft_fail":false,"target_va":5382602288,"va":5375831650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":28584,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":26292,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":28688,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":29620,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":29372,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":28612,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":497,"record_offset":26544,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":27893,"source_line":76,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/placement_data_library.rs
//     evidence={"module":497,"record_offset":26348,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":22883,"source_line":35,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":497,"record_offset":27060,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":26141,"source_line":426,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":497,"record_offset":28744,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":497,"record_offset":29448,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":21884,"source_line":387,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtWorld::reset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":497,"record_offset":26912,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":25722,"source_line":146,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::InteractableForestPlacements::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":497,"record_offset":27884,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":25775,"source_line":105,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":29128,"parent_record_offset":29104,"ancestor_record_offset":26112,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":29152,"parent_record_offset":29128,"ancestor_record_offset":26112,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":29832,"parent_record_offset":29808,"ancestor_record_offset":26112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":29104,"parent_record_offset":29076,"ancestor_record_offset":26112,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":29808,"parent_record_offset":29752,"ancestor_record_offset":26112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":29780,"parent_record_offset":29752,"ancestor_record_offset":26112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":29076,"parent_record_offset":29048,"ancestor_record_offset":26112,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":29048,"parent_record_offset":29020,"ancestor_record_offset":26112,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":29224,"parent_record_offset":29048,"ancestor_record_offset":26112,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":29752,"parent_record_offset":29724,"ancestor_record_offset":26112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":28984,"parent_record_offset":28956,"ancestor_record_offset":26112,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":28956,"parent_record_offset":28928,"ancestor_record_offset":26112,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":28900,"parent_record_offset":28872,"ancestor_record_offset":26112,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":28928,"parent_record_offset":28900,"ancestor_record_offset":26112,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":28872,"parent_record_offset":28844,"ancestor_record_offset":26112,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":497,"record_offset":28844,"parent_record_offset":28744,"ancestor_record_offset":26112,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":28136,"parent_record_offset":28096,"ancestor_record_offset":26112,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":28816,"parent_record_offset":28744,"ancestor_record_offset":26112,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":28096,"parent_record_offset":28068,"ancestor_record_offset":26112,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":27956,"parent_record_offset":27920,"ancestor_record_offset":26112,"inlinee":4126,"source_line":1244,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":27556,"parent_record_offset":27128,"ancestor_record_offset":26112,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":29020,"parent_record_offset":28984,"ancestor_record_offset":26112,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":497,"record_offset":29340,"parent_record_offset":28744,"ancestor_record_offset":26112,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":26316,"parent_record_offset":26292,"ancestor_record_offset":26112,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":497,"record_offset":29284,"parent_record_offset":29260,"ancestor_record_offset":26112,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":497,"record_offset":27492,"parent_record_offset":27456,"ancestor_record_offset":26112,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$25::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":497,"record_offset":27592,"parent_record_offset":27060,"ancestor_record_offset":26112,"inlinee":13656,"source_line":297,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":497,"record_offset":27616,"parent_record_offset":27592,"ancestor_record_offset":26112,"inlinee":26113,"source_line":938,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":497,"record_offset":27668,"parent_record_offset":27644,"ancestor_record_offset":26112,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":497,"record_offset":27644,"parent_record_offset":27616,"ancestor_record_offset":26112,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":497,"record_offset":26692,"parent_record_offset":26668,"ancestor_record_offset":26112,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":497,"record_offset":26788,"parent_record_offset":26764,"ancestor_record_offset":26112,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":497,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":26112,"inlinee":6243,"source_line":4117,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::spec_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":497,"record_offset":26740,"parent_record_offset":26716,"ancestor_record_offset":26112,"inlinee":6219,"source_line":372,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::spec_try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":497,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":26112,"inlinee":8582,"source_line":4172,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":27776,"parent_record_offset":27668,"ancestor_record_offset":26112,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":28788,"parent_record_offset":28744,"ancestor_record_offset":26112,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":497,"record_offset":28712,"parent_record_offset":28688,"ancestor_record_offset":26112,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":29260,"parent_record_offset":28984,"ancestor_record_offset":26112,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":26644,"parent_record_offset":26592,"ancestor_record_offset":26112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":26112,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":28168,"parent_record_offset":28136,"ancestor_record_offset":26112,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":497,"record_offset":28652,"parent_record_offset":28612,"ancestor_record_offset":26112,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":29180,"parent_record_offset":29152,"ancestor_record_offset":26112,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":29520,"parent_record_offset":29448,"ancestor_record_offset":26112,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":27716,"parent_record_offset":27692,"ancestor_record_offset":26112,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":497,"record_offset":27692,"parent_record_offset":27668,"ancestor_record_offset":26112,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":28196,"parent_record_offset":28136,"ancestor_record_offset":26112,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":28228,"parent_record_offset":28096,"ancestor_record_offset":26112,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":497,"record_offset":26568,"parent_record_offset":26544,"ancestor_record_offset":26112,"inlinee":27894,"source_line":23,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":497,"record_offset":27092,"parent_record_offset":27060,"ancestor_record_offset":26112,"inlinee":26115,"source_line":604,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":497,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":26112,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27216,"parent_record_offset":27188,"ancestor_record_offset":26112,"inlinee":14998,"source_line":1092,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27336,"parent_record_offset":27308,"ancestor_record_offset":26112,"inlinee":15001,"source_line":1086,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27364,"parent_record_offset":27336,"ancestor_record_offset":26112,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27456,"parent_record_offset":27364,"ancestor_record_offset":26112,"inlinee":11753,"source_line":2799,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27428,"parent_record_offset":27364,"ancestor_record_offset":26112,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27400,"parent_record_offset":27364,"ancestor_record_offset":26112,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27308,"parent_record_offset":27280,"ancestor_record_offset":26112,"inlinee":15000,"source_line":1098,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\scopeguard.rs
//     evidence={"module":497,"record_offset":27280,"parent_record_offset":27252,"ancestor_record_offset":26112,"inlinee":8805,"source_line":69,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":497,"record_offset":26592,"parent_record_offset":26568,"ancestor_record_offset":26112,"inlinee":25369,"source_line":497,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::drain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":497,"record_offset":26616,"parent_record_offset":26592,"ancestor_record_offset":26112,"inlinee":27895,"source_line":522,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":497,"record_offset":26668,"parent_record_offset":26644,"ancestor_record_offset":26112,"inlinee":25360,"source_line":973,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":497,"record_offset":26372,"parent_record_offset":26348,"ancestor_record_offset":26112,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::index_condenser::IndexCondenser::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/index_condenser.rs
//     evidence={"module":497,"record_offset":27128,"parent_record_offset":27092,"ancestor_record_offset":26112,"inlinee":23537,"source_line":172,"archive":"_pdb/A/modules/00497.jsonl.gz"}
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
// 完整结构及原始类型记录: clearing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::clearing::PlacementConfigEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "serde_json::de::from_str", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry> >", "constructor": "serde_json::de::impl$9::next_element_seed", "arguments": ["serde_json::read::StrRead", "core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::StrRead", "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::clearing::PlacementConfigEntry,serde_json::de::SeqAccess<serde_json::read::StrRead> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "serde_json::de::SeqAccess<serde_json::read::StrRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clearing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clearing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 13; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize
// 信息 ID: d2613d41973db74dcfe5ff65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry> >; module=1306, inline_record=92956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 0763cccb05a52ef2fb6193e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor> @ 0xd4fded -> 0xf1b1a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 77f1577dae415ee150350ad0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=158992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: ac23eaf5b663cb6ab4fe2675
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1363, inline_record=83808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 60f746cf28def29e3db70d39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=158200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: de508681ef4ad16e46e77de4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=163360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing
// 信息 ID: 472fa0b5b24e7e1503185644
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a313fd7c1a3b9a3b0669a9e6: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > @ 0xe7f928 -> 0x6c7ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> @ 0xf1a31f -> 0x6c7ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing::closure$0
// 信息 ID: 1850dafd78b109c1cdbc92ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::clearing::init_clearing", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::clearing::init_clearing::closure_env$0 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>; module=1308, inline_record=103544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing::closure$1
// 信息 ID: e6c353898e971b500fdd709f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::clearing::init_clearing", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::clearing::init_clearing::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; module=1363, inline_record=84280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::uninit_clearing
// 信息 ID: f0e227c88132de67fbfc5597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f1571263866702a2d9c5663c: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)> > @ 0xe5a7d1 -> 0x6cab30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)> @ 0xf19aef -> 0x6cab30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::util_add_clearing_shader
// 信息 ID: 1a274dac3e0f36a4d6c82944
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::clearing::init_clearing; module=497, inline_record=21556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
