// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/ui_settings.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 00bf655b88e405cb5b8daa612a4b7ff522f9f0134980adbf1c604a72b0921347
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_settings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 77, 117, 143, 197, 256, 425, 438, 1306, 1439, 1443
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 5, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2236
// 首条位置: {"module":117,"inlinee":8758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":117,"record_offset":114460,"parent_record_offset":114436,"ancestor_record_offset":114436,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":117,"record_offset":80016,"parent_record_offset":79992,"ancestor_record_offset":79992,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":202832,"parent_record_offset":202808,"ancestor_record_offset":202808,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":202856,"parent_record_offset":202832,"ancestor_record_offset":202808,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":77,"record_offset":8944,"rva":984128,"va":5369693248}
// 编译实例 RVAs: 0xf0440, 0xf0570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33268116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693552,"raw_hex":"e894a1fb01","size":5,"soft_fail":false,"target_va":5402961856,"va":5369693735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":77,"record_offset":9136,"parent_record_offset":8944,"ancestor_record_offset":8944,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":77,"record_offset":9164,"parent_record_offset":9136,"ancestor_record_offset":8944,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":77,"record_offset":9192,"parent_record_offset":9164,"ancestor_record_offset":8944,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":77,"record_offset":9220,"parent_record_offset":9192,"ancestor_record_offset":8944,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1439,"record_offset":55328,"rva":17503840,"va":5386212960}
// 编译实例 RVAs: 0x10b1660, 0x10b19e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16743059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386213856,"raw_hex":"e8937aff00","size":5,"soft_fail":false,"target_va":5402957088,"va":5386214024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1439,"record_offset":55520,"parent_record_offset":55328,"ancestor_record_offset":55328,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1439,"record_offset":55548,"parent_record_offset":55520,"ancestor_record_offset":55328,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1439,"record_offset":55576,"parent_record_offset":55548,"ancestor_record_offset":55328,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1439,"record_offset":55604,"parent_record_offset":55576,"ancestor_record_offset":55328,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 5, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27EA
// 首条位置: {"module":117,"inlinee":10218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":7180,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":277048,"parent_record_offset":277024,"ancestor_record_offset":277024,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":99880,"parent_record_offset":99856,"ancestor_record_offset":99856,"inlinee":11525,"source_line":1886,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":100264,"parent_record_offset":99904,"ancestor_record_offset":99856,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":7244,"parent_record_offset":7208,"ancestor_record_offset":7180,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":7308,"parent_record_offset":7208,"ancestor_record_offset":7180,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":7280,"parent_record_offset":7208,"ancestor_record_offset":7180,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":100092,"parent_record_offset":99964,"ancestor_record_offset":99856,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::fix_position
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":100292,"parent_record_offset":99904,"ancestor_record_offset":99856,"inlinee":11399,"source_line":367,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":99964,"parent_record_offset":99904,"ancestor_record_offset":99856,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":99992,"parent_record_offset":99964,"ancestor_record_offset":99856,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":277072,"parent_record_offset":277048,"ancestor_record_offset":277024,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":99904,"parent_record_offset":99880,"ancestor_record_offset":99856,"inlinee":11526,"source_line":1511,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":100116,"parent_record_offset":100092,"ancestor_record_offset":99856,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":100020,"parent_record_offset":99992,"ancestor_record_offset":99856,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":100140,"parent_record_offset":100116,"ancestor_record_offset":99856,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":100048,"parent_record_offset":100020,"ancestor_record_offset":99856,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x606A
// 首条位置: {"module":1439,"inlinee":24682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2854
// 首条位置: {"module":117,"inlinee":10324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":117,"record_offset":114000,"parent_record_offset":112904,"ancestor_record_offset":112904,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":113936,"parent_record_offset":112904,"ancestor_record_offset":112904,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":112948,"parent_record_offset":112904,"ancestor_record_offset":112904,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":113908,"parent_record_offset":112904,"ancestor_record_offset":112904,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":277184,"parent_record_offset":277072,"ancestor_record_offset":276284,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":113156,"parent_record_offset":112984,"ancestor_record_offset":112904,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":113196,"parent_record_offset":113156,"ancestor_record_offset":112904,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":113332,"parent_record_offset":112984,"ancestor_record_offset":112904,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":113828,"parent_record_offset":112984,"ancestor_record_offset":112904,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":113112,"parent_record_offset":112984,"ancestor_record_offset":112904,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":113788,"parent_record_offset":113688,"ancestor_record_offset":112904,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":117,"record_offset":114436,"parent_record_offset":114412,"ancestor_record_offset":112904,"inlinee":8758,"source_line":24,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":117,"record_offset":113300,"parent_record_offset":113276,"ancestor_record_offset":112904,"inlinee":10218,"source_line":5,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":117,"record_offset":113236,"parent_record_offset":112984,"ancestor_record_offset":112904,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":113688,"parent_record_offset":113660,"ancestor_record_offset":112904,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":112984,"parent_record_offset":112948,"ancestor_record_offset":112904,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":114188,"parent_record_offset":114152,"ancestor_record_offset":112904,"inlinee":10245,"source_line":857,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":117,"record_offset":114460,"parent_record_offset":114436,"ancestor_record_offset":112904,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":113760,"parent_record_offset":113688,"ancestor_record_offset":112904,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":113276,"parent_record_offset":112984,"ancestor_record_offset":112904,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":278164,"parent_record_offset":278140,"ancestor_record_offset":276284,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":276608,"parent_record_offset":276456,"ancestor_record_offset":276284,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":277356,"parent_record_offset":277324,"ancestor_record_offset":276284,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":276456,"parent_record_offset":276376,"ancestor_record_offset":276284,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":276496,"parent_record_offset":276456,"ancestor_record_offset":276284,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":276376,"parent_record_offset":276336,"ancestor_record_offset":276284,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":277324,"parent_record_offset":277296,"ancestor_record_offset":276284,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":277072,"parent_record_offset":277048,"ancestor_record_offset":276284,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":277048,"parent_record_offset":277024,"ancestor_record_offset":276284,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":276632,"parent_record_offset":276608,"ancestor_record_offset":276284,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":276528,"parent_record_offset":276496,"ancestor_record_offset":276284,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":276656,"parent_record_offset":276632,"ancestor_record_offset":276284,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":276560,"parent_record_offset":276528,"ancestor_record_offset":276284,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFD3
// 首条位置: {"module":1306,"inlinee":61395}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":279984,"parent_record_offset":279912,"ancestor_record_offset":279912,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":279956,"parent_record_offset":279912,"ancestor_record_offset":279912,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":280664,"parent_record_offset":280052,"ancestor_record_offset":279912,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":280264,"parent_record_offset":280124,"ancestor_record_offset":279912,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":280124,"parent_record_offset":280052,"ancestor_record_offset":279912,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":280160,"parent_record_offset":280124,"ancestor_record_offset":279912,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":280052,"parent_record_offset":280012,"ancestor_record_offset":279912,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":280288,"parent_record_offset":280264,"ancestor_record_offset":279912,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":280188,"parent_record_offset":280160,"ancestor_record_offset":279912,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":280312,"parent_record_offset":280288,"ancestor_record_offset":279912,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":280216,"parent_record_offset":280188,"ancestor_record_offset":279912,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":77,"record_offset":232,"rva":978832,"va":5369687952}
// 编译实例 RVAs: 0xeef90, 0xef5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140009700
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-941870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689552,"raw_hex":"e8d2a0f1ff","size":5,"soft_fail":false,"target_va":5368747776,"va":5369689641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140009a40
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-941001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689552,"raw_hex":"e837a4f1ff","size":5,"soft_fail":false,"target_va":5368748608,"va":5369689604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14000ba10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-931502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369687952,"raw_hex":"e852c9f1ff","size":5,"soft_fail":false,"target_va":5368756752,"va":5369688249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14000ee70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-917968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369687952,"raw_hex":"e830fef1ff","size":5,"soft_fail":false,"target_va":5368770160,"va":5369688123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400102c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-912831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369687952,"raw_hex":"e84112f2ff","size":5,"soft_fail":false,"target_va":5368775360,"va":5369688186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400105d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-912301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369687952,"raw_hex":"e85314f2ff","size":5,"soft_fail":false,"target_va":5368776144,"va":5369688440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140010ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-909547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369687952,"raw_hex":"e8151ff2ff","size":5,"soft_fail":false,"target_va":5368778480,"va":5369688022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":77,"record_offset":724,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":77,"record_offset":452,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":77,"record_offset":668,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":77,"record_offset":3176,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":5057,"source_line":854,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":77,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":232,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":77,"record_offset":3204,"parent_record_offset":3176,"ancestor_record_offset":2964,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":77,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":232,"inlinee":4979,"source_line":1098,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":77,"record_offset":912,"rva":979984,"va":5369689104}
// 编译实例 RVAs: 0xef410, 0xef7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dfbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-63928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689104,"raw_hex":"e84806ffff","size":5,"soft_fail":false,"target_va":5369625520,"va":5369689443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ef7f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689104,"raw_hex":"e815030000","size":5,"soft_fail":false,"target_va":5369690096,"va":5369689302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140271110
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1580127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689104,"raw_hex":"e85f1c1800","size":5,"soft_fail":false,"target_va":5371269392,"va":5369689260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140272010
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1583889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689104,"raw_hex":"e8112b1800","size":5,"soft_fail":false,"target_va":5371273232,"va":5369689338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140274c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1595329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689104,"raw_hex":"e8c1571800","size":5,"soft_fail":false,"target_va":5371284512,"va":5369689178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33229592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369689104,"raw_hex":"e8180bfb01","size":5,"soft_fail":false,"target_va":5402919072,"va":5369689475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t33269795","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5369690096,"raw_hex":"e923a8fb01","size":5,"soft_fail":false,"target_va":5402960048,"va":5369690248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1192,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":5677,"source_line":409,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":3992,"parent_record_offset":3720,"ancestor_record_offset":3720,"inlinee":8725,"source_line":249,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2812,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":5699,"source_line":774,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1644,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":912,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":1552,"parent_record_offset":1520,"ancestor_record_offset":912,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":912,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1388,"parent_record_offset":1360,"ancestor_record_offset":912,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1520,"parent_record_offset":1388,"ancestor_record_offset":912,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":912,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1584,"parent_record_offset":1388,"ancestor_record_offset":912,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1424,"parent_record_offset":1388,"ancestor_record_offset":912,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":77,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":912,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":77,"record_offset":4104,"parent_record_offset":4068,"ancestor_record_offset":3720,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":77,"record_offset":4068,"parent_record_offset":4032,"ancestor_record_offset":3720,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::SerializeMap::serialize_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/mod.rs
//     evidence={"module":77,"record_offset":1704,"parent_record_offset":1672,"ancestor_record_offset":912,"inlinee":5684,"source_line":1804,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$0::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":77,"record_offset":2204,"parent_record_offset":1744,"ancestor_record_offset":912,"inlinee":5689,"source_line":42,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":912,"inlinee":5679,"source_line":1849,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1812,"parent_record_offset":1744,"ancestor_record_offset":912,"inlinee":5687,"source_line":1868,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2328,"parent_record_offset":2292,"ancestor_record_offset":912,"inlinee":5694,"source_line":1894,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":912,"inlinee":5680,"source_line":1859,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$11::serialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":912,"inlinee":5691,"source_line":857,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1788,"parent_record_offset":1744,"ancestor_record_offset":912,"inlinee":5696,"source_line":503,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":912,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":912,"inlinee":5692,"source_line":219,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2840,"parent_record_offset":2812,"ancestor_record_offset":912,"inlinee":5700,"source_line":720,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::serialize_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":1744,"parent_record_offset":1704,"ancestor_record_offset":912,"inlinee":5686,"source_line":667,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::serialize_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":77,"record_offset":2292,"parent_record_offset":1704,"ancestor_record_offset":912,"inlinee":5693,"source_line":695,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":77,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":912,"inlinee":4269,"source_line":417,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":77,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":912,"inlinee":4267,"source_line":77,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1439,"record_offset":54448,"rva":17503552,"va":5386212672}
// 编译实例 RVAs: 0x10b1540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141104ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t341287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212672,"raw_hex":"e827350500","size":5,"soft_fail":false,"target_va":5386554080,"va":5386212788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141105290
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t343223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212672,"raw_hex":"e8b73c0500","size":5,"soft_fail":false,"target_va":5386556048,"va":5386212820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411058c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t344875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212672,"raw_hex":"e82b430500","size":5,"soft_fail":false,"target_va":5386557632,"va":5386212752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16706147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212672,"raw_hex":"e863eafe00","size":5,"soft_fail":false,"target_va":5402919072,"va":5386212920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x14006d480
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17056223","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5386212672,"raw_hex":"e921befbfe","size":5,"soft_fail":false,"target_va":5369156736,"va":5386212954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1439,"record_offset":54724,"parent_record_offset":54448,"ancestor_record_offset":54448,"inlinee":5677,"source_line":409,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1439,"record_offset":55184,"parent_record_offset":54448,"ancestor_record_offset":54448,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":55024,"parent_record_offset":54992,"ancestor_record_offset":54448,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":55092,"parent_record_offset":55060,"ancestor_record_offset":54448,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":54992,"parent_record_offset":54964,"ancestor_record_offset":54448,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":54924,"parent_record_offset":54896,"ancestor_record_offset":54448,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":55060,"parent_record_offset":54924,"ancestor_record_offset":54448,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":54868,"parent_record_offset":54840,"ancestor_record_offset":54448,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":55124,"parent_record_offset":54924,"ancestor_record_offset":54448,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":54964,"parent_record_offset":54924,"ancestor_record_offset":54448,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1439,"record_offset":54896,"parent_record_offset":54868,"ancestor_record_offset":54448,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1439,"record_offset":54780,"parent_record_offset":54752,"ancestor_record_offset":54448,"inlinee":7541,"source_line":2007,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1439,"record_offset":54752,"parent_record_offset":54724,"ancestor_record_offset":54448,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1439,"record_offset":54840,"parent_record_offset":54812,"ancestor_record_offset":54448,"inlinee":4269,"source_line":417,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1439,"record_offset":54812,"parent_record_offset":54780,"ancestor_record_offset":54448,"inlinee":4267,"source_line":77,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x317A
// 首条位置: {"module":197,"inlinee":12666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$3::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":425,"record_offset":38320,"rva":5978128,"va":5374687248}
// 编译实例 RVAs: 0x5b3810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6069
// 首条位置: {"module":1439,"inlinee":24681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x223D
// 首条位置: {"module":117,"inlinee":8765}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":65052,"parent_record_offset":65000,"ancestor_record_offset":65000,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":65024,"parent_record_offset":65000,"ancestor_record_offset":65000,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":57108,"parent_record_offset":56972,"ancestor_record_offset":56836,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56932,"parent_record_offset":56900,"ancestor_record_offset":56836,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56972,"parent_record_offset":56932,"ancestor_record_offset":56836,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":57004,"parent_record_offset":56972,"ancestor_record_offset":56836,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56900,"parent_record_offset":56868,"ancestor_record_offset":56836,"inlinee":9488,"source_line":2020,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56716,"parent_record_offset":56692,"ancestor_record_offset":56664,"inlinee":9486,"source_line":2073,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":57132,"parent_record_offset":57108,"ancestor_record_offset":56836,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":57032,"parent_record_offset":57004,"ancestor_record_offset":56836,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":57156,"parent_record_offset":57132,"ancestor_record_offset":56836,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":57060,"parent_record_offset":57032,"ancestor_record_offset":56836,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1439,"record_offset":56144,"rva":17504144,"va":5386213264}
// 编译实例 RVAs: 0x10b1790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140d48e80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3574161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386213264,"raw_hex":"e86f76c9ff","size":5,"soft_fail":false,"target_va":5382639232,"va":5386213388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d49c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3570805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386213264,"raw_hex":"e88b83c9ff","size":5,"soft_fail":false,"target_va":5382642832,"va":5386213632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a0d00
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16708865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386213264,"raw_hex":"e801f5fe00","size":5,"soft_fail":false,"target_va":5402922240,"va":5386213370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a16d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16711015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386213264,"raw_hex":"e867fdfe00","size":5,"soft_fail":false,"target_va":5402924752,"va":5386213732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56360,"parent_record_offset":56144,"ancestor_record_offset":56144,"inlinee":9479,"source_line":1855,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":1439,"record_offset":56664,"parent_record_offset":56360,"ancestor_record_offset":56144,"inlinee":8765,"source_line":24,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":56692,"parent_record_offset":56664,"ancestor_record_offset":56144,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56576,"parent_record_offset":56444,"ancestor_record_offset":56144,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56932,"parent_record_offset":56900,"ancestor_record_offset":56144,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56444,"parent_record_offset":56360,"ancestor_record_offset":56144,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56476,"parent_record_offset":56444,"ancestor_record_offset":56144,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56900,"parent_record_offset":56868,"ancestor_record_offset":56144,"inlinee":9488,"source_line":2020,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1439,"record_offset":56716,"parent_record_offset":56692,"ancestor_record_offset":56144,"inlinee":9486,"source_line":2073,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":56600,"parent_record_offset":56576,"ancestor_record_offset":56144,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":56504,"parent_record_offset":56476,"ancestor_record_offset":56144,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":56624,"parent_record_offset":56600,"ancestor_record_offset":56144,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1439,"record_offset":56532,"parent_record_offset":56504,"ancestor_record_offset":56144,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":425,"record_offset":38452,"rva":5978144,"va":5374687264}
// 编译实例 RVAs: 0x5b3820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::previous
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":425,"record_offset":38584,"rva":5978160,"va":5374687280}
// 编译实例 RVAs: 0x5b3830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2235
// 首条位置: {"module":143,"inlinee":8757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1443,"record_offset":86408,"parent_record_offset":86384,"ancestor_record_offset":86384,"inlinee":8725,"source_line":249,"archive":"_pdb/A/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1443,"record_offset":86432,"parent_record_offset":86408,"ancestor_record_offset":86384,"inlinee":5692,"source_line":219,"archive":"_pdb/A/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":425,"record_offset":36232,"rva":5977232,"va":5374686352}
// 编译实例 RVAs: 0x5b3490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1466442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374686352,"raw_hex":"e84a601600","size":5,"soft_fail":false,"target_va":5376152880,"va":5374686433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2897463","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374686352,"raw_hex":"e937362c00","size":5,"soft_fail":false,"target_va":5377583920,"va":5374686452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: ui_settings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ui_settings::ToolbarAlignment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ui_settings::UiSettings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::UiSettings>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::UiSettings>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::UiSettings>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_settings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_settings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize
// 信息 ID: 01fe9a838db0ee8d109a6272
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_config::_::impl$0::deserialize::__Visitor>; module=117, inline_record=79992; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=117, inline_record=114436; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=199684; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=202808; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=205616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: df81de714132424b1cf5e1b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor>; module=1439, inline_record=3216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 037feb4492a7aa52b84c2d00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x1c81ba -> 0xf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x317343 -> 0xf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x317388 -> 0xf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> > @ 0x1ab337 -> 0xf0570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 820bed5942fbdbc79df5bab3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0xd61da9 -> 0x10b1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<country_core::resources::ui_settings::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > > @ 0xd48f40 -> 0x10b19e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 69732b5bb8fce990d6148f48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> >; module=117, inline_record=7180; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=117, inline_record=113300; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=117, inline_record=113508; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=117, inline_record=113660; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$5::deserialize<country_core::resources::ui_settings::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1306, inline_record=99856; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=277024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: a69250e72e779f46dd88e60d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=1072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: f9d85642dd9dccf86475c628
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=117, inline_record=65000; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1439, inline_record=56664; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1439, inline_record=56836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: fc5c5d819891ec63b8b63b26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=117, inline_record=112904; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=276284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 2a5f556d06c1986c1d1c636f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=279912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
// 信息 ID: 15b9abf6baf722bfe36f772f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::StrRead>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize -> ["ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0xd6230c -> 0x10b1790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::next
// 信息 ID: 06042b668b29128f40b5e1d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eef1 -> 0x5b3820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43d12 -> 0x5b3820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::previous
// 信息 ID: 2ec629f365215fbd15af6fe4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ece2 -> 0x5b3830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize
// 信息 ID: 59f55a1b13665f472e93c4fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>; module=1443, inline_record=86384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 2493196e69bc7131c1bc0b0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::UiSettings> @ 0xaaf4 -> 0xeef90 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::ToolbarAlignment> @ 0x104a4 -> 0xef5d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: f99ed4fc9da55cd2f3bc5ba9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::UiSettings> @ 0x274f58 -> 0xef410 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xef4d6 -> 0xef7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: ef2767fdd239de859a63424f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::UiSettings> @ 0x1103803 -> 0x10b1540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$0::fmt
// 信息 ID: e137d2702ef5e9e270750c03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ed37 -> 0x5b3490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$2::clone
// 信息 ID: 9532cf075151241e368adab3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=47896; 不是独立调用边。
// [PDB 内联宿主] country_core::game_config::impl$9::clone; module=256, inline_record=14932; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::input::input_record_replay::input_record_replay; module=438, inline_record=9936; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::input::input_record_replay::input_record_replay; module=438, inline_record=10848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$3::default
// 信息 ID: 43608a089009ddf1b6c50eee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x1c8746 -> 0x5b3810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
