// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/walls/wall_attachment.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5611d4d90840167dfab0fa59a484bbde24512c0f07becfb5fbe7b8aaefae3dbd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_attachment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 55, 60, 100, 431, 475, 1309, 1370, 1389, 1432, 1440, 1459, 3900
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 6, 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6163
// 首条位置: {"module":431,"inlinee":24931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":112912,"parent_record_offset":112856,"ancestor_record_offset":112856,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":213656,"parent_record_offset":213632,"ancestor_record_offset":213632,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":112884,"parent_record_offset":112856,"ancestor_record_offset":112856,"inlinee":10276,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":213680,"parent_record_offset":213656,"ancestor_record_offset":213632,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":291600,"parent_record_offset":291576,"ancestor_record_offset":291552,"inlinee":24941,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1459,"record_offset":51344,"rva":17977200,"va":5386686320}
// 编译实例 RVAs: 0x1124f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1459,"record_offset":51544,"parent_record_offset":51344,"ancestor_record_offset":51344,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1459,"record_offset":51568,"parent_record_offset":51544,"ancestor_record_offset":51344,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":51592,"parent_record_offset":51568,"ancestor_record_offset":51344,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1459,"record_offset":51616,"parent_record_offset":51592,"ancestor_record_offset":51344,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 6, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1044E
// 首条位置: {"module":1440,"inlinee":66638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":346520,"parent_record_offset":346492,"ancestor_record_offset":346492,"inlinee":66649,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":288760,"parent_record_offset":288732,"ancestor_record_offset":288732,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":289028,"parent_record_offset":289004,"ancestor_record_offset":288732,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":289004,"parent_record_offset":288948,"ancestor_record_offset":288732,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":288976,"parent_record_offset":288948,"ancestor_record_offset":288732,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":288948,"parent_record_offset":288924,"ancestor_record_offset":288732,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1440,"record_offset":346608,"parent_record_offset":346576,"ancestor_record_offset":346492,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":346680,"parent_record_offset":346656,"ancestor_record_offset":346492,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":288876,"parent_record_offset":288844,"ancestor_record_offset":288732,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":346704,"parent_record_offset":346680,"ancestor_record_offset":346492,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":346728,"parent_record_offset":346704,"ancestor_record_offset":346492,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":346656,"parent_record_offset":346636,"ancestor_record_offset":346492,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1440,"record_offset":346636,"parent_record_offset":346576,"ancestor_record_offset":346492,"inlinee":66663,"source_line":27,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":288844,"parent_record_offset":288816,"ancestor_record_offset":288732,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1440,"record_offset":346548,"parent_record_offset":346520,"ancestor_record_offset":346492,"inlinee":66650,"source_line":615,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":288788,"parent_record_offset":288760,"ancestor_record_offset":288732,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":288816,"parent_record_offset":288788,"ancestor_record_offset":288732,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6979
// 首条位置: {"module":1459,"inlinee":27001}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1044C
// 首条位置: {"module":1440,"inlinee":66636}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":289824,"parent_record_offset":287248,"ancestor_record_offset":287248,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":290128,"parent_record_offset":287248,"ancestor_record_offset":287248,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":291528,"parent_record_offset":287248,"ancestor_record_offset":287248,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":287292,"parent_record_offset":287248,"ancestor_record_offset":287248,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":289548,"parent_record_offset":287248,"ancestor_record_offset":287248,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":287944,"parent_record_offset":287920,"ancestor_record_offset":287248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":287920,"parent_record_offset":287864,"ancestor_record_offset":287248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":288564,"parent_record_offset":288536,"ancestor_record_offset":287248,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":288624,"parent_record_offset":288564,"ancestor_record_offset":287248,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":287440,"parent_record_offset":287416,"ancestor_record_offset":287248,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":287492,"parent_record_offset":287464,"ancestor_record_offset":287248,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":287516,"parent_record_offset":287492,"ancestor_record_offset":287248,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":287892,"parent_record_offset":287864,"ancestor_record_offset":287248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":288076,"parent_record_offset":288052,"ancestor_record_offset":287248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":287864,"parent_record_offset":287836,"ancestor_record_offset":287248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":288052,"parent_record_offset":288024,"ancestor_record_offset":287248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":288024,"parent_record_offset":287992,"ancestor_record_offset":287248,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":288656,"parent_record_offset":288564,"ancestor_record_offset":287248,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":289664,"parent_record_offset":289640,"ancestor_record_offset":287248,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":288224,"parent_record_offset":288196,"ancestor_record_offset":287248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":287464,"parent_record_offset":287440,"ancestor_record_offset":287248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":289640,"parent_record_offset":289604,"ancestor_record_offset":287248,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":287732,"parent_record_offset":287364,"ancestor_record_offset":287248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":287592,"parent_record_offset":287568,"ancestor_record_offset":287248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":288592,"parent_record_offset":288564,"ancestor_record_offset":287248,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":289720,"parent_record_offset":289664,"ancestor_record_offset":287248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":289300,"parent_record_offset":287364,"ancestor_record_offset":287248,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":287544,"parent_record_offset":287516,"ancestor_record_offset":287248,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":287568,"parent_record_offset":287544,"ancestor_record_offset":287248,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1440,"record_offset":291092,"parent_record_offset":291068,"ancestor_record_offset":287248,"inlinee":25266,"source_line":65,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1440,"record_offset":291140,"parent_record_offset":291116,"ancestor_record_offset":287248,"inlinee":25508,"source_line":65,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1440,"record_offset":290728,"parent_record_offset":290704,"ancestor_record_offset":287248,"inlinee":24931,"source_line":38,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1440,"record_offset":288732,"parent_record_offset":288704,"ancestor_record_offset":287248,"inlinee":66638,"source_line":27,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$15::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1440,"record_offset":291452,"parent_record_offset":291428,"ancestor_record_offset":287248,"inlinee":11509,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":291624,"parent_record_offset":291600,"ancestor_record_offset":287248,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":291600,"parent_record_offset":291576,"ancestor_record_offset":287248,"inlinee":24941,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":287328,"parent_record_offset":287292,"ancestor_record_offset":287248,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":289576,"parent_record_offset":289548,"ancestor_record_offset":287248,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":288844,"parent_record_offset":288816,"ancestor_record_offset":287248,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":289780,"parent_record_offset":289756,"ancestor_record_offset":287248,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":288704,"parent_record_offset":287364,"ancestor_record_offset":287248,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":290752,"parent_record_offset":290728,"ancestor_record_offset":287248,"inlinee":10276,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":291164,"parent_record_offset":291140,"ancestor_record_offset":287248,"inlinee":10259,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":290084,"parent_record_offset":290060,"ancestor_record_offset":287248,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":287416,"parent_record_offset":287364,"ancestor_record_offset":287248,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":288788,"parent_record_offset":288760,"ancestor_record_offset":287248,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":288760,"parent_record_offset":288732,"ancestor_record_offset":287248,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":288816,"parent_record_offset":288788,"ancestor_record_offset":287248,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":291116,"parent_record_offset":291092,"ancestor_record_offset":287248,"inlinee":27028,"source_line":298,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":287364,"parent_record_offset":287328,"ancestor_record_offset":287248,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":289604,"parent_record_offset":289576,"ancestor_record_offset":287248,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10422
// 首条位置: {"module":1440,"inlinee":66594}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":112044,"parent_record_offset":112012,"ancestor_record_offset":112012,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":112128,"parent_record_offset":112100,"ancestor_record_offset":112012,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":112188,"parent_record_offset":112128,"ancestor_record_offset":112012,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":112156,"parent_record_offset":112128,"ancestor_record_offset":112012,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":112276,"parent_record_offset":112100,"ancestor_record_offset":112012,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1440,"record_offset":113172,"parent_record_offset":113148,"ancestor_record_offset":112012,"inlinee":25266,"source_line":65,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1440,"record_offset":113220,"parent_record_offset":113196,"ancestor_record_offset":112012,"inlinee":25508,"source_line":65,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1440,"record_offset":112856,"parent_record_offset":112828,"ancestor_record_offset":112012,"inlinee":24931,"source_line":38,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$15::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1440,"record_offset":112544,"parent_record_offset":112520,"ancestor_record_offset":112012,"inlinee":11509,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":112072,"parent_record_offset":112044,"ancestor_record_offset":112012,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":112220,"parent_record_offset":112100,"ancestor_record_offset":112012,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":112884,"parent_record_offset":112856,"ancestor_record_offset":112012,"inlinee":10276,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":113244,"parent_record_offset":113220,"ancestor_record_offset":112012,"inlinee":10259,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":112244,"parent_record_offset":112220,"ancestor_record_offset":112012,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":113196,"parent_record_offset":113172,"ancestor_record_offset":112012,"inlinee":27028,"source_line":298,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":112100,"parent_record_offset":112072,"ancestor_record_offset":112012,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":60,"record_offset":232,"rva":527296,"va":5369236416}
// 编译实例 RVAs: 0x80bc0, 0x1124d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=2
//     target_aliases=1000 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-514136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e8a827f8ff","size":5,"soft_fail":false,"target_va":5368722320,"va":5369236451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-429952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e88070f9ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5369236987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401505d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t850271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e85ff90c00","size":5,"soft_fail":false,"target_va":5370086864,"va":5369236588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401531a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t861439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e8ff240d00","size":5,"soft_fail":false,"target_va":5370098080,"va":5369236636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401549a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t867545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e8d93c0d00","size":5,"soft_fail":false,"target_va":5370104224,"va":5369236674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t874905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e899590d00","size":5,"soft_fail":false,"target_va":5370111424,"va":5369236514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140156ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t877170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e872620d00","size":5,"soft_fail":false,"target_va":5370113728,"va":5369236553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3017032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e848092e00","size":5,"soft_fail":false,"target_va":5372253808,"va":5369236771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3512907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e84b9a3500","size":5,"soft_fail":false,"target_va":5372749712,"va":5369236800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11903760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e8f05c4aff","size":5,"soft_fail":false,"target_va":5374782544,"va":5386686299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10794121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e8774b5bff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386685988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6789122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369236416,"raw_hex":"e802986700","size":5,"soft_fail":false,"target_va":5376025984,"va":5369236857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-582777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e8871bf7ff","size":5,"soft_fail":false,"target_va":5386103296,"va":5386686068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-581928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e8d81ef7ff","size":5,"soft_fail":false,"target_va":5386103968,"va":5386685891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-581186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e8be21f7ff","size":5,"soft_fail":false,"target_va":5386104640,"va":5386685821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410974a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-580099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e8fd25f7ff","size":5,"soft_fail":false,"target_va":5386106016,"va":5386686110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109c5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-559061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386685728,"raw_hex":"e82b78f7ff","size":5,"soft_fail":false,"target_va":5386126800,"va":5386685856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":60,"record_offset":600,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":60,"record_offset":572,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_attachment::is_default
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":60,"record_offset":432,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8041,"source_line":34,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":60,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":60,"record_offset":960,"parent_record_offset":904,"ancestor_record_offset":232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":60,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":232,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":60,"record_offset":732,"parent_record_offset":676,"ancestor_record_offset":232,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":60,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":60,"record_offset":904,"parent_record_offset":876,"ancestor_record_offset":232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":60,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":60,"record_offset":764,"parent_record_offset":676,"ancestor_record_offset":232,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":60,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":60,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":232,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":60,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":232,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10614
// 首条位置: {"module":1459,"inlinee":67092}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF09F
// 首条位置: {"module":1309,"inlinee":61599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_attachment::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1309,"record_offset":109284,"parent_record_offset":109220,"ancestor_record_offset":109220,"inlinee":61601,"source_line":38,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1309,"record_offset":109252,"parent_record_offset":109220,"ancestor_record_offset":109220,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10467
// 首条位置: {"module":1440,"inlinee":66663}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":346656,"parent_record_offset":346636,"ancestor_record_offset":346636,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":346680,"parent_record_offset":346656,"ancestor_record_offset":346636,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":346704,"parent_record_offset":346680,"ancestor_record_offset":346636,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":346728,"parent_record_offset":346704,"ancestor_record_offset":346636,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1061D
// 首条位置: {"module":1459,"inlinee":67101}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":52988,"parent_record_offset":52932,"ancestor_record_offset":52932,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":53020,"parent_record_offset":52932,"ancestor_record_offset":52932,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":52960,"parent_record_offset":52932,"ancestor_record_offset":52932,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1459,"record_offset":52140,"rva":17977424,"va":5386686544}
// 编译实例 RVAs: 0x1125050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16273256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e8684ff800","size":5,"soft_fail":false,"target_va":5402960384,"va":5386687123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10670069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e80b305dff","size":5,"soft_fail":false,"target_va":5376016896,"va":5386686960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13938006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e8aa522bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386687713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12446084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e87c1642ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5386688351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12443080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e8382242ff","size":5,"soft_fail":false,"target_va":5374243632,"va":5386686707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10794946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e83e485bff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386686813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10756903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e8d9dc5bff","size":5,"soft_fail":false,"target_va":5375930160,"va":5386687058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4b80
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10750357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e86bf65bff","size":5,"soft_fail":false,"target_va":5375937408,"va":5386687760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10699276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e8f4bd5cff","size":5,"soft_fail":false,"target_va":5375988576,"va":5386687847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d95c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-310264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e80844fbff","size":5,"soft_fail":false,"target_va":5386376640,"va":5386686899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_attachment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16269797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e8e541f800","size":5,"soft_fail":false,"target_va":5402956592,"va":5386686790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16272655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386686544,"raw_hex":"e80f4df800","size":5,"soft_fail":false,"target_va":5402959760,"va":5386687100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1459,"record_offset":52332,"parent_record_offset":52140,"ancestor_record_offset":52140,"inlinee":25381,"source_line":255,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1459,"record_offset":53320,"parent_record_offset":53296,"ancestor_record_offset":52140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1459,"record_offset":53296,"parent_record_offset":53204,"ancestor_record_offset":52140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":54364,"parent_record_offset":54336,"ancestor_record_offset":52140,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":52532,"parent_record_offset":52504,"ancestor_record_offset":52140,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":52596,"parent_record_offset":52564,"ancestor_record_offset":52140,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1459,"record_offset":52656,"parent_record_offset":52624,"ancestor_record_offset":52140,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1459,"record_offset":52684,"parent_record_offset":52656,"ancestor_record_offset":52140,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":53232,"parent_record_offset":53204,"ancestor_record_offset":52140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":52140,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1459,"record_offset":53204,"parent_record_offset":53180,"ancestor_record_offset":52140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1459,"record_offset":53648,"parent_record_offset":53620,"ancestor_record_offset":52140,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1459,"record_offset":53116,"parent_record_offset":53084,"ancestor_record_offset":52140,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1459,"record_offset":54456,"parent_record_offset":54364,"ancestor_record_offset":52140,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1459,"record_offset":53260,"parent_record_offset":53232,"ancestor_record_offset":52140,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1459,"record_offset":52624,"parent_record_offset":52596,"ancestor_record_offset":52140,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1459,"record_offset":52904,"parent_record_offset":52332,"ancestor_record_offset":52140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1459,"record_offset":52764,"parent_record_offset":52740,"ancestor_record_offset":52140,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1459,"record_offset":54392,"parent_record_offset":54364,"ancestor_record_offset":52140,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":52988,"parent_record_offset":52932,"ancestor_record_offset":52140,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1459,"record_offset":53020,"parent_record_offset":52932,"ancestor_record_offset":52140,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1459,"record_offset":52716,"parent_record_offset":52684,"ancestor_record_offset":52140,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1459,"record_offset":52740,"parent_record_offset":52716,"ancestor_record_offset":52140,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1459,"record_offset":52932,"parent_record_offset":52332,"ancestor_record_offset":52140,"inlinee":67101,"source_line":27,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1459,"record_offset":52960,"parent_record_offset":52932,"ancestor_record_offset":52140,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1459,"record_offset":52504,"parent_record_offset":52332,"ancestor_record_offset":52140,"inlinee":25383,"source_line":877,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1459,"record_offset":52564,"parent_record_offset":52332,"ancestor_record_offset":52140,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01459.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 27, 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E91
// 首条位置: {"module":55,"inlinee":7825}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":100,"record_offset":177788,"parent_record_offset":177760,"ancestor_record_offset":177760,"inlinee":7826,"source_line":170,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":100,"record_offset":178168,"parent_record_offset":178144,"ancestor_record_offset":177760,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":100,"record_offset":178192,"parent_record_offset":178168,"ancestor_record_offset":177760,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":100,"record_offset":178144,"parent_record_offset":178108,"ancestor_record_offset":177760,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":100,"record_offset":178108,"parent_record_offset":178076,"ancestor_record_offset":177760,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":100,"record_offset":178076,"parent_record_offset":178044,"ancestor_record_offset":177760,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":100,"record_offset":178620,"parent_record_offset":178564,"ancestor_record_offset":177760,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":178012,"parent_record_offset":177980,"ancestor_record_offset":177760,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":177980,"parent_record_offset":177948,"ancestor_record_offset":177760,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":177916,"parent_record_offset":177884,"ancestor_record_offset":177760,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":177948,"parent_record_offset":177916,"ancestor_record_offset":177760,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":100,"record_offset":177884,"parent_record_offset":177852,"ancestor_record_offset":177760,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":100,"record_offset":177852,"parent_record_offset":177820,"ancestor_record_offset":177760,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":100,"record_offset":178044,"parent_record_offset":178012,"ancestor_record_offset":177760,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":100,"record_offset":178680,"parent_record_offset":178656,"ancestor_record_offset":177760,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":100,"record_offset":178656,"parent_record_offset":178500,"ancestor_record_offset":177760,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":100,"record_offset":178220,"parent_record_offset":178192,"ancestor_record_offset":177760,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":100,"record_offset":177820,"parent_record_offset":177788,"ancestor_record_offset":177760,"inlinee":7827,"source_line":150,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$7::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10610
// 首条位置: {"module":1459,"inlinee":67088}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F69
// 首条位置: {"module":60,"inlinee":8041}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":60,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":432,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":60,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":488,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00060.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$13::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1459,"record_offset":56056,"rva":17979568,"va":5386688688}
// 编译实例 RVAs: 0x11258b0, 0x1c84570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9102850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386688688,"raw_hex":"e8fe1975ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5386688733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF0A1
// 首条位置: {"module":1309,"inlinee":61601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":53696,"rva":6727456,"va":5375436576}
// 编译实例 RVAs: 0x66a720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":53856,"rva":6727472,"va":5375436592}
// 编译实例 RVAs: 0x66a730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27542517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375436592,"raw_hex":"e8f543a401","size":5,"soft_fail":false,"target_va":5402979152,"va":5375436630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":54028,"rva":6727520,"va":5375436640}
// 编译实例 RVAs: 0x66a760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_attachment::impl$1::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":54200,"rva":13200,"va":5368722320}
// 编译实例 RVAs: 0x3390
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
// 完整结构及原始类型记录: wall_attachment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_attachment::WallAttachmentAnchor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentAnchor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentAnchor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_attachment::WallAttachmentSide
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentSide>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentSide"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::walls::wall_attachment::WallAttachmentSide>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::walls::wall_attachment::WallAttachmentSide"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_attachment::WallCornerAttachment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallCornerAttachment>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::wall_attachment::WallCornerAttachment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::wall_attachment::WallCornerAttachment>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::wall_attachment::WallCornerAttachment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_attachment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_attachment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::steam::SteamInput>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
// 信息 ID: 77288a19c67975b274f715ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0ac7b -> 0x66a730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
// 信息 ID: 5fb4d5bade0971fc947f1c62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtype @ 0x17d772f -> 0x66a720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d955c -> 0x66a720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e69c1b -> 0x66a720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
// 信息 ID: 283fae2e1f3265fb9877e04e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd3e5 -> 0x66a760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502ef2 -> 0x66a760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c542a1 -> 0x66a760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize
// 信息 ID: 69187c50c7860a91b86f3b8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=112856; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=134368; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=213192; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::decorator::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=213632; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=290728; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=291552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 6e92c198369f2ae73a38e329
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Field> >; module=1440, inline_record=346636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 993aa7864eb59a538bfe418f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor> @ 0x10d3aea -> 0x1124f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: f52901c9f5cad4bf240b7156
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=288732; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Field> >; module=1440, inline_record=346492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 36b15b011cd9b3a9262a259c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1459, inline_record=660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 7aff3b152bc29c89522ec0dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1459, inline_record=52932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 9e517e18fd6f64af57f71fb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=287248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 4511ba358c6d1b83a383ac97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=112012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 9157bb95eba9b1d4788d6129
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::walls::wall_attachment::_::impl$0::deserialize::__Visitor> @ 0x10bd6d3 -> 0x1125050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::serialize
// 信息 ID: e1d064c44ac35bade0abb358
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=100, inline_record=177760; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallCornerAttachment>; module=1432, inline_record=142164; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1432, inline_record=149588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 0c3b8abd4e3f6a1d8ee287c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::wall_attachment::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor> @ 0x1575fb -> 0x80bc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor> @ 0x109c01b -> 0x1124d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$13::fmt
// 信息 ID: ef2a71eac10c8311224cf5d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$17::eq
// 信息 ID: 25fd714a675524ada0eb2103
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes; module=1309, inline_record=109284; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1370, inline_record=12204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$1::default
// 信息 ID: 6c3ca4d795a7e72e8c9fa1f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x80be3 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x80c7a -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::impl$0::add_decorator @ 0x503eab -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a39f -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a3aa -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0ac86 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9641 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9665 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x1988b07 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::set_duck_velocities_towards_goals @ 0x1b34118 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43cc3 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6de5b -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6de63 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorators_on_glade_unload @ 0x1c6ee51 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x1e6ab84 -> 0x3390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$2::fmt
// 信息 ID: 5dadb15309a6b2c47f080cfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>; module=1459, inline_record=2128; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>; module=3900, inline_record=780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$6::eq
// 信息 ID: b1d3ab5aec72382bae40d8d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes; module=1309, inline_record=109220; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1370, inline_record=12136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::impl$7::fmt
// 信息 ID: fcef391a301a4d0268a905f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=1459, inline_record=1708; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::walls::wall_attachment::WallAttachmentSide>; module=3900, inline_record=572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_attachment::is_default
// 信息 ID: a708d58e6001ffeb372c3d57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=60, inline_record=432; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=60, inline_record=488; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1459, inline_record=50536; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1459, inline_record=51256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
