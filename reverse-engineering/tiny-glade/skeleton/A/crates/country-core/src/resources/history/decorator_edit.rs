// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/history/decorator_edit.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e608080134ec90fc0e6190e312cf651e40783685a19c92fd9ea533e4ec5dbb6d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 55, 86, 100, 419, 427, 437, 444, 451, 484, 1308, 1309, 1321, 1323, 1334, 1370, 1385, 1389, 1395, 1432, 1439, 1440, 1459, 2756, 2817, 3386, 3844, 3845, 4651
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
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// use vectorize; // namespace candidate
//   package identities: [{"name":"vectorize","source":"PDB_registry_path_only","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 18, 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1439,"record_offset":38708,"rva":17498048,"va":5386207168}
// 编译实例 RVAs: 0x10affc0, 0x10b0520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16749745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207168,"raw_hex":"e8b194ff00","size":5,"soft_fail":false,"target_va":5402957088,"va":5386207338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1439,"record_offset":38912,"parent_record_offset":38708,"ancestor_record_offset":38708,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1439,"record_offset":38932,"parent_record_offset":38912,"ancestor_record_offset":38708,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1439,"record_offset":38952,"parent_record_offset":38932,"ancestor_record_offset":38708,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1439,"record_offset":38972,"parent_record_offset":38952,"ancestor_record_offset":38708,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 18, 29, 35, 71, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1042E
// 首条位置: {"module":1440,"inlinee":66606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":332428,"parent_record_offset":332400,"ancestor_record_offset":332400,"inlinee":66649,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":160992,"parent_record_offset":160968,"ancestor_record_offset":160968,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":161240,"parent_record_offset":161216,"ancestor_record_offset":160968,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":161216,"parent_record_offset":161160,"ancestor_record_offset":160968,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":161188,"parent_record_offset":161160,"ancestor_record_offset":160968,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":161160,"parent_record_offset":161136,"ancestor_record_offset":160968,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1440,"record_offset":332516,"parent_record_offset":332484,"ancestor_record_offset":332400,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":240224,"parent_record_offset":240196,"ancestor_record_offset":239984,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":161088,"parent_record_offset":161064,"ancestor_record_offset":160968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":240252,"parent_record_offset":240224,"ancestor_record_offset":239984,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":240280,"parent_record_offset":240252,"ancestor_record_offset":239984,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":240196,"parent_record_offset":240160,"ancestor_record_offset":239984,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1440,"record_offset":240160,"parent_record_offset":240108,"ancestor_record_offset":239984,"inlinee":66573,"source_line":382,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":161064,"parent_record_offset":161040,"ancestor_record_offset":160968,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1440,"record_offset":332456,"parent_record_offset":332428,"ancestor_record_offset":332400,"inlinee":66650,"source_line":615,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":161016,"parent_record_offset":160992,"ancestor_record_offset":160968,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":161040,"parent_record_offset":161016,"ancestor_record_offset":160968,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10409
// 首条位置: {"module":1439,"inlinee":66569}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":35556,"parent_record_offset":34904,"ancestor_record_offset":34904,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":34980,"parent_record_offset":34904,"ancestor_record_offset":34904,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":35016,"parent_record_offset":34904,"ancestor_record_offset":34904,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":34952,"parent_record_offset":34904,"ancestor_record_offset":34904,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":35044,"parent_record_offset":34904,"ancestor_record_offset":34904,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1439,"record_offset":35132,"parent_record_offset":35108,"ancestor_record_offset":34904,"inlinee":62216,"source_line":29,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":35108,"parent_record_offset":35072,"ancestor_record_offset":34904,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1439,"record_offset":35072,"parent_record_offset":35044,"ancestor_record_offset":34904,"inlinee":64997,"source_line":502,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1439,"record_offset":34096,"rva":17495248,"va":5386204368}
// 编译实例 RVAs: 0x10af4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t16755535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e84fabff00","size":5,"soft_fail":false,"target_va":5402960384,"va":5386204844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13455917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8d3ad32ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386205624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11964564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e86c6f49ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5386206831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11960907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8b57d49ff","size":5,"soft_fail":false,"target_va":5374243632,"va":5386204534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10312777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8b7a362ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386204644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10274807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8093863ff","size":5,"soft_fail":false,"target_va":5375930160,"va":5386204962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4b80
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10268258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e89e5163ff","size":5,"soft_fail":false,"target_va":5375937408,"va":5386205661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10217749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8eb1664ff","size":5,"soft_fail":false,"target_va":5375988576,"va":5386206320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141014070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-637608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e85845f6ff","size":5,"soft_fail":false,"target_va":5385568368,"va":5386205971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410175c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-623837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8237bf6ff","size":5,"soft_fail":false,"target_va":5385582016,"va":5386205848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141018100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-621169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e88f85f6ff","size":5,"soft_fail":false,"target_va":5385584896,"va":5386206060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410183b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-620164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e87c89f6ff","size":5,"soft_fail":false,"target_va":5385585584,"va":5386205743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d8340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t167169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8018d0200","size":5,"soft_fail":false,"target_va":5386371904,"va":5386204730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16751970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8629dff00","size":5,"soft_fail":false,"target_va":5402956592,"va":5386204617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16754934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204368,"raw_hex":"e8f6a8ff00","size":5,"soft_fail":false,"target_va":5402959760,"va":5386204821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1439,"record_offset":34288,"parent_record_offset":34096,"ancestor_record_offset":34096,"inlinee":25381,"source_line":255,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1439,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":34096,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1439,"record_offset":35992,"parent_record_offset":35900,"ancestor_record_offset":34096,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1439,"record_offset":36500,"parent_record_offset":36472,"ancestor_record_offset":34096,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1439,"record_offset":34504,"parent_record_offset":34476,"ancestor_record_offset":34096,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1439,"record_offset":34568,"parent_record_offset":34536,"ancestor_record_offset":34096,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1439,"record_offset":34628,"parent_record_offset":34596,"ancestor_record_offset":34096,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1439,"record_offset":34656,"parent_record_offset":34628,"ancestor_record_offset":34096,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":35928,"parent_record_offset":35900,"ancestor_record_offset":34096,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":35808,"parent_record_offset":35784,"ancestor_record_offset":34096,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":35900,"parent_record_offset":35876,"ancestor_record_offset":34096,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":35784,"parent_record_offset":35756,"ancestor_record_offset":34096,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":35756,"parent_record_offset":35724,"ancestor_record_offset":34096,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1439,"record_offset":36592,"parent_record_offset":36500,"ancestor_record_offset":34096,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1439,"record_offset":35956,"parent_record_offset":35928,"ancestor_record_offset":34096,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1439,"record_offset":34596,"parent_record_offset":34568,"ancestor_record_offset":34096,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":34876,"parent_record_offset":34288,"ancestor_record_offset":34096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1439,"record_offset":34736,"parent_record_offset":34712,"ancestor_record_offset":34096,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":36528,"parent_record_offset":36500,"ancestor_record_offset":34096,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":35556,"parent_record_offset":34904,"ancestor_record_offset":34096,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":34980,"parent_record_offset":34904,"ancestor_record_offset":34096,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":35016,"parent_record_offset":34904,"ancestor_record_offset":34096,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1439,"record_offset":34688,"parent_record_offset":34656,"ancestor_record_offset":34096,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1439,"record_offset":34712,"parent_record_offset":34688,"ancestor_record_offset":34096,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1439,"record_offset":35132,"parent_record_offset":35108,"ancestor_record_offset":34096,"inlinee":62216,"source_line":29,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1439,"record_offset":34904,"parent_record_offset":34288,"ancestor_record_offset":34096,"inlinee":66569,"source_line":18,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":34952,"parent_record_offset":34904,"ancestor_record_offset":34096,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":35044,"parent_record_offset":34904,"ancestor_record_offset":34096,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1439,"record_offset":35108,"parent_record_offset":35072,"ancestor_record_offset":34096,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1439,"record_offset":34476,"parent_record_offset":34288,"ancestor_record_offset":34096,"inlinee":25383,"source_line":877,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1439,"record_offset":34536,"parent_record_offset":34288,"ancestor_record_offset":34096,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1439,"record_offset":35072,"parent_record_offset":35044,"ancestor_record_offset":34096,"inlinee":64997,"source_line":502,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=10
// 源码行: 18, 35, 71, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":86,"record_offset":1088,"rva":1094944,"va":5369804064}
// 编译实例 RVAs: 0x10b520, 0x10b6a0, 0x10b7b0, 0x10ba80, 0x10bc70, 0x10af3c0, 0x10b0090, 0x10b0280, 0x10b0630, 0x10b07b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=4
//     target_aliases=13 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-997402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804064,"raw_hex":"e8e6c7f0ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5369804437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014cb50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t267345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804448,"raw_hex":"e851140400","size":5,"soft_fail":false,"target_va":5370071888,"va":5369804538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014f3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t277616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804448,"raw_hex":"e8703c0400","size":5,"soft_fail":false,"target_va":5370082272,"va":5369804651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401505d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t281832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e8e84c0400","size":5,"soft_fail":false,"target_va":5370086864,"va":5369805027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140150870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t282632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e808500400","size":5,"soft_fail":false,"target_va":5370087536,"va":5369804899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140150b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t282804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369805440,"raw_hex":"e8b4500400","size":5,"soft_fail":false,"target_va":5370088336,"va":5369805527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140152290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t289119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e85f690400","size":5,"soft_fail":false,"target_va":5370094224,"va":5369805100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140152b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t292323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804064,"raw_hex":"e8e3750400","size":5,"soft_fail":false,"target_va":5370096480,"va":5369804152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401531a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t292513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369805440,"raw_hex":"e8a1760400","size":5,"soft_fail":false,"target_va":5370098080,"va":5369805562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140153980
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t294068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369805936,"raw_hex":"e8b47c0400","size":5,"soft_fail":false,"target_va":5370100096,"va":5369806023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140153fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t296870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e8a6870400","size":5,"soft_fail":false,"target_va":5370101696,"va":5369804821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140154680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t298559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e83f8e0400","size":5,"soft_fail":false,"target_va":5370103424,"va":5369804860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140155010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t300929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e881970400","size":5,"soft_fail":false,"target_va":5370105872,"va":5369804938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t306357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e8b5ac0400","size":5,"soft_fail":false,"target_va":5370111424,"va":5369805062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2449516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804064,"raw_hex":"e86c602500","size":5,"soft_fail":false,"target_va":5372253808,"va":5369804287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2945391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804064,"raw_hex":"e86ff12c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369804316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405971e0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4766209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804720,"raw_hex":"e801ba4800","size":5,"soft_fail":false,"target_va":5374570976,"va":5369804762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=4
//     target_aliases=14 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11425324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207376,"raw_hex":"e8d4a951ff","size":5,"soft_fail":false,"target_va":5374782544,"va":5386207863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10315801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207376,"raw_hex":"e8e79762ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386207668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6221719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369804064,"raw_hex":"e897ef5e00","size":5,"soft_fail":false,"target_va":5376025984,"va":5369804260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410938d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204096,"raw_hex":"e84044feff","size":5,"soft_fail":false,"target_va":5386090704,"va":5386204299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141094650
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-110031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386204096,"raw_hex":"e83152feff","size":5,"soft_fail":false,"target_va":5386094160,"va":5386204186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410960d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-107129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e8875dfeff","size":5,"soft_fail":false,"target_va":5386100944,"va":5386208068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-104207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207376,"raw_hex":"e8f168feff","size":5,"soft_fail":false,"target_va":5386103296,"va":5386207498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-104317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e88368feff","size":5,"soft_fail":false,"target_va":5386103968,"va":5386208280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207376,"raw_hex":"e80e6efeff","size":5,"soft_fail":false,"target_va":5386104640,"va":5386207533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410980d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e8f47dfeff","size":5,"soft_fail":false,"target_va":5386109136,"va":5386207959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410990b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386208816,"raw_hex":"e8238afeff","size":5,"soft_fail":false,"target_va":5386113200,"va":5386208904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141099a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-92403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e80d97feff","size":5,"soft_fail":false,"target_va":5386115600,"va":5386207998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109c8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-80438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e8cac5feff","size":5,"soft_fail":false,"target_va":5386127600,"va":5386208033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109d290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-78302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e822cefeff","size":5,"soft_fail":false,"target_va":5386130064,"va":5386208361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109db60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-76972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386209200,"raw_hex":"e854d3feff","size":5,"soft_fail":false,"target_va":5386132320,"va":5386209287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109de80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-74988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207872,"raw_hex":"e814dbfeff","size":5,"soft_fail":false,"target_va":5386133120,"va":5386208103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109e440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-72876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386207376,"raw_hex":"e854e3feff","size":5,"soft_fail":false,"target_va":5386134592,"va":5386207463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":86,"record_offset":1312,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":86,"record_offset":1284,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1439,"record_offset":41080,"parent_record_offset":40216,"ancestor_record_offset":40216,"inlinee":24220,"source_line":132,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":86,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1088,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":86,"record_offset":1676,"parent_record_offset":1620,"ancestor_record_offset":1088,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":86,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":1088,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":86,"record_offset":1448,"parent_record_offset":1388,"ancestor_record_offset":1088,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":1088,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":1088,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":86,"record_offset":1480,"parent_record_offset":1388,"ancestor_record_offset":1088,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":86,"record_offset":1340,"parent_record_offset":1312,"ancestor_record_offset":1088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":86,"record_offset":1416,"parent_record_offset":1388,"ancestor_record_offset":1088,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00086.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$10::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF30A
// 首条位置: {"module":1334,"inlinee":62218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$11::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15EFA
// 首条位置: {"module":2756,"inlinee":89850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 26, 29, 32, 35, 71, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF308
// 首条位置: {"module":1334,"inlinee":62216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 26, 29, 32, 35, 71, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF0B0
// 首条位置: {"module":1439,"inlinee":61616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 26, 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E97
// 首条位置: {"module":55,"inlinee":7831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$13::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x103FE
// 首条位置: {"module":1439,"inlinee":66558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 29, 35, 71, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1042C
// 首条位置: {"module":1440,"inlinee":66604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":234324,"parent_record_offset":232004,"ancestor_record_offset":232004,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":274276,"parent_record_offset":271316,"ancestor_record_offset":271316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":234600,"parent_record_offset":232004,"ancestor_record_offset":232004,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":237344,"parent_record_offset":232004,"ancestor_record_offset":232004,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":159552,"parent_record_offset":159520,"ancestor_record_offset":159520,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":161524,"parent_record_offset":159520,"ancestor_record_offset":159520,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":160184,"parent_record_offset":160160,"ancestor_record_offset":159520,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":160160,"parent_record_offset":160104,"ancestor_record_offset":159520,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":160804,"parent_record_offset":160776,"ancestor_record_offset":159520,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":160864,"parent_record_offset":160804,"ancestor_record_offset":159520,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":159684,"parent_record_offset":159660,"ancestor_record_offset":159520,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":159732,"parent_record_offset":159708,"ancestor_record_offset":159520,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":159756,"parent_record_offset":159732,"ancestor_record_offset":159520,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":160132,"parent_record_offset":160104,"ancestor_record_offset":159520,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":160316,"parent_record_offset":160292,"ancestor_record_offset":159520,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":160104,"parent_record_offset":160076,"ancestor_record_offset":159520,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":160292,"parent_record_offset":160264,"ancestor_record_offset":159520,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":274132,"parent_record_offset":274068,"ancestor_record_offset":271316,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":160264,"parent_record_offset":160232,"ancestor_record_offset":159520,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":240224,"parent_record_offset":240196,"ancestor_record_offset":238468,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1440,"record_offset":274192,"parent_record_offset":274068,"ancestor_record_offset":271316,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":160896,"parent_record_offset":160804,"ancestor_record_offset":159520,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":161636,"parent_record_offset":161612,"ancestor_record_offset":159520,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":284612,"parent_record_offset":284584,"ancestor_record_offset":280924,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":284636,"parent_record_offset":284612,"ancestor_record_offset":280924,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":160464,"parent_record_offset":160436,"ancestor_record_offset":159520,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":159708,"parent_record_offset":159684,"ancestor_record_offset":159520,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":161612,"parent_record_offset":161580,"ancestor_record_offset":159520,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=101
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":159972,"parent_record_offset":159616,"ancestor_record_offset":159520,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":159832,"parent_record_offset":159808,"ancestor_record_offset":159520,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":284700,"parent_record_offset":284676,"ancestor_record_offset":280924,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":284676,"parent_record_offset":284552,"ancestor_record_offset":280924,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":160832,"parent_record_offset":160804,"ancestor_record_offset":159520,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":161692,"parent_record_offset":161636,"ancestor_record_offset":159520,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":234076,"parent_record_offset":232128,"ancestor_record_offset":232004,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":235328,"parent_record_offset":235276,"ancestor_record_offset":232004,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":240252,"parent_record_offset":240224,"ancestor_record_offset":238468,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":240280,"parent_record_offset":240252,"ancestor_record_offset":238468,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":159784,"parent_record_offset":159756,"ancestor_record_offset":159520,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":159808,"parent_record_offset":159784,"ancestor_record_offset":159520,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":240196,"parent_record_offset":240160,"ancestor_record_offset":238468,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1440,"record_offset":235228,"parent_record_offset":235204,"ancestor_record_offset":232004,"inlinee":63388,"source_line":26,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1440,"record_offset":235276,"parent_record_offset":235252,"ancestor_record_offset":232004,"inlinee":63381,"source_line":26,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1440,"record_offset":240160,"parent_record_offset":240108,"ancestor_record_offset":238468,"inlinee":66573,"source_line":382,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1440,"record_offset":160968,"parent_record_offset":160944,"ancestor_record_offset":159520,"inlinee":66606,"source_line":18,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1440,"record_offset":274044,"parent_record_offset":274020,"ancestor_record_offset":271316,"inlinee":66575,"source_line":354,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1440,"record_offset":236292,"parent_record_offset":236268,"ancestor_record_offset":232004,"inlinee":24935,"source_line":313,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1440,"record_offset":242716,"parent_record_offset":242692,"ancestor_record_offset":238468,"inlinee":64995,"source_line":84,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":284552,"parent_record_offset":284520,"ancestor_record_offset":280924,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":284484,"parent_record_offset":284456,"ancestor_record_offset":280924,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":284520,"parent_record_offset":284484,"ancestor_record_offset":280924,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":284944,"parent_record_offset":284916,"ancestor_record_offset":280924,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":284584,"parent_record_offset":284552,"ancestor_record_offset":280924,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":274372,"parent_record_offset":274348,"ancestor_record_offset":271316,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":237416,"parent_record_offset":237392,"ancestor_record_offset":232004,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":237392,"parent_record_offset":237368,"ancestor_record_offset":232004,"inlinee":6853,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":242900,"parent_record_offset":242876,"ancestor_record_offset":238468,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":284308,"parent_record_offset":284284,"ancestor_record_offset":280924,"inlinee":24938,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":237728,"parent_record_offset":237704,"ancestor_record_offset":232004,"inlinee":11607,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":237616,"parent_record_offset":237592,"ancestor_record_offset":232004,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":237840,"parent_record_offset":237816,"ancestor_record_offset":232004,"inlinee":24928,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":159584,"parent_record_offset":159552,"ancestor_record_offset":159520,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":161552,"parent_record_offset":161524,"ancestor_record_offset":159520,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":161064,"parent_record_offset":161040,"ancestor_record_offset":159520,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":161752,"parent_record_offset":161728,"ancestor_record_offset":159520,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":160944,"parent_record_offset":159616,"ancestor_record_offset":159520,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":274096,"parent_record_offset":274068,"ancestor_record_offset":271316,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":283848,"parent_record_offset":283824,"ancestor_record_offset":280924,"inlinee":24937,"source_line":1505,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":235648,"parent_record_offset":235624,"ancestor_record_offset":232004,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":236316,"parent_record_offset":236292,"ancestor_record_offset":232004,"inlinee":10276,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":234892,"parent_record_offset":234868,"ancestor_record_offset":232004,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":242740,"parent_record_offset":242716,"ancestor_record_offset":238468,"inlinee":10259,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":236656,"parent_record_offset":236632,"ancestor_record_offset":232004,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":159660,"parent_record_offset":159616,"ancestor_record_offset":159520,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":161016,"parent_record_offset":160992,"ancestor_record_offset":159520,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":160992,"parent_record_offset":160968,"ancestor_record_offset":159520,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":161040,"parent_record_offset":161016,"ancestor_record_offset":159520,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":235252,"parent_record_offset":235228,"ancestor_record_offset":232004,"inlinee":27028,"source_line":298,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":159616,"parent_record_offset":159584,"ancestor_record_offset":159520,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":161580,"parent_record_offset":161552,"ancestor_record_offset":159520,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1440,"record_offset":274068,"parent_record_offset":274044,"ancestor_record_offset":271316,"inlinee":26511,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$15::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10404
// 首条位置: {"module":1439,"inlinee":66564}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$17::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10400
// 首条位置: {"module":1439,"inlinee":66560}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 35, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1040D
// 首条位置: {"module":1440,"inlinee":66573}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":240196,"parent_record_offset":240160,"ancestor_record_offset":240160,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":240224,"parent_record_offset":240196,"ancestor_record_offset":240160,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":240252,"parent_record_offset":240224,"ancestor_record_offset":240160,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":240280,"parent_record_offset":240252,"ancestor_record_offset":240160,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 35, 71, 354, 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10413
// 首条位置: {"module":1440,"inlinee":66579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":9288,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":58132,"parent_record_offset":56344,"ancestor_record_offset":56344,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":8696,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":9712,"parent_record_offset":9684,"ancestor_record_offset":8660,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":9684,"parent_record_offset":9468,"ancestor_record_offset":8660,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":51760,"parent_record_offset":51696,"ancestor_record_offset":51440,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":8796,"parent_record_offset":8760,"ancestor_record_offset":8660,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1440,"record_offset":51820,"parent_record_offset":51696,"ancestor_record_offset":51440,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":8660,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":9584,"parent_record_offset":9560,"ancestor_record_offset":8660,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":8660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":51788,"parent_record_offset":51760,"ancestor_record_offset":51440,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":8660,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":9624,"parent_record_offset":9500,"ancestor_record_offset":8660,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":8856,"parent_record_offset":8796,"ancestor_record_offset":8660,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8660,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":8944,"parent_record_offset":8760,"ancestor_record_offset":8660,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":51852,"parent_record_offset":51672,"ancestor_record_offset":51440,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1440,"record_offset":57376,"parent_record_offset":57352,"ancestor_record_offset":56344,"inlinee":63388,"source_line":26,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1440,"record_offset":57424,"parent_record_offset":57400,"ancestor_record_offset":56344,"inlinee":63381,"source_line":26,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1440,"record_offset":51672,"parent_record_offset":51648,"ancestor_record_offset":51440,"inlinee":66588,"source_line":354,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1440,"record_offset":57088,"parent_record_offset":57064,"ancestor_record_offset":56344,"inlinee":24935,"source_line":313,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1440,"record_offset":127960,"parent_record_offset":127936,"ancestor_record_offset":127420,"inlinee":64995,"source_line":84,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":9500,"parent_record_offset":9468,"ancestor_record_offset":8660,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":9400,"parent_record_offset":9372,"ancestor_record_offset":8660,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":9468,"parent_record_offset":9400,"ancestor_record_offset":8660,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":9436,"parent_record_offset":9400,"ancestor_record_offset":8660,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":8660,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1440,"record_offset":9372,"parent_record_offset":9344,"ancestor_record_offset":8660,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":8728,"parent_record_offset":8696,"ancestor_record_offset":8660,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":8888,"parent_record_offset":8760,"ancestor_record_offset":8660,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":51724,"parent_record_offset":51696,"ancestor_record_offset":51440,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":8660,"inlinee":24937,"source_line":1505,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":58052,"parent_record_offset":58028,"ancestor_record_offset":56344,"inlinee":10276,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":57448,"parent_record_offset":57424,"ancestor_record_offset":56344,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$75::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":127984,"parent_record_offset":127960,"ancestor_record_offset":127420,"inlinee":10259,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":128272,"parent_record_offset":128248,"ancestor_record_offset":127420,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":57400,"parent_record_offset":57376,"ancestor_record_offset":56344,"inlinee":27028,"source_line":298,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":8760,"parent_record_offset":8728,"ancestor_record_offset":8660,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1440,"record_offset":51696,"parent_record_offset":51672,"ancestor_record_offset":51440,"inlinee":26511,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$20::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10406
// 首条位置: {"module":1439,"inlinee":66566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$21::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1B8
// 首条位置: {"module":1323,"inlinee":61880}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$22::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10402
// 首条位置: {"module":1439,"inlinee":66562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$24::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7B8
// 首条位置: {"module":1370,"inlinee":63416}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::color_id::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1370,"record_offset":12356,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":61513,"source_line":26,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":12300,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$34::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":12384,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":48739,"source_line":106,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$48::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":12328,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":27847,"source_line":154,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":11984,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":61591,"source_line":258,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$63::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":12080,"parent_record_offset":11984,"ancestor_record_offset":11940,"inlinee":61595,"source_line":307,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$68::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":12024,"parent_record_offset":11984,"ancestor_record_offset":11940,"inlinee":61593,"source_line":313,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$73::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1370,"record_offset":12240,"parent_record_offset":11984,"ancestor_record_offset":11940,"inlinee":61603,"source_line":319,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1370,"record_offset":12204,"parent_record_offset":12136,"ancestor_record_offset":11940,"inlinee":61601,"source_line":38,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1370,"record_offset":12136,"parent_record_offset":12080,"ancestor_record_offset":11940,"inlinee":61599,"source_line":6,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1370,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":11940,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1370,"record_offset":12168,"parent_record_offset":12136,"ancestor_record_offset":11940,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1370,"record_offset":12108,"parent_record_offset":12080,"ancestor_record_offset":11940,"inlinee":61597,"source_line":97,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$1::from
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x62A8
// 首条位置: {"module":437,"inlinee":25256}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF0A9
// 首条位置: {"module":1323,"inlinee":61609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1323,"record_offset":189248,"parent_record_offset":189224,"ancestor_record_offset":189224,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$1::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1323,"record_offset":189272,"parent_record_offset":189248,"ancestor_record_offset":189224,"inlinee":25256,"source_line":97,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$3::from
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E5B
// 首条位置: {"module":419,"inlinee":24155}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$4::from
// 记录形式: inlinee_declaration; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E7F
// 首条位置: {"module":1323,"inlinee":24191}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$25::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x609A
// 首条位置: {"module":427,"inlinee":24730}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":427,"record_offset":16084,"parent_record_offset":16056,"ancestor_record_offset":16056,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":427,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":16056,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":427,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":16056,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":427,"record_offset":16412,"parent_record_offset":16356,"ancestor_record_offset":16056,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":427,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":16056,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":427,"record_offset":16312,"parent_record_offset":16288,"ancestor_record_offset":16056,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":427,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":16056,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":427,"record_offset":16140,"parent_record_offset":16112,"ancestor_record_offset":16056,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":427,"record_offset":16228,"parent_record_offset":16196,"ancestor_record_offset":16056,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":427,"record_offset":16356,"parent_record_offset":16196,"ancestor_record_offset":16056,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":427,"record_offset":16112,"parent_record_offset":16084,"ancestor_record_offset":16056,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":427,"record_offset":16256,"parent_record_offset":16228,"ancestor_record_offset":16056,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":427,"record_offset":16288,"parent_record_offset":16256,"ancestor_record_offset":16056,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":427,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":16056,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":427,"record_offset":16168,"parent_record_offset":16140,"ancestor_record_offset":16056,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$27::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":427,"record_offset":67472,"rva":6017728,"va":5374726848}
// 编译实例 RVAs: 0x5bd2c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2857097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374726848,"raw_hex":"e889982b00","size":5,"soft_fail":false,"target_va":5377584016,"va":5374726914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$5::from
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6098
// 首条位置: {"module":427,"inlinee":24728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":427,"record_offset":15476,"parent_record_offset":15044,"ancestor_record_offset":15044,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":427,"record_offset":15072,"parent_record_offset":15044,"ancestor_record_offset":15044,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":427,"record_offset":15332,"parent_record_offset":15304,"ancestor_record_offset":15044,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":427,"record_offset":15392,"parent_record_offset":15360,"ancestor_record_offset":15044,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":427,"record_offset":15276,"parent_record_offset":15244,"ancestor_record_offset":15044,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":427,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":15044,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":427,"record_offset":15244,"parent_record_offset":15152,"ancestor_record_offset":15044,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":427,"record_offset":15124,"parent_record_offset":15096,"ancestor_record_offset":15044,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":427,"record_offset":15216,"parent_record_offset":15152,"ancestor_record_offset":15044,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":427,"record_offset":15188,"parent_record_offset":15152,"ancestor_record_offset":15044,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":427,"record_offset":15152,"parent_record_offset":15124,"ancestor_record_offset":15044,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":427,"record_offset":15304,"parent_record_offset":15244,"ancestor_record_offset":15044,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":427,"record_offset":15360,"parent_record_offset":15304,"ancestor_record_offset":15044,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$5::from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1BF
// 首条位置: {"module":1323,"inlinee":61887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1323,"record_offset":189200,"parent_record_offset":189136,"ancestor_record_offset":189136,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$52::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1323,"record_offset":189156,"parent_record_offset":189136,"ancestor_record_offset":189136,"inlinee":59537,"source_line":188,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1323,"record_offset":189248,"parent_record_offset":189224,"ancestor_record_offset":189136,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$1::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1323,"record_offset":189272,"parent_record_offset":189248,"ancestor_record_offset":189136,"inlinee":25256,"source_line":97,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1323,"record_offset":189224,"parent_record_offset":189200,"ancestor_record_offset":189136,"inlinee":61609,"source_line":112,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$7::from::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1BD
// 首条位置: {"module":1323,"inlinee":61885}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":437,"record_offset":16148,"rva":6160256,"va":5374869376}
// 编译实例 RVAs: 0x5dff80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5256397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e833cbafff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374869912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3219045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e89be1ceff","size":5,"soft_fail":false,"target_va":5371651744,"va":5374870784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2244457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e897c0ddff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374869476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406df980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1045192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e8c8f20f00","size":5,"soft_fail":false,"target_va":5375916416,"va":5374871219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbdcd0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7198699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e8ebd76d00","size":5,"soft_fail":false,"target_va":5382069456,"va":5374870752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df2630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8463747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e883258100","size":5,"soft_fail":false,"target_va":5383333424,"va":5374869672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df2f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8465515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e86b2c8100","size":5,"soft_fail":false,"target_va":5383335792,"va":5374870272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f34b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9783192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e898479500","size":5,"soft_fail":false,"target_va":5384653616,"va":5374870419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410ace20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11323370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e8eac7ac00","size":5,"soft_fail":false,"target_va":5386194464,"va":5374871089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420c8eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28215956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e8948aae01","size":5,"soft_fail":false,"target_va":5403086512,"va":5374870551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-152899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e8bdaafdff","size":5,"soft_fail":false,"target_va":5374717584,"va":5374870478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2496765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374869376,"raw_hex":"e8fd182600","size":5,"soft_fail":false,"target_va":5377366224,"va":5374869454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":437,"record_offset":22220,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":437,"record_offset":21676,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":437,"record_offset":18176,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":24748,"source_line":119,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":20344,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":20372,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":20428,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":19240,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":16452,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":9059,"source_line":232,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":20224,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":16912,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":17580,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":19600,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":24530,"source_line":2357,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$80::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":17272,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":18503,"source_line":3143,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":437,"record_offset":21808,"parent_record_offset":21772,"ancestor_record_offset":16148,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":437,"record_offset":21772,"parent_record_offset":21748,"ancestor_record_offset":16148,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":437,"record_offset":16744,"parent_record_offset":16720,"ancestor_record_offset":16148,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":437,"record_offset":16796,"parent_record_offset":16772,"ancestor_record_offset":16148,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":437,"record_offset":16772,"parent_record_offset":16720,"ancestor_record_offset":16148,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":437,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":16148,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":437,"record_offset":17124,"parent_record_offset":17096,"ancestor_record_offset":16148,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":437,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":16148,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":437,"record_offset":17184,"parent_record_offset":17152,"ancestor_record_offset":16148,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":437,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":16148,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":437,"record_offset":17360,"parent_record_offset":17336,"ancestor_record_offset":16148,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":437,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":16148,"inlinee":6053,"source_line":85,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":437,"record_offset":17336,"parent_record_offset":17300,"ancestor_record_offset":16148,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":437,"record_offset":16820,"parent_record_offset":16796,"ancestor_record_offset":16148,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":437,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":16148,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":437,"record_offset":18208,"parent_record_offset":18176,"ancestor_record_offset":16148,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":437,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":16148,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":437,"record_offset":17220,"parent_record_offset":17068,"ancestor_record_offset":16148,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":437,"record_offset":21700,"parent_record_offset":21676,"ancestor_record_offset":16148,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":437,"record_offset":18012,"parent_record_offset":17988,"ancestor_record_offset":16148,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":437,"record_offset":17988,"parent_record_offset":17964,"ancestor_record_offset":16148,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":437,"record_offset":21840,"parent_record_offset":21808,"ancestor_record_offset":16148,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":437,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":16148,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":437,"record_offset":21868,"parent_record_offset":21808,"ancestor_record_offset":16148,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":437,"record_offset":21900,"parent_record_offset":21772,"ancestor_record_offset":16148,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":437,"record_offset":22244,"parent_record_offset":22220,"ancestor_record_offset":16148,"inlinee":23940,"source_line":34,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":437,"record_offset":19044,"parent_record_offset":19020,"ancestor_record_offset":16148,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":437,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":16148,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":18284,"parent_record_offset":18260,"ancestor_record_offset":16148,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":18260,"parent_record_offset":18236,"ancestor_record_offset":16148,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":16148,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":16960,"parent_record_offset":16936,"ancestor_record_offset":16148,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":20248,"parent_record_offset":20224,"ancestor_record_offset":16148,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":20276,"parent_record_offset":20248,"ancestor_record_offset":16148,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":17504,"parent_record_offset":17300,"ancestor_record_offset":16148,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":18972,"parent_record_offset":18752,"ancestor_record_offset":16148,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":17436,"parent_record_offset":17412,"ancestor_record_offset":16148,"inlinee":16170,"source_line":4742,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":16148,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$94::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":17300,"parent_record_offset":17272,"ancestor_record_offset":16148,"inlinee":18288,"source_line":6499,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":437,"record_offset":18316,"parent_record_offset":18284,"ancestor_record_offset":16148,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18884,"parent_record_offset":18860,"ancestor_record_offset":16148,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18860,"parent_record_offset":18752,"ancestor_record_offset":16148,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18800,"parent_record_offset":18776,"ancestor_record_offset":16148,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17964,"parent_record_offset":17656,"ancestor_record_offset":16148,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":19180,"parent_record_offset":18392,"ancestor_record_offset":16148,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17068,"parent_record_offset":17008,"ancestor_record_offset":16148,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17656,"parent_record_offset":17628,"ancestor_record_offset":16148,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18776,"parent_record_offset":18752,"ancestor_record_offset":16148,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":16148,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18344,"parent_record_offset":18284,"ancestor_record_offset":16148,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":16984,"parent_record_offset":16960,"ancestor_record_offset":16148,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17532,"parent_record_offset":17504,"ancestor_record_offset":16148,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":20556,"parent_record_offset":20500,"ancestor_record_offset":16148,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17040,"parent_record_offset":17008,"ancestor_record_offset":16148,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18392,"parent_record_offset":18368,"ancestor_record_offset":16148,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17008,"parent_record_offset":16984,"ancestor_record_offset":16148,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":437,"record_offset":17736,"parent_record_offset":17704,"ancestor_record_offset":16148,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":437,"record_offset":17768,"parent_record_offset":17736,"ancestor_record_offset":16148,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":437,"record_offset":18720,"parent_record_offset":18604,"ancestor_record_offset":16148,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":437,"record_offset":17704,"parent_record_offset":17656,"ancestor_record_offset":16148,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18436,"parent_record_offset":18392,"ancestor_record_offset":16148,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":16148,"inlinee":16171,"source_line":4151,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":16148,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":437,"record_offset":18752,"parent_record_offset":18392,"ancestor_record_offset":16148,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":437,"record_offset":18464,"parent_record_offset":18392,"ancestor_record_offset":16148,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":437,"record_offset":17096,"parent_record_offset":17068,"ancestor_record_offset":16148,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":437,"record_offset":18548,"parent_record_offset":18392,"ancestor_record_offset":16148,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":437,"record_offset":19088,"parent_record_offset":18392,"ancestor_record_offset":16148,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":437,"record_offset":17152,"parent_record_offset":17096,"ancestor_record_offset":16148,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":18236,"parent_record_offset":18176,"ancestor_record_offset":16148,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":16936,"parent_record_offset":16912,"ancestor_record_offset":16148,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":17412,"parent_record_offset":17360,"ancestor_record_offset":16148,"inlinee":16169,"source_line":2245,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":19624,"parent_record_offset":19600,"ancestor_record_offset":16148,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":437,"record_offset":16476,"parent_record_offset":16452,"ancestor_record_offset":16148,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":437,"record_offset":16532,"parent_record_offset":16500,"ancestor_record_offset":16148,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":437,"record_offset":16608,"parent_record_offset":16580,"ancestor_record_offset":16148,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":437,"record_offset":16720,"parent_record_offset":16580,"ancestor_record_offset":16148,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":437,"record_offset":16500,"parent_record_offset":16476,"ancestor_record_offset":16148,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":437,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":16148,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":437,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":16148,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":437,"record_offset":16580,"parent_record_offset":16556,"ancestor_record_offset":16148,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":437,"record_offset":16556,"parent_record_offset":16532,"ancestor_record_offset":16148,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00437.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1C3
// 首条位置: {"module":1323,"inlinee":61891}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1323,"record_offset":192568,"parent_record_offset":191308,"ancestor_record_offset":191308,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::impl$21::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1323,"record_offset":192668,"parent_record_offset":191308,"ancestor_record_offset":191308,"inlinee":61880,"source_line":71,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorIdGen::reserve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1323,"record_offset":192640,"parent_record_offset":191308,"ancestor_record_offset":191308,"inlinee":24249,"source_line":460,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1323,"record_offset":191356,"parent_record_offset":191308,"ancestor_record_offset":191308,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1323,"record_offset":192704,"parent_record_offset":191308,"ancestor_record_offset":191308,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1323,"record_offset":192040,"parent_record_offset":192016,"ancestor_record_offset":191308,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1323,"record_offset":191696,"parent_record_offset":191668,"ancestor_record_offset":191308,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1323,"record_offset":191860,"parent_record_offset":191804,"ancestor_record_offset":191308,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1323,"record_offset":192800,"parent_record_offset":192776,"ancestor_record_offset":191308,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1323,"record_offset":193028,"parent_record_offset":193004,"ancestor_record_offset":191308,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1323,"record_offset":193004,"parent_record_offset":192980,"ancestor_record_offset":191308,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1323,"record_offset":192956,"parent_record_offset":192800,"ancestor_record_offset":191308,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":192884,"parent_record_offset":192856,"ancestor_record_offset":191308,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":192912,"parent_record_offset":192884,"ancestor_record_offset":191308,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":192856,"parent_record_offset":192832,"ancestor_record_offset":191308,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":193180,"parent_record_offset":193156,"ancestor_record_offset":191308,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":193108,"parent_record_offset":193084,"ancestor_record_offset":191308,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":193204,"parent_record_offset":193180,"ancestor_record_offset":191308,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":193252,"parent_record_offset":193204,"ancestor_record_offset":191308,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1323,"record_offset":193644,"parent_record_offset":193204,"ancestor_record_offset":191308,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1323,"record_offset":191724,"parent_record_offset":191696,"ancestor_record_offset":191308,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1323,"record_offset":193052,"parent_record_offset":193028,"ancestor_record_offset":191308,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1323,"record_offset":192272,"parent_record_offset":192244,"ancestor_record_offset":191308,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1323,"record_offset":193352,"parent_record_offset":193252,"ancestor_record_offset":191308,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1323,"record_offset":193320,"parent_record_offset":193252,"ancestor_record_offset":191308,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1323,"record_offset":192176,"parent_record_offset":192152,"ancestor_record_offset":191308,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1323,"record_offset":192400,"parent_record_offset":192372,"ancestor_record_offset":191308,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1323,"record_offset":192372,"parent_record_offset":192344,"ancestor_record_offset":191308,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1323,"record_offset":192492,"parent_record_offset":192468,"ancestor_record_offset":191308,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1323,"record_offset":192516,"parent_record_offset":192492,"ancestor_record_offset":191308,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1323,"record_offset":192592,"parent_record_offset":192568,"ancestor_record_offset":191308,"inlinee":24155,"source_line":118,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$22::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1323,"record_offset":192980,"parent_record_offset":192956,"ancestor_record_offset":191308,"inlinee":21684,"source_line":84,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1323,"record_offset":194872,"parent_record_offset":194848,"ancestor_record_offset":191308,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1323,"record_offset":195040,"parent_record_offset":192736,"ancestor_record_offset":191308,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1323,"record_offset":192736,"parent_record_offset":192704,"ancestor_record_offset":191308,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1323,"record_offset":192776,"parent_record_offset":192736,"ancestor_record_offset":191308,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194760,"parent_record_offset":194736,"ancestor_record_offset":191308,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194736,"parent_record_offset":194628,"ancestor_record_offset":191308,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":192344,"parent_record_offset":192316,"ancestor_record_offset":191308,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":192468,"parent_record_offset":191504,"ancestor_record_offset":191308,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191900,"parent_record_offset":191588,"ancestor_record_offset":191308,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194652,"parent_record_offset":194628,"ancestor_record_offset":191308,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194212,"parent_record_offset":192736,"ancestor_record_offset":191308,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191504,"parent_record_offset":191476,"ancestor_record_offset":191308,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":195068,"parent_record_offset":195040,"ancestor_record_offset":191308,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":192316,"parent_record_offset":191504,"ancestor_record_offset":191308,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194236,"parent_record_offset":194212,"ancestor_record_offset":191308,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191588,"parent_record_offset":191556,"ancestor_record_offset":191308,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191764,"parent_record_offset":191588,"ancestor_record_offset":191308,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191928,"parent_record_offset":191588,"ancestor_record_offset":191308,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191960,"parent_record_offset":191928,"ancestor_record_offset":191308,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191556,"parent_record_offset":191504,"ancestor_record_offset":191308,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191640,"parent_record_offset":191588,"ancestor_record_offset":191308,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":192244,"parent_record_offset":192220,"ancestor_record_offset":191308,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":192220,"parent_record_offset":191556,"ancestor_record_offset":191308,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1323,"record_offset":192128,"parent_record_offset":191928,"ancestor_record_offset":191308,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1323,"record_offset":192152,"parent_record_offset":192128,"ancestor_record_offset":191308,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1323,"record_offset":194596,"parent_record_offset":194480,"ancestor_record_offset":191308,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1323,"record_offset":194480,"parent_record_offset":194236,"ancestor_record_offset":191308,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194284,"parent_record_offset":194236,"ancestor_record_offset":191308,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":194628,"parent_record_offset":194236,"ancestor_record_offset":191308,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":191984,"parent_record_offset":191960,"ancestor_record_offset":191308,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1323,"record_offset":192440,"parent_record_offset":191504,"ancestor_record_offset":191308,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1323,"record_offset":191668,"parent_record_offset":191588,"ancestor_record_offset":191308,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1323,"record_offset":192016,"parent_record_offset":191928,"ancestor_record_offset":191308,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1323,"record_offset":194424,"parent_record_offset":194236,"ancestor_record_offset":191308,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1323,"record_offset":194908,"parent_record_offset":194236,"ancestor_record_offset":191308,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1323,"record_offset":191804,"parent_record_offset":191764,"ancestor_record_offset":191308,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1323,"record_offset":191416,"parent_record_offset":191392,"ancestor_record_offset":191308,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1323,"record_offset":191476,"parent_record_offset":191448,"ancestor_record_offset":191308,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1323,"record_offset":194848,"parent_record_offset":194628,"ancestor_record_offset":191308,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1323,"record_offset":191392,"parent_record_offset":191356,"ancestor_record_offset":191308,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1323,"record_offset":191448,"parent_record_offset":191356,"ancestor_record_offset":191308,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1323,"record_offset":192832,"parent_record_offset":192800,"ancestor_record_offset":191308,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1323,"record_offset":193156,"parent_record_offset":192800,"ancestor_record_offset":191308,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1323,"record_offset":193084,"parent_record_offset":193028,"ancestor_record_offset":191308,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1439,"record_offset":8972,"rva":17485344,"va":5386194464}
// 编译实例 RVAs: 0x10ace20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13446678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386194464,"raw_hex":"e8ead132ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386196385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-14885146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386194464,"raw_hex":"e8e6de1cff","size":5,"soft_fail":false,"target_va":5371309936,"va":5386195077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16782862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386194464,"raw_hex":"e80e160001","size":5,"soft_fail":false,"target_va":5402979152,"va":5386196285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3294521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386194464,"raw_hex":"e8c7bacdff","size":5,"soft_fail":false,"target_va":5382901040,"va":5386195556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t16768848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386194464,"raw_hex":"e850dfff00","size":5,"soft_fail":false,"target_va":5402963904,"va":5386195051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t16769939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386194464,"raw_hex":"e893e3ff00","size":5,"soft_fail":false,"target_va":5402965344,"va":5386195400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-13446496","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5386194464,"raw_hex":"e9a0d232ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386196203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":12852,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_pop_bucket.rs
//     evidence={"module":1439,"record_offset":10652,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":66568,"source_line":41,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1439,"record_offset":9228,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":21719,"source_line":2223,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$46::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1439,"record_offset":9980,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":21721,"source_line":2315,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1439,"record_offset":12976,"parent_record_offset":12952,"ancestor_record_offset":8972,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1439,"record_offset":12952,"parent_record_offset":12924,"ancestor_record_offset":8972,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":10968,"parent_record_offset":10936,"ancestor_record_offset":8972,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":10936,"parent_record_offset":10896,"ancestor_record_offset":8972,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":10896,"parent_record_offset":10688,"ancestor_record_offset":8972,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1439,"record_offset":9476,"parent_record_offset":9448,"ancestor_record_offset":8972,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1439,"record_offset":9536,"parent_record_offset":9504,"ancestor_record_offset":8972,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1439,"record_offset":9832,"parent_record_offset":9696,"ancestor_record_offset":8972,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1439,"record_offset":9724,"parent_record_offset":9696,"ancestor_record_offset":8972,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1439,"record_offset":9796,"parent_record_offset":9724,"ancestor_record_offset":8972,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1439,"record_offset":9764,"parent_record_offset":9724,"ancestor_record_offset":8972,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1439,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":8972,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1439,"record_offset":9572,"parent_record_offset":9416,"ancestor_record_offset":8972,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":11032,"parent_record_offset":10688,"ancestor_record_offset":8972,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1439,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":8972,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1439,"record_offset":9904,"parent_record_offset":9636,"ancestor_record_offset":8972,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":11000,"parent_record_offset":10896,"ancestor_record_offset":8972,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":10832,"parent_record_offset":10748,"ancestor_record_offset":8972,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":8972,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":11112,"parent_record_offset":11080,"ancestor_record_offset":8972,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":10776,"parent_record_offset":10748,"ancestor_record_offset":8972,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":8972,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":8972,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1439,"record_offset":10688,"parent_record_offset":10652,"ancestor_record_offset":8972,"inlinee":24244,"source_line":430,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$59::into_iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1439,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":8972,"inlinee":19625,"source_line":4716,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$67::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1439,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":8972,"inlinee":21711,"source_line":4814,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":10436,"parent_record_offset":10080,"ancestor_record_offset":8972,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9416,"parent_record_offset":9332,"ancestor_record_offset":8972,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":10080,"parent_record_offset":10052,"ancestor_record_offset":8972,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":8972,"inlinee":19590,"source_line":1667,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9612,"parent_record_offset":9276,"ancestor_record_offset":8972,"inlinee":19589,"source_line":1653,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":8972,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9388,"parent_record_offset":9332,"ancestor_record_offset":8972,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9360,"parent_record_offset":9332,"ancestor_record_offset":8972,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9332,"parent_record_offset":9304,"ancestor_record_offset":8972,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9696,"parent_record_offset":9636,"ancestor_record_offset":8972,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1439,"record_offset":10168,"parent_record_offset":10128,"ancestor_record_offset":8972,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1439,"record_offset":10204,"parent_record_offset":10168,"ancestor_record_offset":8972,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1439,"record_offset":10264,"parent_record_offset":10128,"ancestor_record_offset":8972,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1439,"record_offset":10128,"parent_record_offset":10080,"ancestor_record_offset":8972,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":9276,"parent_record_offset":9252,"ancestor_record_offset":8972,"inlinee":19588,"source_line":3706,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":8972,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$38::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":8972,"inlinee":19627,"source_line":4312,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1439,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":8972,"inlinee":19629,"source_line":4344,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1439,"record_offset":9448,"parent_record_offset":9416,"ancestor_record_offset":8972,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1439,"record_offset":9504,"parent_record_offset":9448,"ancestor_record_offset":8972,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1439,"record_offset":10724,"parent_record_offset":10688,"ancestor_record_offset":8972,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1439,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":8972,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1439,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":8972,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1439,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":8972,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
// 记录形式: compiled_procedure; instances=1
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1309,"record_offset":108816,"rva":14191920,"va":5382901040}
// 编译实例 RVAs: 0xd88d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1309,"record_offset":108988,"parent_record_offset":108816,"ancestor_record_offset":108816,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$48::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1309,"record_offset":109408,"parent_record_offset":109384,"ancestor_record_offset":108816,"inlinee":27847,"source_line":154,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1309,"record_offset":109072,"parent_record_offset":109040,"ancestor_record_offset":108816,"inlinee":61591,"source_line":258,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$63::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1309,"record_offset":109164,"parent_record_offset":109072,"ancestor_record_offset":108816,"inlinee":61595,"source_line":307,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$68::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1309,"record_offset":109108,"parent_record_offset":109072,"ancestor_record_offset":108816,"inlinee":61593,"source_line":313,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$73::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1309,"record_offset":109320,"parent_record_offset":109072,"ancestor_record_offset":108816,"inlinee":61603,"source_line":319,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1309,"record_offset":109012,"parent_record_offset":108988,"ancestor_record_offset":108816,"inlinee":23940,"source_line":34,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1309,"record_offset":109284,"parent_record_offset":109220,"ancestor_record_offset":108816,"inlinee":61601,"source_line":38,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1309,"record_offset":109220,"parent_record_offset":109164,"ancestor_record_offset":108816,"inlinee":61599,"source_line":6,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1309,"record_offset":109132,"parent_record_offset":109108,"ancestor_record_offset":108816,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1309,"record_offset":109252,"parent_record_offset":109220,"ancestor_record_offset":108816,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1309,"record_offset":109192,"parent_record_offset":109164,"ancestor_record_offset":108816,"inlinee":61597,"source_line":97,"archive":"_pdb/A/modules/01309.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1040F
// 首条位置: {"module":1440,"inlinee":66575}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":274224,"parent_record_offset":274044,"ancestor_record_offset":274044,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1440,"record_offset":274068,"parent_record_offset":274044,"ancestor_record_offset":274044,"inlinee":26511,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":274132,"parent_record_offset":274068,"ancestor_record_offset":274044,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1440,"record_offset":274192,"parent_record_offset":274068,"ancestor_record_offset":274044,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":274160,"parent_record_offset":274132,"ancestor_record_offset":274044,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":274096,"parent_record_offset":274068,"ancestor_record_offset":274044,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1041C
// 首条位置: {"module":1440,"inlinee":66588}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":51852,"parent_record_offset":51672,"ancestor_record_offset":51672,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1440,"record_offset":51696,"parent_record_offset":51672,"ancestor_record_offset":51672,"inlinee":26511,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":51760,"parent_record_offset":51696,"ancestor_record_offset":51672,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1440,"record_offset":51820,"parent_record_offset":51696,"ancestor_record_offset":51672,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":51788,"parent_record_offset":51760,"ancestor_record_offset":51672,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":51724,"parent_record_offset":51696,"ancestor_record_offset":51672,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EAB
// 首条位置: {"module":55,"inlinee":7851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$29::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF314
// 首条位置: {"module":1334,"inlinee":62228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1334,"record_offset":63892,"parent_record_offset":63872,"ancestor_record_offset":63872,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1334,"record_offset":64144,"parent_record_offset":64120,"ancestor_record_offset":63872,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1334,"record_offset":64196,"parent_record_offset":64172,"ancestor_record_offset":63872,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1334,"record_offset":64172,"parent_record_offset":64120,"ancestor_record_offset":63872,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1334,"record_offset":64220,"parent_record_offset":64196,"ancestor_record_offset":63872,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1334,"record_offset":64080,"parent_record_offset":64056,"ancestor_record_offset":63872,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1334,"record_offset":64244,"parent_record_offset":64220,"ancestor_record_offset":63872,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1334,"record_offset":63932,"parent_record_offset":63912,"ancestor_record_offset":63872,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1334,"record_offset":64008,"parent_record_offset":63980,"ancestor_record_offset":63872,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1334,"record_offset":64120,"parent_record_offset":63980,"ancestor_record_offset":63872,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1334,"record_offset":63912,"parent_record_offset":63892,"ancestor_record_offset":63872,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1334,"record_offset":64032,"parent_record_offset":64008,"ancestor_record_offset":63872,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1334,"record_offset":64056,"parent_record_offset":64032,"ancestor_record_offset":63872,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1334,"record_offset":63980,"parent_record_offset":63956,"ancestor_record_offset":63872,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1334,"record_offset":63956,"parent_record_offset":63932,"ancestor_record_offset":63872,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01334.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$31::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15FFB
// 首条位置: {"module":2817,"inlinee":90107}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2817,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":16044,"inlinee":10178,"source_line":1269,"archive":"_pdb/A/modules/02817.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2817,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":16044,"inlinee":10180,"source_line":196,"archive":"_pdb/A/modules/02817.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x63BE
// 首条位置: {"module":444,"inlinee":25534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":444,"record_offset":10988,"parent_record_offset":10968,"ancestor_record_offset":10968,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":444,"record_offset":11008,"parent_record_offset":10988,"ancestor_record_offset":10968,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/00444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6ABA
// 首条位置: {"module":484,"inlinee":27322}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::flat_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":24748,"parent_record_offset":24280,"ancestor_record_offset":24280,"inlinee":16079,"source_line":1456,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":484,"record_offset":24304,"parent_record_offset":24280,"ancestor_record_offset":24280,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":484,"record_offset":24600,"parent_record_offset":24572,"ancestor_record_offset":24280,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":484,"record_offset":24660,"parent_record_offset":24628,"ancestor_record_offset":24280,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::FlatMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":484,"record_offset":24776,"parent_record_offset":24748,"ancestor_record_offset":24280,"inlinee":16081,"source_line":24,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":24544,"parent_record_offset":24508,"ancestor_record_offset":24280,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":484,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":24280,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":484,"record_offset":24508,"parent_record_offset":24416,"ancestor_record_offset":24280,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":484,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":24280,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":484,"record_offset":24480,"parent_record_offset":24416,"ancestor_record_offset":24280,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":484,"record_offset":24452,"parent_record_offset":24416,"ancestor_record_offset":24280,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":484,"record_offset":24416,"parent_record_offset":24388,"ancestor_record_offset":24280,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":484,"record_offset":24572,"parent_record_offset":24508,"ancestor_record_offset":24280,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":484,"record_offset":24628,"parent_record_offset":24572,"ancestor_record_offset":24280,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":484,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":24280,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF081
// 首条位置: {"module":1308,"inlinee":61569}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1308,"record_offset":340676,"parent_record_offset":340636,"ancestor_record_offset":340636,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1308,"record_offset":340992,"parent_record_offset":340964,"ancestor_record_offset":340636,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1308,"record_offset":341052,"parent_record_offset":341020,"ancestor_record_offset":340636,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1308,"record_offset":340936,"parent_record_offset":340904,"ancestor_record_offset":340636,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1308,"record_offset":340740,"parent_record_offset":340708,"ancestor_record_offset":340636,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1308,"record_offset":340904,"parent_record_offset":340804,"ancestor_record_offset":340636,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1308,"record_offset":340772,"parent_record_offset":340740,"ancestor_record_offset":340636,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1308,"record_offset":340876,"parent_record_offset":340804,"ancestor_record_offset":340636,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1308,"record_offset":340844,"parent_record_offset":340804,"ancestor_record_offset":340636,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1308,"record_offset":340804,"parent_record_offset":340772,"ancestor_record_offset":340636,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1308,"record_offset":340964,"parent_record_offset":340904,"ancestor_record_offset":340636,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1308,"record_offset":341020,"parent_record_offset":340964,"ancestor_record_offset":340636,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1308,"record_offset":340708,"parent_record_offset":340676,"ancestor_record_offset":340636,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 376
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFD8A
// 首条位置: {"module":1395,"inlinee":64906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1395,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1395,"record_offset":7488,"parent_record_offset":7464,"ancestor_record_offset":6764,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1395,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6764,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1395,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6764,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1395,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6764,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1395,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":6764,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1395,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":6764,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1395,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":6764,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1395,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":6764,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1395,"record_offset":7464,"parent_record_offset":7440,"ancestor_record_offset":6764,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1395,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":6764,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1395,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":6764,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1395,"record_offset":7440,"parent_record_offset":7304,"ancestor_record_offset":6764,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1395,"record_offset":6868,"parent_record_offset":6836,"ancestor_record_offset":6764,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":6764,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":6764,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":7648,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":6764,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":6896,"parent_record_offset":6836,"ancestor_record_offset":6764,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":6764,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1395,"record_offset":7184,"parent_record_offset":7156,"ancestor_record_offset":6764,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1395,"record_offset":7212,"parent_record_offset":7184,"ancestor_record_offset":6764,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1395,"record_offset":7272,"parent_record_offset":7156,"ancestor_record_offset":6764,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1395,"record_offset":7156,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":6988,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1395,"record_offset":7304,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1395,"record_offset":7016,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1395,"record_offset":7100,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1395,"record_offset":7556,"parent_record_offset":6944,"ancestor_record_offset":6764,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$34::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1BA
// 首条位置: {"module":1323,"inlinee":61882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1323,"record_offset":30924,"parent_record_offset":30888,"ancestor_record_offset":30888,"inlinee":10178,"source_line":1269,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1323,"record_offset":30948,"parent_record_offset":30924,"ancestor_record_offset":30888,"inlinee":10180,"source_line":196,"archive":"_pdb/A/modules/01323.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryDiff
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>", "constructor": "core::mem::maybe_uninit::MaybeUninit", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$15::new_uninit_in::closure_env$0<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::impl$15::new_uninit_in::closure_env$0", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff> >", "constructor": "alloc::sync::ArcInner", "arguments": ["core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ptr_mut$<alloc::sync::ArcInner<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff> > >", "constructor": "ptr_mut$", "arguments": ["alloc::sync::ArcInner<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::allocate_for_layout<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>,alloc::sync::impl$15::new_uninit_in::closure_env$0<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>,ptr_mut$<alloc::sync::ArcInner<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff> > > (*)(ptr_mut$<u8>)>", "constructor": "alloc::sync::Arc::allocate_for_layout", "arguments": ["core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>", "alloc::sync::impl$15::new_uninit_in::closure_env$0<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "ptr_mut$<alloc::sync::ArcInner<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff> > > (*)(ptr_mut$<u8>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["core::mem::maybe_uninit::MaybeUninit<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$52::default<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>", "constructor": "alloc::sync::impl$52::default", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$50::fmt<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$50::fmt", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::extend_from_slice<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::extend_from_slice", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditChange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditChange"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditChange"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>", "constructor": "ref$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace> >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 28
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "ref$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "ref$", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > >", "constructor": "serde::ser::Serializer::collect_map", "arguments": ["serde_json::value::ser::Serializer", "ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState>", "ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$65::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >", "constructor": "alloc::sync::impl$65::to_arc_slice", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$79::from_iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >", "constructor": "std::collections::hash::map::impl$79::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$4::take_snapshot::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$4::take_snapshot::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >", "constructor": "hashbrown::map::impl$94::extend::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >", "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "tuple$<>", "country_core::resources::history::decorator_edit::impl$5::from::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >", "constructor": "hashbrown::map::impl$60::fold::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator::DecoratorInfo", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>", "hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "tuple$", "arguments": ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > >", "constructor": "hashbrown::map::impl$94::extend::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator::DecoratorInfo", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>", "hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>", "tuple$<>", "country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >", "constructor": "hashbrown::map::impl$60::fold::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::walls::decorator_storage::impl$4::restore_snapshot::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "country_core::resources::walls::decorator_storage::impl$4::restore_snapshot::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::walls::decorator_storage::impl$4::restore_snapshot::closure_env$0> >", "constructor": "hashbrown::map::impl$94::extend", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator::DecoratorInfo", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::walls::decorator_storage::impl$4::restore_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "serde::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_tuple<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_json::value::de::impl$2::deserialize_tuple", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "ref$", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "ref$", "arguments": ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > >", "constructor": "ref$", "arguments": ["ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >", "constructor": "core::iter::adapters::copied::impl$1::try_fold", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "country_core::resources::walls::decorator::DecoratorId", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0> >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_object<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::visit_object", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "constructor": "tuple$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState", "country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$79::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >", "constructor": "std::collections::hash::map::impl$79::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$79::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "std::collections::hash::map::impl$79::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >", "constructor": "hashbrown::map::impl$94::extend::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >", "constructor": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "tuple$<>", "country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1>,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>", "core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1>", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawIntoIter", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0,tuple$<>,ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>", "country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0", "tuple$<>", "ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>", "country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIter::drop_elements<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> > >", "constructor": "hashbrown::raw::RawIter::drop_elements", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "hashbrown::map::impl$94::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 36
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "ref$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "tuple$", "arguments": ["ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >", "ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "ref$", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > >", "constructor": "ref$", "arguments": ["ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > > >", "constructor": "ref$", "arguments": ["ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "std::collections::hash::map::Iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "ref$", "arguments": ["std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "vectorize::serialize<ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >,enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,serde_json::value::ser::Serializer>", "constructor": "vectorize::serialize", "arguments": ["ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "serde_json::value::ser::Serializer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "std::collections::hash::map::Values", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$65::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >", "constructor": "alloc::sync::impl$65::to_arc_slice", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$79::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::impl$79::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$79::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "std::collections::hash::map::impl$79::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "ref_mut$", "arguments": ["hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["usize", "ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "hashbrown::scopeguard::ScopeGuard", "arguments": ["tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$94::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "hashbrown::map::impl$94::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "serde::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_tuple<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_json::value::de::impl$2::deserialize_tuple", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde::de::impl$1::fmt", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::decorator_edit::DecoratorHistoryEdit", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 16; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list
// 信息 ID: f0345777dd0d788a57f2425c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=24280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty
// 信息 ID: f0cee95f181dc25f93e65fad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::is_decorator_history_diff_empty; module=444, inline_record=10968; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::finish_input_recording; module=444, inline_record=16176; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=25040; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1459, inline_record=25852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=2817, inline_record=17232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize
// 信息 ID: 82fcf25305f9f905bd2bcc89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impls::impl$216::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >; module=1334, inline_record=54832; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1439, inline_record=35132; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1439, inline_record=35260; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1439, inline_record=35388; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1439, inline_record=35516; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >; module=1440, inline_record=79984; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >; module=1440, inline_record=103232; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >; module=1440, inline_record=302108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 9ed6d9c3f3731dcf44757e3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=240160; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=272920; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=282596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 72d5358ab8fe00edc53d6011
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> > @ 0x10d8382 -> 0x10affc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor> @ 0x10cc770 -> 0x10b0520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 83df8eefb59fa66a2885a856
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=160968; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=233508; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=239984; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=272796; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=282444; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1440, inline_record=332400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 1174f455c19b74b0b4d2094e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=644; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=864; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=1292; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=2128; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=2348; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=3004; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1439, inline_record=3436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: c26879bc1c54e58f0c82d5d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1439, inline_record=34904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: ffbe2c801bc1aef168cb33dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=159520; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=232004; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=238468; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=271316; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=280924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 2e31efc82f6e178168991953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=274044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 9b1281778831797c153a168e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=8660; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=51440; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=56344; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=127420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: 42c343eaac3e8d58ee58d8a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=51672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 3bf0421733c8ab07baed8501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,enum2$<serde_json::value::Value> > @ 0xea124c -> 0x10af4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize
// 信息 ID: 0e9b13afda5e0aa0273276c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>; module=100, inline_record=71380; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>; module=1432, inline_record=110072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize
// 信息 ID: 1ce59f317e2522b4c5a42a3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::history::decorator_edit::_::impl$0::serialize::__SerializeWith>; module=100, inline_record=150880; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::history::decorator_edit::_::impl$0::serialize::__SerializeWith>; module=1432, inline_record=175900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 10b1e138c224cde5860a64a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> > @ 0x152f3f -> 0x10b520 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> > @ 0x14dd8e -> 0x10b6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > > @ 0x14ae27 -> 0x10b7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate> @ 0x14f47a -> 0x10ba80 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > > > > @ 0x7a909 -> 0x10bc70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> > @ 0x109438e -> 0x10af3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate> @ 0x109396a -> 0x10b0090 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > > > @ 0x101aef4 -> 0x10b0280 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref$<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> > > @ 0x108f577 -> 0x10b0280 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> > @ 0x109d02f -> 0x10b0630 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<ref$<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > > > > > @ 0x101a339 -> 0x10b07b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
// 信息 ID: 579a149b906fa7db2938bbb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0x10ad264 -> 0xd88d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$10::fmt
// 信息 ID: 110b7b15e7f1d421167273ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$50::fmt<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>; module=1334, inline_record=61820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$11::clone
// 信息 ID: e90a7a2b28f44c70311abdeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::Vec::extend_from_slice<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>; module=2756, inline_record=900; 不是独立调用边。
// [PDB 内联宿主] system_decorator::backup::create_or_restore_decorator_backup; module=3844, inline_record=5824; 不是独立调用边。
// [PDB 内联宿主] system_decorator::backup::create_or_restore_decorator_backup; module=3844, inline_record=7444; 不是独立调用边。
// [PDB 内联宿主] system_decorator::backup::impl$2::clone_from; module=3844, inline_record=9836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$13::fmt
// 信息 ID: 760253ef5b6265fb60222678
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>; module=1439, inline_record=4264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$15::fmt
// 信息 ID: f0183971d16f8c60bc079ad9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>; module=1439, inline_record=4920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$17::fmt
// 信息 ID: 70b1c5031d74d4960e00018d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove>; module=1439, inline_record=4480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$1::from
// 信息 ID: d3ec643c88e431c394f64335
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$4::take_snapshot::closure$0::closure_env$0> >; module=1308, inline_record=233468; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::walls::decorator_storage::impl$4::take_snapshot::closure$0::closure_env$0> >; module=1308, inline_record=234760; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=1323, inline_record=189272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$20::fmt
// 信息 ID: 4173c4868a381533f6252448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>; module=1439, inline_record=5136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$21::clone
// 信息 ID: 93e6637b0b95a3575adc4bb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>; module=1323, inline_record=28744; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; module=1323, inline_record=192668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$22::fmt
// 信息 ID: 00f02cd09f6ae258f5dc6ce6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryState>; module=1439, inline_record=4688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$24::eq
// 信息 ID: 1e77cbac7191bba8337160ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::impl$94::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1370, inline_record=11940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$25::default
// 信息 ID: 0fa1be3d60ffc8727932d066
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl; module=427, inline_record=16056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$27::fmt
// 信息 ID: c80cf76e9a3a7daa73d49abd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$29::default
// 信息 ID: c4c3c62059a53d7e4f7cd3d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$52::default<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>; module=1334, inline_record=63872; 不是独立调用边。
// [PDB 内联宿主] system_tree::convert_preplaced_trees::convert_preplaced_trees; module=3386, inline_record=6992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$2::from
// 信息 ID: 9c01f3e719914cd27817f8fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=1323, inline_record=189224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$31::clone
// 信息 ID: df1a6ce8354804b0a5a65d33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=2817, inline_record=16044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$34::clone
// 信息 ID: 24570cc770987cb3513e856e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>; module=1323, inline_record=30888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$3::from
// 信息 ID: ff24954d2009e1b3174d77f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; module=1323, inline_record=192592; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::impl$94::extend<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::walls::decorator_storage::impl$4::restore_snapshot::closure_env$0> >; module=1370, inline_record=21384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$4::from
// 信息 ID: 8c2d48fd1c3480aca1454689
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$5::from
// 信息 ID: 45205c2e9cd7546146e84ca8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl; module=427, inline_record=15044; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >; module=451, inline_record=2616; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$94::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >; module=1321, inline_record=58924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$5::from::closure$0
// 信息 ID: 17b36ae39b3143c284f71fd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$5::from", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- std::collections::hash::map::impl$79::from_iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=1323, inline_record=189136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$7::from::closure$0
// 信息 ID: f4b19b390ff4f6144060e10f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$7::from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure$0
// 信息 ID: f129a6e42740af1e87c2d0e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$8::restore_decorators", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; module=1323, inline_record=191308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0
// 信息 ID: 97bec3d0a3f2ad0b0d23ed3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$9::cull_allow_list", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0 <- alloc::sync::impl$65::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >; module=1308, inline_record=340636; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >; module=1308, inline_record=342808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure$0
// 信息 ID: 2734216ecf937ee0641efe56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$9::cull_allow_list", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0 <- alloc::sync::impl$65::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0 <- core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$9::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >; module=1395, inline_record=6764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 信息 ID: 37990d003667036889fdcc7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::send_state_diff_events -> ["country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0x5e0631 -> 0x10ace20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 信息 ID: 88a6d589babc06f96246a8f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators -> ["country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket @ 0x1108560 -> 0x5dff80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
