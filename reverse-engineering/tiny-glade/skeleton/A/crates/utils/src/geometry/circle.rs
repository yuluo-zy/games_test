// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/circle.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a6a6544628791432de0283fc61f93cac7c58009a915b2bc7938e5253a8590e7e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 114, 741, 1321, 1326, 1350, 1399, 1421, 1426, 2269, 2825, 2831, 3086, 3194, 3215, 3217, 4114, 4466
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1F8
// 首条位置: {"module":1326,"inlinee":61944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":6744,"rva":9261152,"va":5377970272}
// 编译实例 RVAs: 0x8d5060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-386373","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377970272,"raw_hex":"e9bb1afaff","size":5,"soft_fail":false,"target_va":5377583920,"va":5377970288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDE6
// 首条位置: {"module":1421,"inlinee":64998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDE7
// 首条位置: {"module":1421,"inlinee":64999}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1421,"record_offset":18412,"parent_record_offset":18392,"ancestor_record_offset":18392,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1421,"record_offset":18436,"parent_record_offset":18412,"ancestor_record_offset":18392,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":18460,"parent_record_offset":18436,"ancestor_record_offset":18392,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":18484,"parent_record_offset":18460,"ancestor_record_offset":18392,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDE0
// 首条位置: {"module":1399,"inlinee":64992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10245
// 首条位置: {"module":1421,"inlinee":66117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":6888,"rva":9261184,"va":5377970304}
// 编译实例 RVAs: 0x8d5080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-386405","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377970304,"raw_hex":"e99b1afaff","size":5,"soft_fail":false,"target_va":5377583920,"va":5377970320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10243
// 首条位置: {"module":1421,"inlinee":66115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":9388,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":10380,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":10412,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":8584,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":9420,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":10016,"parent_record_offset":9988,"ancestor_record_offset":8544,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":9988,"parent_record_offset":9952,"ancestor_record_offset":8544,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":8988,"parent_record_offset":8912,"ancestor_record_offset":8544,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":8912,"parent_record_offset":8836,"ancestor_record_offset":8544,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1421,"record_offset":9144,"parent_record_offset":8988,"ancestor_record_offset":8544,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":9064,"parent_record_offset":8988,"ancestor_record_offset":8544,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1421,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":8544,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":9268,"parent_record_offset":8744,"ancestor_record_offset":8544,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":9192,"parent_record_offset":9168,"ancestor_record_offset":8544,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":8544,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1421,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":8544,"inlinee":24930,"source_line":16,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":9828,"parent_record_offset":9760,"ancestor_record_offset":8544,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":9760,"parent_record_offset":9692,"ancestor_record_offset":8544,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":8544,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1421,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":8544,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1421,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":8544,"inlinee":11581,"source_line":126,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":8664,"parent_record_offset":8584,"ancestor_record_offset":8544,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":9488,"parent_record_offset":9420,"ancestor_record_offset":8544,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":9312,"parent_record_offset":8744,"ancestor_record_offset":8544,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":9692,"parent_record_offset":9624,"ancestor_record_offset":8544,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":9952,"parent_record_offset":9828,"ancestor_record_offset":8544,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":10152,"parent_record_offset":9828,"ancestor_record_offset":8544,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":10120,"parent_record_offset":9828,"ancestor_record_offset":8544,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":10184,"parent_record_offset":9828,"ancestor_record_offset":8544,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":10088,"parent_record_offset":9828,"ancestor_record_offset":8544,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":8836,"parent_record_offset":8744,"ancestor_record_offset":8544,"inlinee":7217,"source_line":1146,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":8744,"parent_record_offset":8664,"ancestor_record_offset":8544,"inlinee":7249,"source_line":1210,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":9556,"parent_record_offset":9488,"ancestor_record_offset":8544,"inlinee":7251,"source_line":1223,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1421,"record_offset":10052,"parent_record_offset":9952,"ancestor_record_offset":8544,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1421,"record_offset":9340,"parent_record_offset":9312,"ancestor_record_offset":8544,"inlinee":66117,"source_line":12,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10246
// 首条位置: {"module":1421,"inlinee":66118}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":10744,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":10700,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":10700,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":10700,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":10840,"parent_record_offset":10808,"ancestor_record_offset":10700,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":10888,"parent_record_offset":10864,"ancestor_record_offset":10700,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":10980,"parent_record_offset":10808,"ancestor_record_offset":10700,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1421,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":10700,"inlinee":24930,"source_line":16,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":11308,"parent_record_offset":11276,"ancestor_record_offset":10700,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":11276,"parent_record_offset":11244,"ancestor_record_offset":10700,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":10776,"parent_record_offset":10744,"ancestor_record_offset":10700,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":10924,"parent_record_offset":10808,"ancestor_record_offset":10700,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":11244,"parent_record_offset":11212,"ancestor_record_offset":10700,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":11376,"parent_record_offset":11308,"ancestor_record_offset":10700,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":11576,"parent_record_offset":11308,"ancestor_record_offset":10700,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":11548,"parent_record_offset":11308,"ancestor_record_offset":10700,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":11492,"parent_record_offset":11308,"ancestor_record_offset":10700,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":11520,"parent_record_offset":11308,"ancestor_record_offset":10700,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":10808,"parent_record_offset":10776,"ancestor_record_offset":10700,"inlinee":7253,"source_line":963,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1421,"record_offset":11460,"parent_record_offset":11376,"ancestor_record_offset":10700,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":114,"record_offset":9368,"rva":1730560,"va":5370439680}
// 编译实例 RVAs: 0x1a6800, 0x1033fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1633068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370439680,"raw_hex":"e8d414e7ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5370440103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-328386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370439680,"raw_hex":"e83efdfaff","size":5,"soft_fail":false,"target_va":5370111424,"va":5370439805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140156880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-327644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370439680,"raw_hex":"e82400fbff","size":5,"soft_fail":false,"target_va":5370112128,"va":5370439767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1813905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370439680,"raw_hex":"e891ad1b00","size":5,"soft_fail":false,"target_va":5372253808,"va":5370439898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2309780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370439680,"raw_hex":"e8943e2300","size":5,"soft_fail":false,"target_va":5372749712,"va":5370439927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10917180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385699296,"raw_hex":"e8c46a59ff","size":5,"soft_fail":false,"target_va":5374782544,"va":5385699719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9807647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385699296,"raw_hex":"e8e1586aff","size":5,"soft_fail":false,"target_va":5375891872,"va":5385699514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5586010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370439680,"raw_hex":"e85a3c5500","size":5,"soft_fail":false,"target_va":5376025984,"va":5370439969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t405214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385699296,"raw_hex":"e8de2e0600","size":5,"soft_fail":false,"target_va":5386104640,"va":5385699421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109a3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t418676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385699296,"raw_hex":"e874630600","size":5,"soft_fail":false,"target_va":5386118064,"va":5385699383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":9568,"parent_record_offset":9368,"ancestor_record_offset":9368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":114,"record_offset":9540,"parent_record_offset":9368,"ancestor_record_offset":9368,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":9368,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":9928,"parent_record_offset":9872,"ancestor_record_offset":9368,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":9368,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":9700,"parent_record_offset":9644,"ancestor_record_offset":9368,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":9368,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":9368,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":114,"record_offset":9732,"parent_record_offset":9644,"ancestor_record_offset":9368,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":9596,"parent_record_offset":9568,"ancestor_record_offset":9368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":9668,"parent_record_offset":9644,"ancestor_record_offset":9368,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$3::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDD5
// 首条位置: {"module":1399,"inlinee":64981}
// 编译实例 RVAs: 0x1802f30, 0x181ef60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16302929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393887024,"raw_hex":"e8af3c07ff","size":5,"soft_fail":false,"target_va":5377584208,"va":5393887132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D26
// 首条位置: {"module":2269,"inlinee":81190}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2269,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":15928,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/02269.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB33B
// 首条位置: {"module":741,"inlinee":45883}
// 编译实例 RVAs: 0x8d46f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::area
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":360,"rva":9258768,"va":5377967888}
// 编译实例 RVAs: 0x8d4710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":484,"parent_record_offset":360,"ancestor_record_offset":360,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_pos_at_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB33D
// 首条位置: {"module":741,"record_offset":512,"rva":9258800,"va":5377967920}
// 编译实例 RVAs: 0x8d4730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25626034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377967920,"raw_hex":"e8b2058701","size":5,"soft_fail":false,"target_va":5403594000,"va":5377967961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25626359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377967920,"raw_hex":"e8f7068701","size":5,"soft_fail":false,"target_va":5403594336,"va":5377967972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":820,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":736,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":652,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":45875,"source_line":47,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":512,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":512,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":512,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":704,"parent_record_offset":652,"ancestor_record_offset":512,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":908,"rva":9258912,"va":5377968032}
// 编译实例 RVAs: 0x8d47a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144b80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25625415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968032,"raw_hex":"e847038701","size":5,"soft_fail":false,"target_va":5403593600,"va":5377968180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25625707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968032,"raw_hex":"e86b048701","size":5,"soft_fail":false,"target_va":5403593904,"va":5377968192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25625927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968032,"raw_hex":"e847058701","size":5,"soft_fail":false,"target_va":5403594144,"va":5377968212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25010769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968032,"raw_hex":"e851a27d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377968378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25010585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968032,"raw_hex":"e899a17d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377968354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":1120,"parent_record_offset":908,"ancestor_record_offset":908,"inlinee":45876,"source_line":465,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":1064,"parent_record_offset":908,"ancestor_record_offset":908,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::cartesian_to_polar
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":741,"record_offset":1284,"parent_record_offset":908,"ancestor_record_offset":908,"inlinee":36093,"source_line":405,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":741,"record_offset":1372,"parent_record_offset":908,"ancestor_record_offset":908,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":741,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":908,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":741,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":908,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":741,"record_offset":1524,"parent_record_offset":1372,"ancestor_record_offset":908,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":741,"record_offset":1552,"parent_record_offset":1372,"ancestor_record_offset":908,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":908,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":741,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":908,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":741,"record_offset":1412,"parent_record_offset":1372,"ancestor_record_offset":908,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":908,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":908,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":1152,"parent_record_offset":1120,"ancestor_record_offset":908,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":1340,"parent_record_offset":1284,"ancestor_record_offset":908,"inlinee":13077,"source_line":886,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":908,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":1240,"parent_record_offset":1208,"ancestor_record_offset":908,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_normal_at_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB333
// 首条位置: {"module":741,"inlinee":45875}
// 编译实例 RVAs: 0x8d4900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25625574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968384,"raw_hex":"e8e6038701","size":5,"soft_fail":false,"target_va":5403594000,"va":5377968421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25625899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377968384,"raw_hex":"e82b058701","size":5,"soft_fail":false,"target_va":5403594336,"va":5377968432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":704,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 78, 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB335
// 首条位置: {"module":741,"inlinee":45877}
// 编译实例 RVAs: 0x8d4950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::intersect_ray
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":1908,"rva":9259360,"va":5377968480}
// 编译实例 RVAs: 0x8d4960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2044,"parent_record_offset":1908,"ancestor_record_offset":1908,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":2072,"parent_record_offset":1908,"ancestor_record_offset":1908,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::intersect_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":2104,"rva":9259552,"va":5377968672}
// 编译实例 RVAs: 0x8d4a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2264,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2524,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2608,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2468,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2552,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":2412,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":2440,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":2104,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":2104,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":2312,"parent_record_offset":2284,"ancestor_record_offset":2104,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2340,"parent_record_offset":2284,"ancestor_record_offset":2104,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2284,"parent_record_offset":2264,"ancestor_record_offset":2104,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":2372,"parent_record_offset":2340,"ancestor_record_offset":2104,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::is_point_inside
// 记录形式: compiled_procedure; instances=1
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":2780,"rva":9259760,"va":5377968880}
// 编译实例 RVAs: 0x8d4af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2916,"parent_record_offset":2780,"ancestor_record_offset":2780,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":2780,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2992,"parent_record_offset":2936,"ancestor_record_offset":2780,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":2936,"parent_record_offset":2916,"ancestor_record_offset":2780,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":3020,"parent_record_offset":2992,"ancestor_record_offset":2780,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::closest_point_on_perimeter
// 记录形式: compiled_procedure; instances=1
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":3064,"rva":9259808,"va":5377968928}
// 编译实例 RVAs: 0x8d4b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3276,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3572,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3224,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3488,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":3420,"parent_record_offset":3296,"ancestor_record_offset":3064,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":3064,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":3244,"parent_record_offset":3224,"ancestor_record_offset":3064,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3356,"parent_record_offset":3328,"ancestor_record_offset":3064,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3328,"parent_record_offset":3296,"ancestor_record_offset":3064,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":3296,"parent_record_offset":3276,"ancestor_record_offset":3064,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":3384,"parent_record_offset":3356,"ancestor_record_offset":3064,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points2_ptcount
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB336
// 首条位置: {"module":741,"inlinee":45878}
// 编译实例 RVAs: 0x8d4b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":741,"record_offset":3800,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":741,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":3660,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=19
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF12F
// 首条位置: {"module":1321,"inlinee":61743}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1321,"record_offset":1060,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3086,"record_offset":1264,"parent_record_offset":1024,"ancestor_record_offset":1024,"inlinee":45885,"source_line":27,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1321,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":1028,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1321,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":1028,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1321,"record_offset":1208,"parent_record_offset":1060,"ancestor_record_offset":1028,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":1236,"parent_record_offset":1060,"ancestor_record_offset":1028,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3086,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":1024,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3086,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":1024,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1321,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":1028,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1321,"record_offset":1096,"parent_record_offset":1060,"ancestor_record_offset":1028,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3086,"record_offset":1436,"parent_record_offset":1264,"ancestor_record_offset":1024,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3086,"record_offset":1380,"parent_record_offset":1264,"ancestor_record_offset":1024,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3086,"record_offset":1320,"parent_record_offset":1292,"ancestor_record_offset":1024,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3086,"record_offset":1348,"parent_record_offset":1292,"ancestor_record_offset":1024,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3086,"record_offset":1292,"parent_record_offset":1264,"ancestor_record_offset":1024,"inlinee":45875,"source_line":47,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB337
// 首条位置: {"module":741,"inlinee":45879}
// 编译实例 RVAs: 0x8d4ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25624878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969056,"raw_hex":"e82e018701","size":5,"soft_fail":false,"target_va":5403593968,"va":5377969085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":4020,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2_ptcount
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4048,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":45878,"source_line":162,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":3996,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":45877,"source_line":78,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":741,"record_offset":4096,"parent_record_offset":4072,"ancestor_record_offset":3856,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":741,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":3856,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB338
// 首条位置: {"module":741,"inlinee":45880}
// 编译实例 RVAs: 0x8d4c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25624726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969200,"raw_hex":"e896008701","size":5,"soft_fail":false,"target_va":5403593968,"va":5377969237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":741,"record_offset":4356,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4272,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":45879,"source_line":169,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":741,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":4136,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":4324,"parent_record_offset":4272,"ancestor_record_offset":4136,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":4136,"inlinee":45877,"source_line":78,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve2
// 记录形式: compiled_procedure; instances=1
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":4416,"rva":9260240,"va":5377969360}
// 编译实例 RVAs: 0x8d4cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9516503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969360,"raw_hex":"e8d7359100","size":5,"soft_fail":false,"target_va":5387486016,"va":5377969508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c5700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10422661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969360,"raw_hex":"e885099f00","size":5,"soft_fail":false,"target_va":5388392192,"va":5377969526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25624574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969360,"raw_hex":"e8feff8601","size":5,"soft_fail":false,"target_va":5403593968,"va":5377969389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":741,"record_offset":4624,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4548,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":45879,"source_line":169,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":741,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":4416,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":741,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":4416,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":4592,"parent_record_offset":4548,"ancestor_record_offset":4416,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4568,"parent_record_offset":4548,"ancestor_record_offset":4416,"inlinee":45877,"source_line":78,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":4712,"rva":9260432,"va":5377969552}
// 编译实例 RVAs: 0x8d4d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9515945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969552,"raw_hex":"e8a9339100","size":5,"soft_fail":false,"target_va":5387485664,"va":5377969714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c59e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10423191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969552,"raw_hex":"e8970b9f00","size":5,"soft_fail":false,"target_va":5388392928,"va":5377969732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25624374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969552,"raw_hex":"e836ff8601","size":5,"soft_fail":false,"target_va":5403593968,"va":5377969589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":741,"record_offset":4956,"parent_record_offset":4712,"ancestor_record_offset":4712,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4844,"parent_record_offset":4712,"ancestor_record_offset":4712,"inlinee":45880,"source_line":174,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":741,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":4712,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":741,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4712,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":4920,"parent_record_offset":4868,"ancestor_record_offset":4712,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":4712,"inlinee":45879,"source_line":169,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":4712,"inlinee":45877,"source_line":78,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve2_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":5044,"rva":9260640,"va":5377969760}
// 编译实例 RVAs: 0x8d4e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9516391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969760,"raw_hex":"e867359100","size":5,"soft_fail":false,"target_va":5387486352,"va":5377969956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c5700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10422213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969760,"raw_hex":"e8c5079f00","size":5,"soft_fail":false,"target_va":5388392192,"va":5377969974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25624138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377969760,"raw_hex":"e84afe8601","size":5,"soft_fail":false,"target_va":5403593968,"va":5377969825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":741,"record_offset":5244,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":5216,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":5188,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":45877,"source_line":78,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":741,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":5044,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":741,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":5044,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$0::as_curve2_range::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x188EB
// 首条位置: {"module":3086,"inlinee":100587}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3086,"record_offset":6296,"parent_record_offset":6260,"ancestor_record_offset":6260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3086,"record_offset":6408,"parent_record_offset":6260,"ancestor_record_offset":6260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3086,"record_offset":6620,"parent_record_offset":6260,"ancestor_record_offset":6260,"inlinee":45885,"source_line":27,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3086,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":6260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3086,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":6260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3086,"record_offset":6560,"parent_record_offset":6408,"ancestor_record_offset":6260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3086,"record_offset":6588,"parent_record_offset":6408,"ancestor_record_offset":6260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3086,"record_offset":6348,"parent_record_offset":6296,"ancestor_record_offset":6260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3086,"record_offset":6376,"parent_record_offset":6296,"ancestor_record_offset":6260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3086,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":6260,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3086,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":6260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3086,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":6260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3086,"record_offset":6448,"parent_record_offset":6408,"ancestor_record_offset":6260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3086,"record_offset":6792,"parent_record_offset":6620,"ancestor_record_offset":6260,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3086,"record_offset":6736,"parent_record_offset":6620,"ancestor_record_offset":6260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3086,"record_offset":6676,"parent_record_offset":6648,"ancestor_record_offset":6260,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3086,"record_offset":6704,"parent_record_offset":6648,"ancestor_record_offset":6260,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3086,"record_offset":6648,"parent_record_offset":6620,"ancestor_record_offset":6260,"inlinee":45875,"source_line":47,"archive":"_pdb/A/modules/03086.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::with_expanded_diameter
// 记录形式: compiled_procedure; instances=1
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":5332,"rva":9260896,"va":5377970016}
// 编译实例 RVAs: 0x8d4f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":5468,"rva":9260928,"va":5377970048}
// 编译实例 RVAs: 0x8d4f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":5660,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":5608,"parent_record_offset":5468,"ancestor_record_offset":5468,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":5628,"parent_record_offset":5608,"ancestor_record_offset":5468,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":5680,"parent_record_offset":5660,"ancestor_record_offset":5468,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::signed_distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":5712,"rva":9260976,"va":5377970096}
// 编译实例 RVAs: 0x8d4fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":5848,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":5712,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":5924,"parent_record_offset":5868,"ancestor_record_offset":5712,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":5868,"parent_record_offset":5848,"ancestor_record_offset":5712,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":741,"record_offset":5952,"parent_record_offset":5924,"ancestor_record_offset":5712,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":5996,"rva":9261008,"va":5377970128}
// 编译实例 RVAs: 0x8d4fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$19::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":6120,"parent_record_offset":5996,"ancestor_record_offset":5996,"inlinee":45882,"source_line":1070,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":6140,"parent_record_offset":6120,"ancestor_record_offset":5996,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::lerp
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":6176,"rva":9261056,"va":5377970176}
// 编译实例 RVAs: 0x8d5000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":6432,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":741,"record_offset":6308,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":741,"record_offset":6596,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":45883,"source_line":19,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":6356,"parent_record_offset":6308,"ancestor_record_offset":6176,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":6404,"parent_record_offset":6308,"ancestor_record_offset":6176,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":741,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":6176,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":741,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":6176,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Segment2d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":741,"record_offset":6628,"rva":9261120,"va":5377970240}
// 编译实例 RVAs: 0x8d5040
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
// 完整结构及原始类型记录: circle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::circle::Circle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 24; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<utils::geometry::circle::Circle2d>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::geometry::circle::Circle2d>", "constructor": "ref$", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<utils::geometry::circle::Circle2d> >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<utils::geometry::circle::Circle2d>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::circle::Segment2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::audio::ivy_audio::IvyAudioState>,bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::audio::ivy_audio::IvyAudioState>", "bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::ivy_audio::IvyAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::aabb
// 信息 ID: 30b794fe1b2de60df10c5a97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4a6a -> 0x8d4fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_circle::closure_env$0> @ 0x1046abf -> 0x8d4fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2> @ 0x1047066 -> 0x8d4fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env$0> @ 0x1a9a30f -> 0x8d4fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env$0> @ 0x1dbb8bf -> 0x8d4fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0> @ 0x1dbbc0f -> 0x8d4fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::area
// 信息 ID: 27f47c597af2b9fb85b94754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x61712d -> 0x8d4710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x617516 -> 0x8d4710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::get_building_size_01 @ 0x6181a1 -> 0x8d4710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle @ 0x1803ade -> 0x8d4710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve2
// 信息 ID: 1baa10688c9dca92dcb82e2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x570c7a -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2 @ 0x6b0959 -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height @ 0x6bce17 -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc92e9a -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc94075 -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca047f -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c5d1 -> 0x8d4cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve2_range
// 信息 ID: d77512eb212c96ec6b042fc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xf031e5 -> 0x8d4e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve3
// 信息 ID: cfafaa4598d4dae3fb9a8f04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline::closure$0 @ 0x575bba -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x58603b -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3 @ 0x6b099f -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers @ 0xd2291e -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure$0 @ 0x1802ee8 -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure$0 @ 0x181ea1a -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x181fda5 -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x1acd38f -> 0x8d4d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points2
// 信息 ID: 6d7d7b8e67e02bf2756f247e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x58d6c5 -> 0x8d4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x69e8b1 -> 0x8d4ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xefd140 -> 0x8d4ba0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points3; module=741, inline_record=4272; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2; module=741, inline_record=4548; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=741, inline_record=4868; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points2_ptcount
// 信息 ID: 97ace3ee1804a789b535f7c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x1990242 -> 0x8d4b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x19904f6 -> 0x8d4b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x1990a54 -> 0x8d4b80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points2; module=741, inline_record=4048; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points3
// 信息 ID: 3f3673c8fe19bb402746ac7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x58d67b -> 0x8d4c30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=741, inline_record=4844; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::closest_point_on_perimeter
// 信息 ID: 54683cf071733ae83e3d9354
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x502790 -> 0x8d4b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_birdnests::add_birdnests @ 0x1ad2d27 -> 0x8d4b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c4109b -> 0x8d4b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::contains_point
// 信息 ID: d4a1184063a5927e683ac5fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x546224 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x545ee1 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x545b64 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x5d115d -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x5d110e -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point | country_core::resources::glade_border::GladeBorder::is_pos_inside @ 0x6b09c0 -> 0x8d4f80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0a93 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0ab6 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0ad9 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b02 -> 0x8d4f80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0be9 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c0f -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c31 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c53 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c75 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0x6b7f91 -> 0x8d4f80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_closest_u_from_pos
// 信息 ID: bc825fc5f8db42676509bf63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f0b5e -> 0x8d47a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f0b7b -> 0x8d47a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos @ 0x6b0d10 -> 0x8d47a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_normal_at_u
// 信息 ID: b592819ceeabf9a670acf68c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f10b2 -> 0x8d4900 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u @ 0x6b0cb0 -> 0x8d4900 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0a4e -> 0x8d4900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9eda8 -> 0x8d4900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9eec9 -> 0x8d4900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce3aa2 -> 0x8d4900 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_pos_at_u; module=741, inline_record=652; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=3086, inline_record=1292; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=3086, inline_record=3168; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; module=3086, inline_record=6648; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_pos_at_u
// 信息 ID: b3e8b3fc1badabc0898806b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 43 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b0d -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b33 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b59 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b7f -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0ba5 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0bcb -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0bdb -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c01 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c23 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c45 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c67 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0c89 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u @ 0x6b0cd0 -> 0x8d4730 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e08cc -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e08e4 -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0xc9e6ec -> 0x8d4730 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=3086, inline_record=1264; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=3086, inline_record=3140; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; module=3086, inline_record=6620; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::intersect_circle
// 信息 ID: 9b1029c5c9fb8453f2da52cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7bbc -> 0x8d4a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b8ece -> 0x8d4a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x1c5e7e8 -> 0x8d4a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::intersect_ray
// 信息 ID: dc26b20593c9fe4f34352fad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0x5a6fb1 -> 0x8d4960 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7aa3 -> 0x8d4960 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b8cb3 -> 0x8d4960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x1044b3f -> 0x8d4960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::is_point_inside
// 信息 ID: 2b2df5696ed990b786c85410
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0xc9e289 -> 0x8d4af0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::lerp
// 信息 ID: 589931783aee649c9cc9aa08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xefd53c -> 0x8d5000 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xefd643 -> 0x8d5000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::new
// 信息 ID: e3ac0bcef32760a276787e9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x585fe7 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::draw_garden::draw_garden @ 0x6b2c8e -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y @ 0x6baac8 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e06c6 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca0461 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> > @ 0xe85e77 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> > @ 0xe8627e -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::init_glade_border @ 0xefd765 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xf02dfd -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xf0317f -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::draw_path::draw_path @ 0xf0bded -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain @ 0xf284f6 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_water::draw_water::draw_water @ 0x1409bd8 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x144001f -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x182102f -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0x181fd68 -> 0x8d46f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=741, inline_record=6596; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::perimeter
// 信息 ID: 21720932cf8355299a5634ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x570d21 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter @ 0x6b0cf0 -> 0x8d4950 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0707 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc94044 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0xc9e42a -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9eb48 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca0487 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12> > @ 0xcac48d -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd189d -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd1aac -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0 @ 0x1804345 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x19901d2 -> 0x8d4950 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points2; module=741, inline_record=3996; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points3; module=741, inline_record=4296; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2; module=741, inline_record=4568; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=741, inline_record=4892; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2_range; module=741, inline_record=5188; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::signed_distance
// 信息 ID: e4f9817d824f85b653400e2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9efa2 -> 0x8d4fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0xd0ba91 -> 0x8d4fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0xd0bab2 -> 0x8d4fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::with_expanded_diameter
// 信息 ID: 2452f743496817199fca3fe2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x1a92d0a -> 0x8d4f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Segment2d::new
// 信息 ID: 9e1b44eb7f7bcbda598c43d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x1b35319 -> 0x8d5040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x1b358ee -> 0x8d5040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x1b366ec -> 0x8d5040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x1b36b23 -> 0x8d5040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize
// 信息 ID: a3d31057730d6b57e890bbbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >; module=1326, inline_record=23188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 5dff0767a85959a9158ee65c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor> @ 0x1033a90 -> 0x8d5060 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes
// 信息 ID: 274fa09b2669f9f4cfa449ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=18552; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=18776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: ca72030923ce995ee006cec0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=18392; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=18604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 83e46719aeba895d9cddbc21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=18364; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=18528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 3a60a9e72bb6a16ddb219c2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=9340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: f5bd84d219867bf61e2e72f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::circle::_::impl$0::deserialize::__Visitor> @ 0x1033aa0 -> 0x8d5080 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: a0083564bae7582dbfcb9c7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=8544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 16911114ab329db9f9f6464f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=10700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 405b369a9abc3302968f12f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::circle::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d> @ 0x14d6ca -> 0x1a6800 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d> @ 0x1093cca -> 0x1033fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$0::as_curve2_range::closure$0
// 信息 ID: 616bcbddc99496c4be7484e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::circle::impl$0::as_curve2_range", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> > > >; module=3086, inline_record=6260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
// 信息 ID: 8d24b410b888ec79efe61053
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::circle::impl$0::as_points2_ptcount", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 13 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=1321, inline_record=1028; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=1321, inline_record=49832; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::glade_border::GladeBorder::is_shape_inside; module=1350, inline_record=16392; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> > > >; module=3086, inline_record=1024; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > > >; module=3086, inline_record=2896; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=3215, inline_record=3104; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=3215, inline_record=4312; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=3215, inline_record=4844; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$3::fmt
// 信息 ID: 6d71cfd8df88a9cf2f76d6ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<utils::geometry::circle::Circle2d>; module=1399, inline_record=2736; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<utils::geometry::circle::Circle2d>; module=4466, inline_record=392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$6::eq
// 信息 ID: 9df00ad1151d86cabc4a4847
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2269, inline_record=15928; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3194, inline_record=1976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
