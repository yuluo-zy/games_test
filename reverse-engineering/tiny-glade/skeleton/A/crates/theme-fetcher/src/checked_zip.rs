// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/theme-fetcher/src/checked_zip.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4f8bc0cf2763140d7bf048e8ba1e982ca2ba1c0fd9cfca8c4fa6ffaf56171be1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: checked_zip.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 682, 700, 703, 1792
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"PDB_registry_path_only","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"PDB_registry_path_only","version":"0.5.8"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use theme_fetcher; // namespace candidate
//   package identities: []
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"PDB_registry_path_only","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB0F4
// 首条位置: {"module":703,"inlinee":45300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB0F5
// 首条位置: {"module":703,"inlinee":45301}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":35224,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":35276,"parent_record_offset":35252,"ancestor_record_offset":35224,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":35224,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":35224,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB10E
// 首条位置: {"module":703,"inlinee":45326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":35072,"parent_record_offset":35044,"ancestor_record_offset":35044,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":35196,"parent_record_offset":35100,"ancestor_record_offset":35044,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":35276,"parent_record_offset":35252,"ancestor_record_offset":35044,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":35044,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":35044,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":35044,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":35140,"parent_record_offset":35100,"ancestor_record_offset":35044,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":35100,"parent_record_offset":35072,"ancestor_record_offset":35044,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":35164,"parent_record_offset":35140,"ancestor_record_offset":35044,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":703,"record_offset":35224,"parent_record_offset":35100,"ancestor_record_offset":35044,"inlinee":45301,"source_line":9,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAD81
// 首条位置: {"module":700,"inlinee":44417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB10C
// 首条位置: {"module":703,"inlinee":45324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":703,"record_offset":35508,"parent_record_offset":34436,"ancestor_record_offset":34436,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":36460,"parent_record_offset":34436,"ancestor_record_offset":34436,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":703,"record_offset":36396,"parent_record_offset":34436,"ancestor_record_offset":34436,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":703,"record_offset":36680,"parent_record_offset":34436,"ancestor_record_offset":34436,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":34488,"parent_record_offset":34436,"ancestor_record_offset":34436,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":35536,"parent_record_offset":34436,"ancestor_record_offset":34436,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":36632,"parent_record_offset":36608,"ancestor_record_offset":34436,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":36608,"parent_record_offset":36544,"ancestor_record_offset":34436,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":36576,"parent_record_offset":36544,"ancestor_record_offset":34436,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":36544,"parent_record_offset":36516,"ancestor_record_offset":34436,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":35196,"parent_record_offset":35100,"ancestor_record_offset":34436,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":36928,"parent_record_offset":36904,"ancestor_record_offset":34436,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":35276,"parent_record_offset":35252,"ancestor_record_offset":34436,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":37524,"parent_record_offset":37496,"ancestor_record_offset":34436,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":36488,"parent_record_offset":36460,"ancestor_record_offset":34436,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":34436,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":34436,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":34436,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":703,"record_offset":36752,"parent_record_offset":36728,"ancestor_record_offset":34436,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":36840,"parent_record_offset":36816,"ancestor_record_offset":34436,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":36728,"parent_record_offset":36704,"ancestor_record_offset":34436,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":35016,"parent_record_offset":34536,"ancestor_record_offset":34436,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":35912,"parent_record_offset":35888,"ancestor_record_offset":34436,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":36356,"parent_record_offset":36332,"ancestor_record_offset":34436,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":34740,"parent_record_offset":34624,"ancestor_record_offset":34436,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":35620,"parent_record_offset":35576,"ancestor_record_offset":34436,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":34624,"parent_record_offset":34536,"ancestor_record_offset":34436,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":34664,"parent_record_offset":34624,"ancestor_record_offset":34436,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":34536,"parent_record_offset":34488,"ancestor_record_offset":34436,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":35576,"parent_record_offset":35536,"ancestor_record_offset":34436,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":35100,"parent_record_offset":35072,"ancestor_record_offset":34436,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":703,"record_offset":35072,"parent_record_offset":35044,"ancestor_record_offset":34436,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":34764,"parent_record_offset":34740,"ancestor_record_offset":34436,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":34696,"parent_record_offset":34664,"ancestor_record_offset":34436,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":703,"record_offset":35224,"parent_record_offset":35100,"ancestor_record_offset":34436,"inlinee":45301,"source_line":9,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":703,"record_offset":35044,"parent_record_offset":35016,"ancestor_record_offset":34436,"inlinee":45326,"source_line":9,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB10F
// 首条位置: {"module":703,"inlinee":45327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":38232,"parent_record_offset":37856,"ancestor_record_offset":37856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":703,"record_offset":38204,"parent_record_offset":37856,"ancestor_record_offset":37856,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":37900,"parent_record_offset":37856,"ancestor_record_offset":37856,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":38508,"parent_record_offset":38484,"ancestor_record_offset":37856,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":38484,"parent_record_offset":38420,"ancestor_record_offset":37856,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38452,"parent_record_offset":38420,"ancestor_record_offset":37856,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38420,"parent_record_offset":38388,"ancestor_record_offset":37856,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38256,"parent_record_offset":38232,"ancestor_record_offset":37856,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":38852,"parent_record_offset":38824,"ancestor_record_offset":37856,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":38280,"parent_record_offset":38256,"ancestor_record_offset":37856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":703,"record_offset":38140,"parent_record_offset":37928,"ancestor_record_offset":37856,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":703,"record_offset":38164,"parent_record_offset":38140,"ancestor_record_offset":37856,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":38080,"parent_record_offset":37972,"ancestor_record_offset":37856,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":37972,"parent_record_offset":37928,"ancestor_record_offset":37856,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":38008,"parent_record_offset":37972,"ancestor_record_offset":37856,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":703,"record_offset":37928,"parent_record_offset":37900,"ancestor_record_offset":37856,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":37856,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":703,"record_offset":38040,"parent_record_offset":38008,"ancestor_record_offset":37856,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB0F2
// 首条位置: {"module":700,"inlinee":45298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9836,"parent_record_offset":9804,"ancestor_record_offset":9804,"inlinee":5677,"source_line":409,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10152,"parent_record_offset":9804,"ancestor_record_offset":9804,"inlinee":5699,"source_line":774,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10092,"parent_record_offset":9804,"ancestor_record_offset":9804,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":9804,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":9804,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":700,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":9804,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":700,"record_offset":10032,"parent_record_offset":10004,"ancestor_record_offset":9804,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":9804,"inlinee":5679,"source_line":1849,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10208,"parent_record_offset":10180,"ancestor_record_offset":9804,"inlinee":5680,"source_line":1859,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":9804,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":9804,"inlinee":5700,"source_line":720,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":700,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":9804,"inlinee":4269,"source_line":417,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":700,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":9804,"inlinee":4267,"source_line":77,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":700,"record_offset":2408,"rva":9122960,"va":5377832080}
// 编译实例 RVAs: 0x8b3490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$u8$GT$$u20$as$u20$std..io..copy..BufferedWriterSpec$GT$::buffer_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-421434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8c691f9ff","size":5,"soft_fail":false,"target_va":5377411456,"va":5377832885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8c2d2ffff","size":5,"soft_fail":false,"target_va":5377821664,"va":5377833241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dfbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8209522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e88ebb82ff","size":5,"soft_fail":false,"target_va":5369625520,"va":5377835037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6274344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8d842a0ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5377836115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5596216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8c89baaff","size":5,"soft_fail":false,"target_va":5372239520,"va":5377835731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5518556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e824cbabff","size":5,"soft_fail":false,"target_va":5372314208,"va":5377832759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5083140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8fc6fb2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5377832831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5083843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e83d6db2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377833550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=5
//     target_aliases=28 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2661484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e89463d7ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5377832535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-427978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e83678f9ff","size":5,"soft_fail":false,"target_va":5377405008,"va":5377832981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14124b270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10056180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8f4719900","size":5,"soft_fail":false,"target_va":5387891312,"va":5377835127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25760945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8b1148901","size":5,"soft_fail":false,"target_va":5403593360,"va":5377832410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t25142804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e814a67f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5377835335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6525794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e89e6c9cff","size":5,"soft_fail":false,"target_va":5371307664,"va":5377833453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-308227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8fd4bfbff","size":5,"soft_fail":false,"target_va":5377527120,"va":5377835342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-217345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8ffaefcff","size":5,"soft_fail":false,"target_va":5377616960,"va":5377834300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::haa3d630f77335115 (.llvm.923038179807553715)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t25148001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e861ba7f01","size":5,"soft_fail":false,"target_va":5402981936,"va":5377833930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t25149050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e87abe7f01","size":5,"soft_fail":false,"target_va":5402983040,"va":5377833985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e805eaffff","size":5,"soft_fail":false,"target_va":5377828752,"va":5377834374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e860e4ffff","size":5,"soft_fail":false,"target_va":5377826256,"va":5377833323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e882e0ffff","size":5,"soft_fail":false,"target_va":5377825072,"va":5377833129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25143555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e803a97f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5377835320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..checked_zip..FolderChecksumInfo$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-205925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e89bdbfcff","size":5,"soft_fail":false,"target_va":5377629296,"va":5377835216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..ZipFile$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-211280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8b0c6fcff","size":5,"soft_fail":false,"target_va":5377622320,"va":5377833595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25144817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8f1ad7f01","size":5,"soft_fail":false,"target_va":5402980128,"va":5377835306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::ser::SerializeMap::serialize_entry
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10356367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e88f069e00","size":5,"soft_fail":false,"target_va":5388191056,"va":5377834684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-438283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8f54ff9ff","size":5,"soft_fail":false,"target_va":5377395552,"va":5377833830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-423802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e88688f9ff","size":5,"soft_fail":false,"target_va":5377408752,"va":5377832549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-425745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8ef80f9ff","size":5,"soft_fail":false,"target_va":5377408016,"va":5377833756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-426647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8697df9ff","size":5,"soft_fail":false,"target_va":5377406304,"va":5377832946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10357849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8590c9e00","size":5,"soft_fail":false,"target_va":5388192960,"va":5377835106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::default_read_to_end
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10355764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e834049e00","size":5,"soft_fail":false,"target_va":5388188672,"va":5377832903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-392725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8eb01faff","size":5,"soft_fail":false,"target_va":5377439776,"va":5377832496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-395652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e87cf6f9ff","size":5,"soft_fail":false,"target_va":5377436944,"va":5377832591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-359361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e83f84faff","size":5,"soft_fail":false,"target_va":5377473264,"va":5377832620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::ZipFile::enclosed_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10053014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e896659900","size":5,"soft_fail":false,"target_va":5387885472,"va":5377832453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::ZipFile::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10052915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e833659900","size":5,"soft_fail":false,"target_va":5387885424,"va":5377832504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::ZipFile::size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10052718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e86e649900","size":5,"soft_fail":false,"target_va":5387885520,"va":5377832797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::_$LT$impl$u20$zip..read..zip_archive..ZipArchive$LT$R$GT$$GT$::by_index
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377832080,"raw_hex":"e8e7fcffff","size":5,"soft_fail":false,"target_va":5377831552,"va":5377832340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*33449602(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5377832080,"raw_hex":"ff158266fe01","size":6,"soft_fail":false,"va":5377834952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":700,"record_offset":4816,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":700,"record_offset":5392,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":5744,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":3868,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":2688,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":700,"record_offset":7316,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/blake3-1.5.3/src/lib.rs
//     evidence={"module":700,"record_offset":2744,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":44410,"source_line":1010,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":700,"record_offset":2856,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":700,"record_offset":2984,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":700,"record_offset":4780,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":700,"record_offset":5884,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":700,"record_offset":2952,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":700,"record_offset":7424,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":700,"record_offset":3040,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":12644,"source_line":1213,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9380,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":6388,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":45296,"source_line":432,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":3236,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":4268,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::io::copy::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\copy.rs
//     evidence={"module":700,"record_offset":4152,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":24422,"source_line":61,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":3660,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":3012,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":4688,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":3520,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zip-0.6.6/src/read.rs
//     evidence={"module":700,"record_offset":2800,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":45294,"source_line":483,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":700,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":2408,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":700,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":2408,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":700,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":2408,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":700,"record_offset":3988,"parent_record_offset":3952,"ancestor_record_offset":2408,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":700,"record_offset":5668,"parent_record_offset":5612,"ancestor_record_offset":2408,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":700,"record_offset":5640,"parent_record_offset":5612,"ancestor_record_offset":2408,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":700,"record_offset":5820,"parent_record_offset":5788,"ancestor_record_offset":2408,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":700,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":2408,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":700,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":2408,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":700,"record_offset":4108,"parent_record_offset":3924,"ancestor_record_offset":2408,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":700,"record_offset":5612,"parent_record_offset":5584,"ancestor_record_offset":2408,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":700,"record_offset":4996,"parent_record_offset":4960,"ancestor_record_offset":2408,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":700,"record_offset":4960,"parent_record_offset":4924,"ancestor_record_offset":2408,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":700,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":2408,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":700,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":2408,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":700,"record_offset":4852,"parent_record_offset":4816,"ancestor_record_offset":2408,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":700,"record_offset":5444,"parent_record_offset":5392,"ancestor_record_offset":2408,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":700,"record_offset":8868,"parent_record_offset":8828,"ancestor_record_offset":2408,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":700,"record_offset":8828,"parent_record_offset":8800,"ancestor_record_offset":2408,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":2408,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":5788,"parent_record_offset":5744,"ancestor_record_offset":2408,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":2408,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2408,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2408,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":9440,"parent_record_offset":9380,"ancestor_record_offset":2408,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":3896,"parent_record_offset":3868,"ancestor_record_offset":2408,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":2408,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$29::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":700,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":2408,"inlinee":5330,"source_line":3244,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":700,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":2408,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":700,"record_offset":7384,"parent_record_offset":7348,"ancestor_record_offset":2408,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":700,"record_offset":7348,"parent_record_offset":7316,"ancestor_record_offset":2408,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":700,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":2408,"inlinee":11795,"source_line":253,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":700,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":2408,"inlinee":11793,"source_line":49,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":700,"record_offset":13820,"parent_record_offset":13792,"ancestor_record_offset":2408,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":700,"record_offset":13768,"parent_record_offset":13744,"ancestor_record_offset":2408,"inlinee":11796,"source_line":72,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":700,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":2408,"inlinee":10364,"source_line":76,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/blake3-1.5.3/src/lib.rs
//     evidence={"module":700,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2408,"inlinee":44411,"source_line":1001,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":700,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":2408,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":700,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2408,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":700,"record_offset":3288,"parent_record_offset":3236,"ancestor_record_offset":2408,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":700,"record_offset":5472,"parent_record_offset":5392,"ancestor_record_offset":2408,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":700,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":2408,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":700,"record_offset":2884,"parent_record_offset":2856,"ancestor_record_offset":2408,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":700,"record_offset":8668,"parent_record_offset":8640,"ancestor_record_offset":2408,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":700,"record_offset":5312,"parent_record_offset":4996,"ancestor_record_offset":2408,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=83
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":700,"record_offset":3752,"parent_record_offset":3660,"ancestor_record_offset":2408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":700,"record_offset":8920,"parent_record_offset":8868,"ancestor_record_offset":2408,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":700,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2408,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":700,"record_offset":5852,"parent_record_offset":5744,"ancestor_record_offset":2408,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":700,"record_offset":5500,"parent_record_offset":5392,"ancestor_record_offset":2408,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":700,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":2408,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":700,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":2408,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":700,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2408,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":700,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2408,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":700,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2408,"inlinee":5504,"source_line":2585,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":700,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2408,"inlinee":12627,"source_line":1005,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":700,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2408,"inlinee":18064,"source_line":589,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":700,"record_offset":8952,"parent_record_offset":8868,"ancestor_record_offset":2408,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":700,"record_offset":8988,"parent_record_offset":8828,"ancestor_record_offset":2408,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":2408,"inlinee":5679,"source_line":1849,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10208,"parent_record_offset":10180,"ancestor_record_offset":2408,"inlinee":5680,"source_line":1859,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9748,"parent_record_offset":9712,"ancestor_record_offset":2408,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":2408,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":2408,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9836,"parent_record_offset":9804,"ancestor_record_offset":2408,"inlinee":5677,"source_line":409,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":2408,"inlinee":5700,"source_line":720,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10152,"parent_record_offset":9804,"ancestor_record_offset":2408,"inlinee":5699,"source_line":774,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":10092,"parent_record_offset":9804,"ancestor_record_offset":2408,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":700,"record_offset":9712,"parent_record_offset":9380,"ancestor_record_offset":2408,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":700,"record_offset":4716,"parent_record_offset":4688,"ancestor_record_offset":2408,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":700,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":2408,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":700,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":2408,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":700,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":2408,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":3484,"parent_record_offset":3236,"ancestor_record_offset":2408,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":2408,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":6420,"parent_record_offset":6388,"ancestor_record_offset":2408,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":6680,"parent_record_offset":6388,"ancestor_record_offset":2408,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":700,"record_offset":3688,"parent_record_offset":3660,"ancestor_record_offset":2408,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_to_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":700,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":2408,"inlinee":29044,"source_line":851,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::generic_copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\copy.rs
//     evidence={"module":700,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":2408,"inlinee":24356,"source_line":77,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::impl$6::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\copy.rs
//     evidence={"module":700,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":2408,"inlinee":45295,"source_line":263,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":700,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":2408,"inlinee":4269,"source_line":417,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":700,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":2408,"inlinee":4267,"source_line":77,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":700,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":2408,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2408,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":2408,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":700,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2408,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":700,"record_offset":6480,"parent_record_offset":6420,"ancestor_record_offset":2408,"inlinee":41894,"source_line":221,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":700,"record_offset":6452,"parent_record_offset":6420,"ancestor_record_offset":2408,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":700,"record_offset":9804,"parent_record_offset":9712,"ancestor_record_offset":2408,"inlinee":45298,"source_line":9,"archive":"_pdb/A/modules/00700.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":682,"record_offset":2132,"rva":8925104,"va":5377634224}
// 编译实例 RVAs: 0x882fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t186645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e815d90200","size":5,"soft_fail":false,"target_va":5377821664,"va":5377635014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8501760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e800ba8100","size":5,"soft_fail":false,"target_va":5386138016,"va":5377636251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5398283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8f5a0adff","size":5,"soft_fail":false,"target_va":5372239520,"va":5377637798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5322022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8dacaaeff","size":5,"soft_fail":false,"target_va":5372314208,"va":5377636225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5317190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8baddaeff","size":5,"soft_fail":false,"target_va":5372320512,"va":5377637697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=32
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4884641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e85f77b5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377634348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2463255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8e969daff","size":5,"soft_fail":false,"target_va":5375171056,"va":5377634306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087ef80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8cdb0ffff","size":5,"soft_fail":false,"target_va":5377617792,"va":5377638062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b4b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t203392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8801a0300","size":5,"soft_fail":false,"target_va":5377837920,"va":5377634523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14124b110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10255305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8c97b9c00","size":5,"soft_fail":false,"target_va":5387890960,"va":5377635650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25958272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e880178c01","size":5,"soft_fail":false,"target_va":5403593344,"va":5377635067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-106717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8235ffeff","size":5,"soft_fail":false,"target_va":5377529888,"va":5377636600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t25348174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e84ec88201","size":5,"soft_fail":false,"target_va":5402983344,"va":5377635165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h7fb519d3df594fae (.llvm.923038179807553715)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t25346887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e847c38201","size":5,"soft_fail":false,"target_va":5402981536,"va":5377634644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::haa3d630f77335115 (.llvm.923038179807553715)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t25347452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e87cc58201","size":5,"soft_fail":false,"target_va":5402981936,"va":5377634479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t25344033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e821b88201","size":5,"soft_fail":false,"target_va":5402980688,"va":5377636650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t25345772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8ecbe8201","size":5,"soft_fail":false,"target_va":5402982752,"va":5377636975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t193853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e83df50200","size":5,"soft_fail":false,"target_va":5377828752,"va":5377634894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t191412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8b4eb0200","size":5,"soft_fail":false,"target_va":5377826256,"va":5377634839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t189126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8c6e20200","size":5,"soft_fail":false,"target_va":5377825072,"va":5377635941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<memmap2::Mmap,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8e3efffff","size":5,"soft_fail":false,"target_va":5377634160,"va":5377638280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25342790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e846b38201","size":5,"soft_fail":false,"target_va":5402980128,"va":5377637333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8502883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e863be8100","size":5,"soft_fail":false,"target_va":5386138608,"va":5377635720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1587466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8f6c6e7ff","size":5,"soft_fail":false,"target_va":5376048224,"va":5377635685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-238853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8fb5afcff","size":5,"soft_fail":false,"target_va":5377395552,"va":5377634400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-227617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e8df86fcff","size":5,"soft_fail":false,"target_va":5377408016,"va":5377635628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-229257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e87780fcff","size":5,"soft_fail":false,"target_va":5377405072,"va":5377634324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-194527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377634224,"raw_hex":"e82108fdff","size":5,"soft_fail":false,"target_va":5377439776,"va":5377634298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*33648756(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5377634224,"raw_hex":"ff1574700102","size":6,"soft_fail":false,"va":5377635798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":682,"record_offset":8096,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":682,"record_offset":3680,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":682,"record_offset":3444,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":682,"record_offset":5724,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":682,"record_offset":3148,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/blake3-1.5.3/src/lib.rs
//     evidence={"module":682,"record_offset":3388,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":44410,"source_line":1010,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":682,"record_offset":4032,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":682,"record_offset":4396,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":682,"record_offset":6180,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":682,"record_offset":4196,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":682,"record_offset":7868,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4722,"source_line":682,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":682,"record_offset":5936,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":682,"record_offset":9668,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":682,"record_offset":3556,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":682,"record_offset":5964,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":13488,"source_line":579,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":682,"record_offset":5820,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":682,"record_offset":2364,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":682,"record_offset":2336,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":682,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":2132,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":682,"record_offset":2760,"parent_record_offset":2704,"ancestor_record_offset":2132,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":682,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":2132,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":682,"record_offset":2732,"parent_record_offset":2704,"ancestor_record_offset":2132,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":682,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":2132,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":682,"record_offset":3732,"parent_record_offset":3680,"ancestor_record_offset":2132,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":682,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":2132,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":682,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":2132,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":682,"record_offset":8868,"parent_record_offset":8344,"ancestor_record_offset":2132,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":682,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":2132,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":682,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":2132,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":682,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":2132,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":682,"record_offset":4088,"parent_record_offset":4060,"ancestor_record_offset":2132,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":682,"record_offset":3232,"parent_record_offset":3188,"ancestor_record_offset":2132,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":682,"record_offset":3188,"parent_record_offset":3148,"ancestor_record_offset":2132,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":682,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":2132,"inlinee":11795,"source_line":253,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":682,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":2132,"inlinee":11793,"source_line":49,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":682,"record_offset":11620,"parent_record_offset":11592,"ancestor_record_offset":2132,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":682,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":2132,"inlinee":11796,"source_line":72,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":682,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":2132,"inlinee":10364,"source_line":76,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/blake3-1.5.3/src/lib.rs
//     evidence={"module":682,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2132,"inlinee":44411,"source_line":1001,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":682,"record_offset":3760,"parent_record_offset":3680,"ancestor_record_offset":2132,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":682,"record_offset":8916,"parent_record_offset":8892,"ancestor_record_offset":2132,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":682,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":2132,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":682,"record_offset":5044,"parent_record_offset":5016,"ancestor_record_offset":2132,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":682,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":2132,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":682,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":2132,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":682,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":2132,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=97
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":682,"record_offset":2568,"parent_record_offset":2364,"ancestor_record_offset":2132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":682,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2132,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":682,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":2132,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":682,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":2132,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":682,"record_offset":3592,"parent_record_offset":3556,"ancestor_record_offset":2132,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":682,"record_offset":3788,"parent_record_offset":3680,"ancestor_record_offset":2132,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":682,"record_offset":6136,"parent_record_offset":5992,"ancestor_record_offset":2132,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":682,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":2132,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":682,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":2132,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":682,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":2132,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":682,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2132,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":682,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2132,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":682,"record_offset":3620,"parent_record_offset":3556,"ancestor_record_offset":2132,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":682,"record_offset":6024,"parent_record_offset":5992,"ancestor_record_offset":2132,"inlinee":11471,"source_line":228,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":682,"record_offset":5992,"parent_record_offset":5964,"ancestor_record_offset":2132,"inlinee":11475,"source_line":338,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":682,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":2132,"inlinee":11473,"source_line":229,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":682,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":2132,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":682,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2132,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":682,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":2132,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":682,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":2132,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":682,"record_offset":5904,"parent_record_offset":5820,"ancestor_record_offset":2132,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":682,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":2132,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":682,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":2132,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":682,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":2132,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":682,"record_offset":2400,"parent_record_offset":2364,"ancestor_record_offset":2132,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":682,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":2132,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/00682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12432
// 首条位置: {"module":1792,"inlinee":74802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1792,"record_offset":524,"parent_record_offset":504,"ancestor_record_offset":504,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01792.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1792,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":504,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01792.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1792,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":504,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01792.jsonl.gz"}
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
// 完整结构及原始类型记录: checked_zip.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::checked_zip::FolderChecksumInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_fetcher::checked_zip::FolderChecksumInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo>", "constructor": "serde_json::de::from_slice", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: checked_zip.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: checked_zip.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize
// 信息 ID: 60b0045d09d000505602d74b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo>; module=703, inline_record=500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: e859293475ddbb8ed4dfe22e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=703, inline_record=35224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 44c246738381b31e0a5dd69c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=703, inline_record=35044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 5d76cc8e300d10a534f623a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=700, inline_record=424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 2d7c64b49a5ca9da63aba17d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=703, inline_record=34436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 38754408ea4cf82c5148d608
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=703, inline_record=37856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::serialize
// 信息 ID: 00ed81c8073455f7c4cbae98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >; module=700, inline_record=9804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: 89aee815d45c7812a503ab52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] theme_fetcher::checked_zip::extract_with_checksums -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x3dd127 -> 0x8b3490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums
// 信息 ID: f87658500c3cf75f22cdfd9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x3dadf2 -> 0x882fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums::closure$0
// 信息 ID: c8c72584684ea9412a990625
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::checked_zip::verify_folder_checksums", "captures": "unknown"}
// [闭包环境类型引用] theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0>; module=1792, inline_record=504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
