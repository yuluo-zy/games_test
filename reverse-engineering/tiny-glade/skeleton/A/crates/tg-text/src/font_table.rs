// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-text/src/font_table.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f574bf591a470045720fef15e96d70042a98770478f4ffae939d945490e3e9a2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: font_table.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 514, 515, 1015, 1021, 1033, 1502, 2447, 2449, 2468, 3677, 3678
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 5, 16, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145F4
// 首条位置: {"module":2447,"inlinee":83444}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":74772,"parent_record_offset":74748,"ancestor_record_offset":74748,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":44592,"parent_record_offset":44568,"ancestor_record_offset":44568,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 5, 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145F9
// 首条位置: {"module":2447,"inlinee":83449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":69608,"parent_record_offset":69584,"ancestor_record_offset":69584,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":69632,"parent_record_offset":69608,"ancestor_record_offset":69584,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":69656,"parent_record_offset":69632,"ancestor_record_offset":69584,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":69680,"parent_record_offset":69656,"ancestor_record_offset":69584,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 5, 16, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145E5
// 首条位置: {"module":2447,"inlinee":83429}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":11944,"parent_record_offset":11916,"ancestor_record_offset":11916,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":69632,"parent_record_offset":69608,"ancestor_record_offset":69460,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":11980,"parent_record_offset":11944,"ancestor_record_offset":11916,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":12044,"parent_record_offset":11944,"ancestor_record_offset":11916,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":69656,"parent_record_offset":69632,"ancestor_record_offset":69460,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":69680,"parent_record_offset":69656,"ancestor_record_offset":69460,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":69608,"parent_record_offset":69584,"ancestor_record_offset":69460,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":12016,"parent_record_offset":11944,"ancestor_record_offset":11916,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2447,"record_offset":69584,"parent_record_offset":69560,"ancestor_record_offset":69460,"inlinee":83449,"source_line":5,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5, 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145F8
// 首条位置: {"module":2447,"inlinee":83448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2447,"record_offset":69884,"parent_record_offset":68716,"ancestor_record_offset":68716,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2447,"record_offset":70704,"parent_record_offset":68716,"ancestor_record_offset":68716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":69784,"parent_record_offset":68716,"ancestor_record_offset":68716,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":70844,"parent_record_offset":68716,"ancestor_record_offset":68716,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":68764,"parent_record_offset":68716,"ancestor_record_offset":68716,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":69820,"parent_record_offset":68716,"ancestor_record_offset":68716,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2447,"record_offset":88044,"parent_record_offset":88020,"ancestor_record_offset":86032,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2447,"record_offset":88020,"parent_record_offset":87956,"ancestor_record_offset":86032,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2447,"record_offset":87988,"parent_record_offset":87956,"ancestor_record_offset":86032,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2447,"record_offset":87956,"parent_record_offset":87928,"ancestor_record_offset":86032,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":69632,"parent_record_offset":69608,"ancestor_record_offset":68716,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2447,"record_offset":68956,"parent_record_offset":68796,"ancestor_record_offset":68716,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2447,"record_offset":68996,"parent_record_offset":68956,"ancestor_record_offset":68716,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2447,"record_offset":69132,"parent_record_offset":68796,"ancestor_record_offset":68716,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2447,"record_offset":70428,"parent_record_offset":69948,"ancestor_record_offset":68716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":68916,"parent_record_offset":68796,"ancestor_record_offset":68716,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":70400,"parent_record_offset":69948,"ancestor_record_offset":68716,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":69656,"parent_record_offset":69632,"ancestor_record_offset":68716,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":69680,"parent_record_offset":69656,"ancestor_record_offset":68716,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":69608,"parent_record_offset":69584,"ancestor_record_offset":68716,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2447,"record_offset":70500,"parent_record_offset":70476,"ancestor_record_offset":68716,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":2447,"record_offset":69036,"parent_record_offset":68796,"ancestor_record_offset":68716,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_f32
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":87668,"parent_record_offset":87644,"ancestor_record_offset":86032,"inlinee":9493,"source_line":424,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":69488,"parent_record_offset":69460,"ancestor_record_offset":68716,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":68796,"parent_record_offset":68764,"ancestor_record_offset":68716,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":69948,"parent_record_offset":69912,"ancestor_record_offset":68716,"inlinee":10245,"source_line":857,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_f32
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":87644,"parent_record_offset":87620,"ancestor_record_offset":86032,"inlinee":9513,"source_line":118,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":70228,"parent_record_offset":70204,"ancestor_record_offset":68716,"inlinee":83450,"source_line":214,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":69560,"parent_record_offset":69488,"ancestor_record_offset":68716,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":69076,"parent_record_offset":68796,"ancestor_record_offset":68716,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":2447,"record_offset":70204,"parent_record_offset":70180,"ancestor_record_offset":68716,"inlinee":24937,"source_line":1505,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":2447,"record_offset":87620,"parent_record_offset":87596,"ancestor_record_offset":86032,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2447,"record_offset":69584,"parent_record_offset":69560,"ancestor_record_offset":68716,"inlinee":83449,"source_line":5,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2447,"record_offset":69100,"parent_record_offset":69076,"ancestor_record_offset":68716,"inlinee":83429,"source_line":5,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::default_scale
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":515,"record_offset":244,"rva":7337456,"va":5376046576}
// 编译实例 RVAs: 0x6ff5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC714
// 首条位置: {"module":1021,"inlinee":50964}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1021,"record_offset":24104,"parent_record_offset":24084,"ancestor_record_offset":24084,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1021,"record_offset":24356,"parent_record_offset":24332,"ancestor_record_offset":24084,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1021,"record_offset":24408,"parent_record_offset":24384,"ancestor_record_offset":24084,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1021,"record_offset":24384,"parent_record_offset":24332,"ancestor_record_offset":24084,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1021,"record_offset":24432,"parent_record_offset":24408,"ancestor_record_offset":24084,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1021,"record_offset":24292,"parent_record_offset":24268,"ancestor_record_offset":24084,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1021,"record_offset":24456,"parent_record_offset":24432,"ancestor_record_offset":24084,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1021,"record_offset":24144,"parent_record_offset":24124,"ancestor_record_offset":24084,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1021,"record_offset":24220,"parent_record_offset":24192,"ancestor_record_offset":24084,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1021,"record_offset":24332,"parent_record_offset":24192,"ancestor_record_offset":24084,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1021,"record_offset":24124,"parent_record_offset":24104,"ancestor_record_offset":24084,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1021,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":24084,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1021,"record_offset":24268,"parent_record_offset":24244,"ancestor_record_offset":24084,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1021,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":24084,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1021,"record_offset":24168,"parent_record_offset":24144,"ancestor_record_offset":24084,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::FontTable::get
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F90
// 首条位置: {"module":514,"inlinee":28560}
// 编译实例 RVAs: 0x6ff600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a551b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: font_table.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3496836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376046592,"raw_hex":"e8845b3500","size":5,"soft_fail":false,"target_va":5379543472,"va":5376046631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":514,"record_offset":22588,"parent_record_offset":22564,"ancestor_record_offset":22564,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":514,"record_offset":23368,"parent_record_offset":23344,"ancestor_record_offset":22564,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":514,"record_offset":22836,"parent_record_offset":22812,"ancestor_record_offset":22564,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":514,"record_offset":22920,"parent_record_offset":22896,"ancestor_record_offset":22564,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":514,"record_offset":22860,"parent_record_offset":22836,"ancestor_record_offset":22564,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":514,"record_offset":23032,"parent_record_offset":23008,"ancestor_record_offset":22564,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":514,"record_offset":23280,"parent_record_offset":23256,"ancestor_record_offset":22564,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":514,"record_offset":23172,"parent_record_offset":23148,"ancestor_record_offset":22564,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":514,"record_offset":23344,"parent_record_offset":23320,"ancestor_record_offset":22564,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":514,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":22564,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":514,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":22564,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":514,"record_offset":23320,"parent_record_offset":23100,"ancestor_record_offset":22564,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":514,"record_offset":22664,"parent_record_offset":22636,"ancestor_record_offset":22564,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":23232,"parent_record_offset":23208,"ancestor_record_offset":22564,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":23208,"parent_record_offset":23100,"ancestor_record_offset":22564,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":23148,"parent_record_offset":23124,"ancestor_record_offset":22564,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":23528,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":23124,"parent_record_offset":23100,"ancestor_record_offset":22564,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":22564,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":22692,"parent_record_offset":22636,"ancestor_record_offset":22564,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":22564,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":514,"record_offset":22980,"parent_record_offset":22952,"ancestor_record_offset":22564,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":514,"record_offset":23008,"parent_record_offset":22980,"ancestor_record_offset":22564,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":514,"record_offset":23068,"parent_record_offset":22952,"ancestor_record_offset":22564,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":514,"record_offset":22952,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":22784,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":514,"record_offset":23100,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":514,"record_offset":22812,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":514,"record_offset":22896,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":514,"record_offset":23436,"parent_record_offset":22740,"ancestor_record_offset":22564,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::impl$6::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":514,"record_offset":23392,"parent_record_offset":23368,"ancestor_record_offset":22564,"inlinee":28558,"source_line":27,"archive":"_pdb/A/modules/00514.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3677,"record_offset":244,"rva":28843840,"va":5397552960}
// 编译实例 RVAs: 0x1b81f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5408704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397552960,"raw_hex":"e8c0875200","size":5,"soft_fail":false,"target_va":5402961856,"va":5397553147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3677,"record_offset":420,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03677.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3677,"record_offset":440,"parent_record_offset":420,"ancestor_record_offset":244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03677.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3677,"record_offset":460,"parent_record_offset":440,"ancestor_record_offset":244,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03677.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3677,"record_offset":480,"parent_record_offset":460,"ancestor_record_offset":244,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145F7
// 首条位置: {"module":2447,"inlinee":83447}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":60552,"parent_record_offset":60500,"ancestor_record_offset":60500,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":60524,"parent_record_offset":60500,"ancestor_record_offset":60500,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F8E
// 首条位置: {"module":514,"inlinee":28558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$8::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6BE
// 首条位置: {"module":1015,"inlinee":50878}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1015,"record_offset":6756,"parent_record_offset":6736,"ancestor_record_offset":6736,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/01015.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1015,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":6736,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/01015.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1015,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":6736,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01015.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1015,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":6736,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1015,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":6736,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01015.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1015,"record_offset":6860,"parent_record_offset":6804,"ancestor_record_offset":6736,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::FontTableEntry::scaled
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F92
// 首条位置: {"module":514,"inlinee":28562}
// 编译实例 RVAs: 0x6ff6f0
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
// 完整结构及原始类型记录: font_table.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontFamily
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "constructor": "tuple$", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tg_text::font_table::FontFamily>", "constructor": "ref$", "arguments": ["tg_text::font_table::FontFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<tg_text::font_table::FontFamily>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontTable
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<tg_text::font_table::FontTable,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["tg_text::font_table::FontTable", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$52::default<tg_text::font_table::FontTable>", "constructor": "alloc::sync::impl$52::default", "arguments": ["tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<tg_text::font_table::FontTable>", "constructor": "core::marker::PhantomData", "arguments": ["tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<tg_text::font_table::FontTable>", "tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontTableEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "constructor": "tuple$", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: font_table.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: font_table.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>

// ------------------------------------------------------------------
// 函数: tg_text::font_table::FontTable::get
// 信息 ID: 51023e8dae517d7a433065ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x27846a -> 0x6ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0xa5c21a -> 0x6ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::new<f32> @ 0xa5c2ce -> 0x6ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x1dc74fa -> 0x6ff600 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=514, inline_record=22564; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::FontTableEntry::scaled
// 信息 ID: 6b85e8226bd350f6ebf31b90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x278481 -> 0x6ff6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0xa5c231 -> 0x6ff6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::new<f32> @ 0xa5c2ed -> 0x6ff6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x1dc7511 -> 0x6ff6f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=514, inline_record=23588; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize
// 信息 ID: 641d6d72df298c5e1b9c8797
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >; module=2447, inline_record=44232; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >; module=2447, inline_record=44568; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<toolbar::mei::config::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=74748; 不是独立调用边。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>; module=2449, inline_record=552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: d19508d5260963b06889f3dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=69584; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=86932; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3678, inline_record=6244; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3678, inline_record=6520; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3678, inline_record=7240; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3678, inline_record=7660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 3dab086a7cdcd8faa626b49a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::font_table::_::impl$0::deserialize::__Field> > @ 0x15fe997 -> 0x1b81f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: fb96fcded10ad5d99d23c344
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::font_table::_::impl$0::deserialize::__Field> >; module=2447, inline_record=11916; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=69100; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=69308; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=69460; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=86428; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=86636; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=86792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: d9996d7de2439d78900127bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=60500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 4f5ea8a6419bca8e7fb195b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=68716; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=86032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::default_scale
// 信息 ID: d47e79bff413ba1aa06ec763
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor> @ 0x16107c5 -> 0x6ff5f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$2::default
// 信息 ID: 67127922a9ca5dbf9c0a6423
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$52::default<tg_text::font_table::FontTable>; module=1021, inline_record=24084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$6::eq
// 信息 ID: edf30603f3c2ad7f890291d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=514, inline_record=22448; 不是独立调用边。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=514, inline_record=23392; 不是独立调用边。
// [PDB 内联宿主] tg_text::font_table::FontTable::get; module=515, inline_record=1280; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>; module=1033, inline_record=3048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$8::hash
// 信息 ID: b8baed07cd388299a79c804c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >; module=1015, inline_record=6736; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >; module=1502, inline_record=568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
