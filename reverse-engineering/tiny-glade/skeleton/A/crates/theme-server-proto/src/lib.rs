// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/theme-server-proto/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 808a90f13a37512f7dc630563b3d3991ded7a802af23792732356ff5a7fd072e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 703, 1837, 3138
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use theme_server_proto; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: theme_server_proto::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 3, 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB0FD
// 首条位置: {"module":703,"inlinee":45309}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_server_proto::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 3, 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB10A
// 首条位置: {"module":703,"inlinee":45322}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":30580,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":30580,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":30652,"parent_record_offset":30628,"ancestor_record_offset":30580,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":30676,"parent_record_offset":30652,"ancestor_record_offset":30580,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_server_proto::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 3, 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB108
// 首条位置: {"module":703,"inlinee":45320}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":30440,"parent_record_offset":30416,"ancestor_record_offset":30416,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":30552,"parent_record_offset":30464,"ancestor_record_offset":30416,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":30416,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":30652,"parent_record_offset":30628,"ancestor_record_offset":30416,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":30676,"parent_record_offset":30652,"ancestor_record_offset":30416,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":30416,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30496,"parent_record_offset":30464,"ancestor_record_offset":30416,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":30416,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":30520,"parent_record_offset":30496,"ancestor_record_offset":30416,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] theme_server_proto::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":703,"record_offset":30580,"parent_record_offset":30464,"ancestor_record_offset":30416,"inlinee":45322,"source_line":3,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_server_proto::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=2
// 源码行: 3, 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3138,"record_offset":256,"rva":26668096,"va":5395377216}
// 编译实例 RVAs: 0x196ec40, 0x196ec60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17793317","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5395377216,"raw_hex":"e9db7ef0fe","size":5,"soft_fail":false,"target_va":5377583920,"va":5395377232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_server_proto::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 3, 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB106
// 首条位置: {"module":703,"inlinee":45318}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":703,"record_offset":30748,"parent_record_offset":29824,"ancestor_record_offset":29824,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":31208,"parent_record_offset":29824,"ancestor_record_offset":29824,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":703,"record_offset":31176,"parent_record_offset":29824,"ancestor_record_offset":29824,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":703,"record_offset":31740,"parent_record_offset":29824,"ancestor_record_offset":29824,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":29868,"parent_record_offset":29824,"ancestor_record_offset":29824,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":30776,"parent_record_offset":29824,"ancestor_record_offset":29824,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":29824,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":31472,"parent_record_offset":31408,"ancestor_record_offset":29824,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":31440,"parent_record_offset":31408,"ancestor_record_offset":29824,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":31408,"parent_record_offset":31376,"ancestor_record_offset":29824,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":30552,"parent_record_offset":30464,"ancestor_record_offset":29824,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":31236,"parent_record_offset":31208,"ancestor_record_offset":29824,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":29824,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":31636,"parent_record_offset":31608,"ancestor_record_offset":29824,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":31264,"parent_record_offset":31236,"ancestor_record_offset":29824,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":30652,"parent_record_offset":30628,"ancestor_record_offset":29824,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":30676,"parent_record_offset":30652,"ancestor_record_offset":29824,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":29824,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":703,"record_offset":31812,"parent_record_offset":31788,"ancestor_record_offset":29824,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":44536,"parent_record_offset":44512,"ancestor_record_offset":41712,"inlinee":45328,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":31788,"parent_record_offset":31764,"ancestor_record_offset":29824,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":44424,"parent_record_offset":44400,"ancestor_record_offset":41712,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":30392,"parent_record_offset":29916,"ancestor_record_offset":29824,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":31136,"parent_record_offset":31112,"ancestor_record_offset":29824,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$57::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":44512,"parent_record_offset":44488,"ancestor_record_offset":41712,"inlinee":45314,"source_line":134,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":44084,"parent_record_offset":44060,"ancestor_record_offset":41712,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30116,"parent_record_offset":30000,"ancestor_record_offset":29824,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30848,"parent_record_offset":30808,"ancestor_record_offset":29824,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30000,"parent_record_offset":29916,"ancestor_record_offset":29824,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30040,"parent_record_offset":30000,"ancestor_record_offset":29824,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":29916,"parent_record_offset":29868,"ancestor_record_offset":29824,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30808,"parent_record_offset":30776,"ancestor_record_offset":29824,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":29824,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":30440,"parent_record_offset":30416,"ancestor_record_offset":29824,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":30140,"parent_record_offset":30116,"ancestor_record_offset":29824,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":30072,"parent_record_offset":30040,"ancestor_record_offset":29824,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] theme_server_proto::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":703,"record_offset":30580,"parent_record_offset":30464,"ancestor_record_offset":29824,"inlinee":45322,"source_line":3,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] theme_server_proto::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":703,"record_offset":30416,"parent_record_offset":30392,"ancestor_record_offset":29824,"inlinee":45320,"source_line":3,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_server_proto::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 3, 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB10B
// 首条位置: {"module":703,"inlinee":45323}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":703,"record_offset":44976,"parent_record_offset":44940,"ancestor_record_offset":44940,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":31944,"parent_record_offset":31912,"ancestor_record_offset":31912,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":32188,"parent_record_offset":31976,"ancestor_record_offset":31912,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":32212,"parent_record_offset":32188,"ancestor_record_offset":31912,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":32128,"parent_record_offset":32020,"ancestor_record_offset":31912,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":32020,"parent_record_offset":31976,"ancestor_record_offset":31912,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":32056,"parent_record_offset":32020,"ancestor_record_offset":31912,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":31976,"parent_record_offset":31944,"ancestor_record_offset":31912,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":32152,"parent_record_offset":32128,"ancestor_record_offset":31912,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":32088,"parent_record_offset":32056,"ancestor_record_offset":31912,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_server_proto::ActiveEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["theme_server_proto::ActiveEvent", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<theme_server_proto::ActiveEvent,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["theme_server_proto::ActiveEvent", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_server_proto::ActiveEvent,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["theme_server_proto::ActiveEvent", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_server_proto::ActiveEvent,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_server_proto::ActiveEvent,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_server_proto::ActiveEvent,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_server_proto::ActiveEvent,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<theme_server_proto::ActiveEvent,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["theme_server_proto::ActiveEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<serde::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent> >", "constructor": "core::ptr::drop_in_place", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<theme_server_proto::ActiveEvent>", "constructor": "core::marker::PhantomData", "arguments": ["theme_server_proto::ActiveEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$9::next_element_seed<serde_json::read::SliceRead,core::marker::PhantomData<theme_server_proto::ActiveEvent> >", "constructor": "serde_json::de::impl$9::next_element_seed", "arguments": ["serde_json::read::SliceRead", "core::marker::PhantomData<theme_server_proto::ActiveEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::SliceRead", "serde::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<theme_server_proto::ActiveEvent>", "constructor": "core::slice::iter::Iter", "arguments": ["theme_server_proto::ActiveEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<theme_server_proto::ActiveEvent>", "theme_fetcher::impl$0::fetch_events::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<theme_fetcher::MaybePendingActiveEvent,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<theme_server_proto::ActiveEvent,serde_json::de::SeqAccess<serde_json::read::SliceRead> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["theme_server_proto::ActiveEvent", "serde_json::de::SeqAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::slice::iter::Iter<theme_server_proto::ActiveEvent>", "theme_fetcher::impl$0::fetch_events::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_server_proto::EventsResponse
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_server_proto::EventsResponse,alloc::string::String>", "constructor": "core::result::Result", "arguments": ["theme_server_proto::EventsResponse", "alloc::string::String"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_server_proto::EventsResponse,alloc::string::String> >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_server_proto::EventsResponse,alloc::string::String>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_server_proto::EventsResponse,alloc::string::String> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_server_proto::EventsResponse,alloc::string::String> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_server_proto::EventsResponse>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_server_proto::EventsResponse"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<theme_server_proto::EventsResponse>", "constructor": "serde_json::de::from_slice", "arguments": ["theme_server_proto::EventsResponse"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_server_proto::EventsResponse,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["theme_server_proto::EventsResponse", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_server_proto::EventsResponse,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_server_proto::EventsResponse,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_server_proto::EventsResponse,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_server_proto::EventsResponse,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: theme_server_proto::_::impl$0::deserialize
// 信息 ID: f7354fba95b4c957c410b52f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<theme_server_proto::EventsResponse>; module=703, inline_record=2324; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$9::next_element_seed<serde_json::read::SliceRead,core::marker::PhantomData<theme_server_proto::ActiveEvent> >; module=703, inline_record=25316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_server_proto::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: c09314a677368740c4e7680e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=30580; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=42508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_server_proto::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 5d5fca4f2cc4107b90084c96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=30416; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=42332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_server_proto::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 5b32004af0f9440ed30aee9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<theme_server_proto::_::impl$0::deserialize::__Visitor> @ 0x1296b40 -> 0x196ec40 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$1::fmt<theme_server_proto::_::impl$0::deserialize::__Visitor> @ 0x1296b50 -> 0x196ec60 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_server_proto::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 578acb35285ed703644e41eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=29824; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=41712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_server_proto::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: cba927c931b24217f7ce756a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=31912; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>; module=703, inline_record=44940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
