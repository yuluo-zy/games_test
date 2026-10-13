// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/history/terrain_edit.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b0333a6b79a25ec159f56b76d8cea28c9da557cded4be0a6556650e792de770a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 11, 55, 100, 416, 470, 484, 493, 1321, 1389, 1431, 1432, 1440
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
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
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102E3
// 首条位置: {"module":1440,"inlinee":66275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":335928,"parent_record_offset":335904,"ancestor_record_offset":335904,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":335952,"parent_record_offset":335928,"ancestor_record_offset":335904,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":335976,"parent_record_offset":335952,"ancestor_record_offset":335904,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":336000,"parent_record_offset":335976,"ancestor_record_offset":335904,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 15, 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1042B
// 首条位置: {"module":1440,"inlinee":66603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":335780,"parent_record_offset":335752,"ancestor_record_offset":335752,"inlinee":66649,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":146628,"parent_record_offset":146600,"ancestor_record_offset":146600,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":146908,"parent_record_offset":146884,"ancestor_record_offset":146600,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":146884,"parent_record_offset":146828,"ancestor_record_offset":146600,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":146856,"parent_record_offset":146828,"ancestor_record_offset":146600,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":146828,"parent_record_offset":146804,"ancestor_record_offset":146600,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1440,"record_offset":335876,"parent_record_offset":335836,"ancestor_record_offset":335752,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":335952,"parent_record_offset":335928,"ancestor_record_offset":335752,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":146756,"parent_record_offset":146724,"ancestor_record_offset":146600,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":335976,"parent_record_offset":335952,"ancestor_record_offset":335752,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":336000,"parent_record_offset":335976,"ancestor_record_offset":335752,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":335928,"parent_record_offset":335904,"ancestor_record_offset":335752,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1440,"record_offset":335904,"parent_record_offset":335836,"ancestor_record_offset":335752,"inlinee":66275,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":146724,"parent_record_offset":146692,"ancestor_record_offset":146600,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1440,"record_offset":335808,"parent_record_offset":335780,"ancestor_record_offset":335752,"inlinee":66650,"source_line":615,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":146660,"parent_record_offset":146628,"ancestor_record_offset":146600,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":146692,"parent_record_offset":146660,"ancestor_record_offset":146600,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102DF
// 首条位置: {"module":1431,"inlinee":66271}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":127152,"parent_record_offset":126756,"ancestor_record_offset":126756,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":126832,"parent_record_offset":126756,"ancestor_record_offset":126756,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":126868,"parent_record_offset":126756,"ancestor_record_offset":126756,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1431,"record_offset":126804,"parent_record_offset":126756,"ancestor_record_offset":126756,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1431,"record_offset":126896,"parent_record_offset":126756,"ancestor_record_offset":126756,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1431,"record_offset":126984,"parent_record_offset":126960,"ancestor_record_offset":126756,"inlinee":66272,"source_line":21,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1431,"record_offset":126960,"parent_record_offset":126924,"ancestor_record_offset":126756,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1431,"record_offset":126924,"parent_record_offset":126896,"ancestor_record_offset":126756,"inlinee":64997,"source_line":502,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1431,"record_offset":125944,"rva":17343152,"va":5386052272}
// 编译实例 RVAs: 0x108a2b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t16907628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e86cfd0101","size":5,"soft_fail":false,"target_va":5402960384,"va":5386052751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13304078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8f2fe34ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386053785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11811921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8afc34bff","size":5,"soft_fail":false,"target_va":5374242272,"va":5386054188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11808802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8decf4bff","size":5,"soft_fail":false,"target_va":5374243632,"va":5386052429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10160678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8daf564ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386052545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10122727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8198a65ff","size":5,"soft_fail":false,"target_va":5375930160,"va":5386052882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4b80
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10116444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8a4a265ff","size":5,"soft_fail":false,"target_va":5375937408,"va":5386053847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10065428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8ec6966ff","size":5,"soft_fail":false,"target_va":5375988576,"va":5386053999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141013da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-485724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8a496f8ff","size":5,"soft_fail":false,"target_va":5385567648,"va":5386053367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410191a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-464475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e8a5e9f8ff","size":5,"soft_fail":false,"target_va":5385589152,"va":5386053622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d8790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t320372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e874e30400","size":5,"soft_fail":false,"target_va":5386373008,"va":5386052631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16904075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e88bef0101","size":5,"soft_fail":false,"target_va":5402956592,"va":5386052512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16907027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386052272,"raw_hex":"e813fb0101","size":5,"soft_fail":false,"target_va":5402959760,"va":5386052728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1431,"record_offset":126136,"parent_record_offset":125944,"ancestor_record_offset":125944,"inlinee":25381,"source_line":255,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":127568,"parent_record_offset":127544,"ancestor_record_offset":125944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":127544,"parent_record_offset":127452,"ancestor_record_offset":125944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1431,"record_offset":128052,"parent_record_offset":128024,"ancestor_record_offset":125944,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1431,"record_offset":126356,"parent_record_offset":126328,"ancestor_record_offset":125944,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1431,"record_offset":126420,"parent_record_offset":126388,"ancestor_record_offset":125944,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1431,"record_offset":126480,"parent_record_offset":126448,"ancestor_record_offset":125944,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1431,"record_offset":126508,"parent_record_offset":126480,"ancestor_record_offset":125944,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":127480,"parent_record_offset":127452,"ancestor_record_offset":125944,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":127360,"parent_record_offset":127336,"ancestor_record_offset":125944,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":127452,"parent_record_offset":127428,"ancestor_record_offset":125944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":127336,"parent_record_offset":127308,"ancestor_record_offset":125944,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":127308,"parent_record_offset":127276,"ancestor_record_offset":125944,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1431,"record_offset":128144,"parent_record_offset":128052,"ancestor_record_offset":125944,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":127508,"parent_record_offset":127480,"ancestor_record_offset":125944,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":126448,"parent_record_offset":126420,"ancestor_record_offset":125944,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":126728,"parent_record_offset":126136,"ancestor_record_offset":125944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1431,"record_offset":126588,"parent_record_offset":126564,"ancestor_record_offset":125944,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":128080,"parent_record_offset":128052,"ancestor_record_offset":125944,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":127152,"parent_record_offset":126756,"ancestor_record_offset":125944,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":126832,"parent_record_offset":126756,"ancestor_record_offset":125944,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":126868,"parent_record_offset":126756,"ancestor_record_offset":125944,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":126540,"parent_record_offset":126508,"ancestor_record_offset":125944,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":126564,"parent_record_offset":126540,"ancestor_record_offset":125944,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1431,"record_offset":126984,"parent_record_offset":126960,"ancestor_record_offset":125944,"inlinee":66272,"source_line":21,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1431,"record_offset":126756,"parent_record_offset":126136,"ancestor_record_offset":125944,"inlinee":66271,"source_line":15,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1431,"record_offset":126804,"parent_record_offset":126756,"ancestor_record_offset":125944,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1431,"record_offset":126896,"parent_record_offset":126756,"ancestor_record_offset":125944,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1431,"record_offset":126960,"parent_record_offset":126924,"ancestor_record_offset":125944,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1431,"record_offset":126328,"parent_record_offset":126136,"ancestor_record_offset":125944,"inlinee":25383,"source_line":877,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1431,"record_offset":126388,"parent_record_offset":126136,"ancestor_record_offset":125944,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$5::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1431,"record_offset":126924,"parent_record_offset":126896,"ancestor_record_offset":125944,"inlinee":64997,"source_line":502,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1510
// 首条位置: {"module":55,"inlinee":5392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102E0
// 首条位置: {"module":1431,"inlinee":66272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1431,"record_offset":131092,"rva":17346192,"va":5386055312}
// 编译实例 RVAs: 0x108ae90, 0x108b180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17537027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386056064,"raw_hex":"e803980b01","size":5,"soft_fail":false,"target_va":5403593344,"va":5386056312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1431,"record_offset":131292,"parent_record_offset":131092,"ancestor_record_offset":131092,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":131312,"parent_record_offset":131292,"ancestor_record_offset":131092,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1431,"record_offset":131332,"parent_record_offset":131312,"ancestor_record_offset":131092,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1431,"record_offset":131352,"parent_record_offset":131332,"ancestor_record_offset":131092,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C5B
// 首条位置: {"module":1431,"inlinee":27739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10429
// 首条位置: {"module":1440,"inlinee":66601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":147704,"parent_record_offset":145108,"ancestor_record_offset":145108,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":204660,"parent_record_offset":200932,"ancestor_record_offset":200932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":148344,"parent_record_offset":145108,"ancestor_record_offset":145108,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":149384,"parent_record_offset":145108,"ancestor_record_offset":145108,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":145148,"parent_record_offset":145108,"ancestor_record_offset":145108,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":147428,"parent_record_offset":145108,"ancestor_record_offset":145108,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":145804,"parent_record_offset":145780,"ancestor_record_offset":145108,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":145780,"parent_record_offset":145724,"ancestor_record_offset":145108,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":146428,"parent_record_offset":146400,"ancestor_record_offset":145108,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":146488,"parent_record_offset":146428,"ancestor_record_offset":145108,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":145300,"parent_record_offset":145276,"ancestor_record_offset":145108,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":145352,"parent_record_offset":145324,"ancestor_record_offset":145108,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":145376,"parent_record_offset":145352,"ancestor_record_offset":145108,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":145752,"parent_record_offset":145724,"ancestor_record_offset":145108,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":145940,"parent_record_offset":145916,"ancestor_record_offset":145108,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":145724,"parent_record_offset":145696,"ancestor_record_offset":145108,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":145916,"parent_record_offset":145884,"ancestor_record_offset":145108,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":145884,"parent_record_offset":145852,"ancestor_record_offset":145108,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":146520,"parent_record_offset":146428,"ancestor_record_offset":145108,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":147544,"parent_record_offset":147520,"ancestor_record_offset":145108,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":146088,"parent_record_offset":146060,"ancestor_record_offset":145108,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":145324,"parent_record_offset":145300,"ancestor_record_offset":145108,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":147520,"parent_record_offset":147484,"ancestor_record_offset":145108,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":145592,"parent_record_offset":145220,"ancestor_record_offset":145108,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":145452,"parent_record_offset":145428,"ancestor_record_offset":145108,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":146456,"parent_record_offset":146428,"ancestor_record_offset":145108,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":147600,"parent_record_offset":147544,"ancestor_record_offset":145108,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":147180,"parent_record_offset":145220,"ancestor_record_offset":145108,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":145404,"parent_record_offset":145376,"ancestor_record_offset":145108,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":145428,"parent_record_offset":145404,"ancestor_record_offset":145108,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1440,"record_offset":146600,"parent_record_offset":146568,"ancestor_record_offset":145108,"inlinee":66603,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1440,"record_offset":147964,"parent_record_offset":147940,"ancestor_record_offset":145108,"inlinee":24927,"source_line":28,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":149456,"parent_record_offset":149432,"ancestor_record_offset":145108,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":149432,"parent_record_offset":149408,"ancestor_record_offset":145108,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":204596,"parent_record_offset":204572,"ancestor_record_offset":200932,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":149568,"parent_record_offset":149544,"ancestor_record_offset":145108,"inlinee":24928,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":145184,"parent_record_offset":145148,"ancestor_record_offset":145108,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":147456,"parent_record_offset":147428,"ancestor_record_offset":145108,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":146724,"parent_record_offset":146692,"ancestor_record_offset":145108,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":147660,"parent_record_offset":147636,"ancestor_record_offset":145108,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":146568,"parent_record_offset":145220,"ancestor_record_offset":145108,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":204472,"parent_record_offset":204448,"ancestor_record_offset":200932,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":147988,"parent_record_offset":147964,"ancestor_record_offset":145108,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":148300,"parent_record_offset":148276,"ancestor_record_offset":145108,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":145276,"parent_record_offset":145220,"ancestor_record_offset":145108,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":146660,"parent_record_offset":146628,"ancestor_record_offset":145108,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":146628,"parent_record_offset":146600,"ancestor_record_offset":145108,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":146692,"parent_record_offset":146660,"ancestor_record_offset":145108,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":145220,"parent_record_offset":145184,"ancestor_record_offset":145108,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":147484,"parent_record_offset":147456,"ancestor_record_offset":145108,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10414
// 首条位置: {"module":1440,"inlinee":66580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":82028,"parent_record_offset":81112,"ancestor_record_offset":81112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":13692,"parent_record_offset":13664,"ancestor_record_offset":13664,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":82192,"parent_record_offset":82168,"ancestor_record_offset":81112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":82168,"parent_record_offset":82076,"ancestor_record_offset":81112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":82104,"parent_record_offset":82076,"ancestor_record_offset":81112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":82076,"parent_record_offset":82052,"ancestor_record_offset":81112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":13776,"parent_record_offset":13748,"ancestor_record_offset":13664,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":82132,"parent_record_offset":82104,"ancestor_record_offset":81112,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":82052,"parent_record_offset":82028,"ancestor_record_offset":81112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":13836,"parent_record_offset":13776,"ancestor_record_offset":13664,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":13804,"parent_record_offset":13776,"ancestor_record_offset":13664,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":13944,"parent_record_offset":13892,"ancestor_record_offset":13664,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1440,"record_offset":13892,"parent_record_offset":13868,"ancestor_record_offset":13664,"inlinee":24927,"source_line":28,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":13720,"parent_record_offset":13692,"ancestor_record_offset":13664,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":13868,"parent_record_offset":13748,"ancestor_record_offset":13664,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":81664,"parent_record_offset":81640,"ancestor_record_offset":81112,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":13664,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":14208,"parent_record_offset":14184,"ancestor_record_offset":13664,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":13748,"parent_record_offset":13720,"ancestor_record_offset":13664,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=4
// 源码行: 21, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":11,"record_offset":1044,"rva":96096,"va":5368805216}
// 编译实例 RVAs: 0x17760, 0x17930, 0x108acc0, 0x108af60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=2
//     target_aliases=13 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e854050000","size":5,"soft_fail":false,"target_va":5368807040,"va":5368805671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140155010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1300563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e853d81300","size":5,"soft_fail":false,"target_va":5370105872,"va":5368805304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1306043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e8bbed1300","size":5,"soft_fail":false,"target_va":5370111424,"va":5368805376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140156ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1307585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e8c1f31300","size":5,"soft_fail":false,"target_va":5370112928,"va":5368805338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3448330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e80a9e3400","size":5,"soft_fail":false,"target_va":5372253808,"va":5368805473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3944205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e80d2f3c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368805502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11272764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386054848,"raw_hex":"e8c4fd53ff","size":5,"soft_fail":false,"target_va":5374782544,"va":5386055303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10163238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386054848,"raw_hex":"e8daeb64ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386055105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7220422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368805216,"raw_hex":"e8c62c6e00","size":5,"soft_fail":false,"target_va":5376025984,"va":5368805557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410960d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386054848,"raw_hex":"e8b3b30000","size":5,"soft_fail":false,"target_va":5386100944,"va":5386054936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t49627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386054848,"raw_hex":"e8dbc10000","size":5,"soft_fail":false,"target_va":5386104640,"va":5386055008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109afc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t66177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386054848,"raw_hex":"e881020100","size":5,"soft_fail":false,"target_va":5386121152,"va":5386054970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":11,"record_offset":1264,"parent_record_offset":1044,"ancestor_record_offset":1044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":11,"record_offset":1236,"parent_record_offset":1044,"ancestor_record_offset":1044,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":11,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1044,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":11,"record_offset":1624,"parent_record_offset":1568,"ancestor_record_offset":1044,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":11,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":1044,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":11,"record_offset":1396,"parent_record_offset":1340,"ancestor_record_offset":1044,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":11,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":1044,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":11,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":1044,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":11,"record_offset":1428,"parent_record_offset":1340,"ancestor_record_offset":1044,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":11,"record_offset":1292,"parent_record_offset":1264,"ancestor_record_offset":1044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":11,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":1044,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00011.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":3460,"rva":7050528,"va":5375759648}
// 编译实例 RVAs: 0x6b9520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1824254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759648,"raw_hex":"e8fed51b00","size":5,"soft_fail":false,"target_va":5377583952,"va":5375759693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::field
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1810646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759648,"raw_hex":"e8d6a01b00","size":5,"soft_fail":false,"target_va":5377570416,"va":5375759765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::builders::DebugStruct::finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1811294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759648,"raw_hex":"e85ea31b00","size":5,"soft_fail":false,"target_va":5377571072,"va":5375759773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::duration
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CCE
// 首条位置: {"module":416,"inlinee":23758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":19968,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":20024,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":19856,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":19912,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":19772,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":19992,"parent_record_offset":19968,"ancestor_record_offset":19728,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":19728,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":484,"record_offset":19936,"parent_record_offset":19912,"ancestor_record_offset":19728,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":19728,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":19728,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::duration::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x687F
// 首条位置: {"module":470,"inlinee":26751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1321,"record_offset":35904,"parent_record_offset":35824,"ancestor_record_offset":35824,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":35848,"parent_record_offset":35824,"ancestor_record_offset":35824,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1321,"record_offset":35976,"parent_record_offset":35948,"ancestor_record_offset":35824,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":35872,"parent_record_offset":35848,"ancestor_record_offset":35824,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1321,"record_offset":35928,"parent_record_offset":35904,"ancestor_record_offset":35824,"inlinee":26753,"source_line":68,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":36064,"parent_record_offset":36036,"ancestor_record_offset":35824,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":36036,"parent_record_offset":35928,"ancestor_record_offset":35824,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":35948,"parent_record_offset":35928,"ancestor_record_offset":35824,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1321,"record_offset":36100,"parent_record_offset":36064,"ancestor_record_offset":35824,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6881
// 首条位置: {"module":470,"inlinee":26753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":36036,"parent_record_offset":35928,"ancestor_record_offset":35928,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":35948,"parent_record_offset":35928,"ancestor_record_offset":35928,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1321,"record_offset":35976,"parent_record_offset":35948,"ancestor_record_offset":35928,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":36064,"parent_record_offset":36036,"ancestor_record_offset":35928,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1321,"record_offset":36100,"parent_record_offset":36064,"ancestor_record_offset":35928,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::revert
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":3596,"rva":7050672,"va":5375759792}
// 编译实例 RVAs: 0x6b95b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6146352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759792,"raw_hex":"e8d036a2ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5375760219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b9b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759792,"raw_hex":"e892040000","size":5,"soft_fail":false,"target_va":5375761280,"va":5375760105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1183825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759792,"raw_hex":"e851101200","size":5,"soft_fail":false,"target_va":5376944000,"va":5375760170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4452701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759792,"raw_hex":"e8a30ebcff","size":5,"soft_fail":false,"target_va":5371307280,"va":5375759976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27218241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375759792,"raw_hex":"e841519f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5375760698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":493,"record_offset":3880,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":3772,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$162::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":493,"record_offset":4136,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":27674,"source_line":35,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":4224,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":5840,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":493,"record_offset":4192,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":493,"record_offset":4292,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":24204,"source_line":58,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":4064,"parent_record_offset":4032,"ancestor_record_offset":3596,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":4032,"parent_record_offset":3996,"ancestor_record_offset":3596,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":3996,"parent_record_offset":3964,"ancestor_record_offset":3596,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":493,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":3596,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":493,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3596,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":493,"record_offset":3964,"parent_record_offset":3880,"ancestor_record_offset":3596,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$33::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3596,"inlinee":4544,"source_line":1469,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3596,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":3596,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":3596,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":493,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":3596,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":493,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":3596,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":493,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":3596,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3596,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":3596,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":3596,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":4096,"parent_record_offset":3996,"ancestor_record_offset":3596,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::revert::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":493,"record_offset":4260,"parent_record_offset":4224,"ancestor_record_offset":3596,"inlinee":27676,"source_line":104,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::revert::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":493,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":3596,"inlinee":27677,"source_line":110,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":493,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":3596,"inlinee":24172,"source_line":28,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":493,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":3596,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":3596,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3596,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":5052,"parent_record_offset":4832,"ancestor_record_offset":3596,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":4396,"parent_record_offset":4364,"ancestor_record_offset":3596,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":3596,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4940,"parent_record_offset":4832,"ancestor_record_offset":3596,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":3596,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":5260,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":3596,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":3596,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4424,"parent_record_offset":4364,"ancestor_record_offset":3596,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3596,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":4712,"parent_record_offset":4684,"ancestor_record_offset":3596,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":3596,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":4800,"parent_record_offset":4684,"ancestor_record_offset":3596,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":4684,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4516,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":4832,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":4544,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":4628,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":5168,"parent_record_offset":4472,"ancestor_record_offset":3596,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":493,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3596,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C1C
// 首条位置: {"module":493,"inlinee":27676}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C1D
// 首条位置: {"module":493,"inlinee":27677}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":493,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":5864,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::reapply
// 记录形式: compiled_procedure; instances=1
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":5928,"rva":7051584,"va":5375760704}
// 编译实例 RVAs: 0x6b9940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b9b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375760704,"raw_hex":"e822010000","size":5,"soft_fail":false,"target_va":5375761280,"va":5375760985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1182947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375760704,"raw_hex":"e8e30c1200","size":5,"soft_fail":false,"target_va":5376944000,"va":5375761048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4453609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375760704,"raw_hex":"e8170bbcff","size":5,"soft_fail":false,"target_va":5371307280,"va":5375760884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":493,"record_offset":6200,"parent_record_offset":5928,"ancestor_record_offset":5928,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":6096,"parent_record_offset":5928,"ancestor_record_offset":5928,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$162::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":493,"record_offset":6456,"parent_record_offset":5928,"ancestor_record_offset":5928,"inlinee":27674,"source_line":35,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":6544,"parent_record_offset":5928,"ancestor_record_offset":5928,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":493,"record_offset":6512,"parent_record_offset":5928,"ancestor_record_offset":5928,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":6384,"parent_record_offset":6352,"ancestor_record_offset":5928,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":6352,"parent_record_offset":6316,"ancestor_record_offset":5928,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":6316,"parent_record_offset":6284,"ancestor_record_offset":5928,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":493,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":5928,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":493,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":5928,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":493,"record_offset":6284,"parent_record_offset":6200,"ancestor_record_offset":5928,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$33::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":5928,"inlinee":4544,"source_line":1469,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":5928,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":6416,"parent_record_offset":6316,"ancestor_record_offset":5928,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":493,"record_offset":6576,"parent_record_offset":6544,"ancestor_record_offset":5928,"inlinee":27679,"source_line":164,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C1F
// 首条位置: {"module":493,"inlinee":27679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":7132,"rva":7052160,"va":5375761280}
// 编译实例 RVAs: 0x6b9b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6147507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e84d32a2ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5375761374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4062383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e85103c2ff","size":5,"soft_fail":false,"target_va":5371700720,"va":5375763098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3011911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8b90ad2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375761602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3012551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e83908d2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375762258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d6fb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7036344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8b85d6b00","size":5,"soft_fail":false,"target_va":5382798128,"va":5375761779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e876c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8181700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8c4d77c00","size":5,"soft_fail":false,"target_va":5383943872,"va":5375762167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e89460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8189635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8c3f67c00","size":5,"soft_fail":false,"target_va":5383951456,"va":5375761816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27831621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e845ada801","size":5,"soft_fail":false,"target_va":5403593360,"va":5375761734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27215551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8bf469f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375762588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1764522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8aaec1a00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375762593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27216574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8be4a9f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5375762573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27216390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e8064a9f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5375762549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27217307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e89b4d9f01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375762816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1176367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375761280,"raw_hex":"e82ff31100","size":5,"soft_fail":false,"target_va":5376938064,"va":5375761692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":8488,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":9036,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":493,"record_offset":10772,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":12032,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":11164,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":493,"record_offset":7344,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":24204,"source_line":58,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":493,"record_offset":10448,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":493,"record_offset":10804,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":24897,"source_line":669,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_new_from_points_remove_duplicates
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":493,"record_offset":9468,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":21586,"source_line":221,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":493,"record_offset":10560,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":493,"record_offset":10388,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":493,"record_offset":10292,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":21345,"source_line":189,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":7132,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":7132,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":7132,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":8804,"parent_record_offset":8712,"ancestor_record_offset":7132,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":9656,"parent_record_offset":9556,"ancestor_record_offset":7132,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":7132,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":8712,"parent_record_offset":8684,"ancestor_record_offset":7132,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":8684,"parent_record_offset":8656,"ancestor_record_offset":7132,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":8924,"parent_record_offset":8684,"ancestor_record_offset":7132,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":7132,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":8616,"parent_record_offset":8584,"ancestor_record_offset":7132,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":8584,"parent_record_offset":8552,"ancestor_record_offset":7132,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":8520,"parent_record_offset":8488,"ancestor_record_offset":7132,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":8552,"parent_record_offset":8520,"ancestor_record_offset":7132,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":10848,"parent_record_offset":10804,"ancestor_record_offset":7132,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":8656,"parent_record_offset":8616,"ancestor_record_offset":7132,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":8748,"parent_record_offset":8712,"ancestor_record_offset":7132,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":8772,"parent_record_offset":8748,"ancestor_record_offset":7132,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":10648,"parent_record_offset":10624,"ancestor_record_offset":7132,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":7132,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":7132,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":7132,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":7132,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":493,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":7132,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":493,"record_offset":10356,"parent_record_offset":10292,"ancestor_record_offset":7132,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":493,"record_offset":10328,"parent_record_offset":10292,"ancestor_record_offset":7132,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":493,"record_offset":10740,"parent_record_offset":10560,"ancestor_record_offset":7132,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":493,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":7132,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":493,"record_offset":9620,"parent_record_offset":9588,"ancestor_record_offset":7132,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":493,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":7132,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":10500,"parent_record_offset":10448,"ancestor_record_offset":7132,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":10528,"parent_record_offset":10448,"ancestor_record_offset":7132,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":7132,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":493,"record_offset":10624,"parent_record_offset":10600,"ancestor_record_offset":7132,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":493,"record_offset":10600,"parent_record_offset":10560,"ancestor_record_offset":7132,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":493,"record_offset":8960,"parent_record_offset":8616,"ancestor_record_offset":7132,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":9508,"parent_record_offset":9468,"ancestor_record_offset":7132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":7132,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":7132,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":8880,"parent_record_offset":8852,"ancestor_record_offset":7132,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":493,"record_offset":10904,"parent_record_offset":10876,"ancestor_record_offset":7132,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":493,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":7132,"inlinee":27681,"source_line":203,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":493,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":7132,"inlinee":24172,"source_line":28,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":493,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":7132,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":7132,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":7132,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":8108,"parent_record_offset":7888,"ancestor_record_offset":7132,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":7448,"parent_record_offset":7416,"ancestor_record_offset":7132,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7132,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7996,"parent_record_offset":7888,"ancestor_record_offset":7132,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":7132,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":8316,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7912,"parent_record_offset":7888,"ancestor_record_offset":7132,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":7132,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7476,"parent_record_offset":7416,"ancestor_record_offset":7132,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7528,"parent_record_offset":7500,"ancestor_record_offset":7132,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":7768,"parent_record_offset":7740,"ancestor_record_offset":7132,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":7132,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":7856,"parent_record_offset":7740,"ancestor_record_offset":7132,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":7740,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7572,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":7888,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":7600,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":7684,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":8224,"parent_record_offset":7528,"ancestor_record_offset":7132,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":493,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":7132,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":493,"record_offset":10876,"parent_record_offset":10804,"ancestor_record_offset":7132,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C21
// 首条位置: {"module":493,"inlinee":27681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":493,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":12056,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00493.jsonl.gz"}
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
// 完整结构及原始类型记录: terrain_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::terrain_edit::TerrainEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::terrain_edit::TerrainEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "core::option::Option", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$50::fmt<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$50::fmt", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::terrain_edit::TerrainEditEditStroke
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::terrain_edit::TerrainEditEditStroke>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["country_core::resources::history::terrain_edit::TerrainEditEditStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::terrain_edit::TerrainEditNewStroke
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::terrain_edit::TerrainEditNewStroke>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["country_core::resources::history::terrain_edit::TerrainEditNewStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: terrain_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::terrain_edit::TerrainEdit", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/source-ownership.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/terrain_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: terrain_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::duration
// 信息 ID: 0d040d848205b4f102ecbd9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=19728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=20432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::reapply
// 信息 ID: 6a0583511b60d1821f1766bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_terrain::replay_terrain @ 0x58502c -> 0x6b9940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::revert
// 信息 ID: 054987ba9df72a0be8231c9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_terrain::replay_terrain @ 0x585010 -> 0x6b95b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
// 信息 ID: ac885602bb66e3682b14a34a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::revert @ 0x6b96e9 -> 0x6b9b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::reapply @ 0x6b9a59 -> 0x6b9b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize
// 信息 ID: dfee1ffc797c882048c25261
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1431, inline_record=126984; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1431, inline_record=127112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 060a5da4a183fd68e44aad5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1440, inline_record=335904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 1893612dd4adc97a981c929b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor> @ 0x10c8264 -> 0x108ae90 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor> @ 0x10c102c -> 0x108b180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 8f602e6b3b39a28909564e8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=146600; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=202416; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1440, inline_record=335752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: b33483725a47f8e9ad0a7270
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1431, inline_record=108024; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1431, inline_record=108244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 0706ea1a1701517b650a8143
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1431, inline_record=126756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: e70ade5e0e295b51e9c5b64d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=145108; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=200932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: a123d1a64c6c3c4c511de4eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=13664; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=81112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: aa30ecd4a2ade84354cdab61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,enum2$<serde_json::value::Value> > @ 0xea137c -> 0x108a2b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::serialize
// 信息 ID: f60eefcafde7830d65f9bfb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >; module=100, inline_record=76460; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >; module=1432, inline_record=122768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: fdab285cc8150b839b917a27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::terrain_edit::TerrainEditNewStroke> @ 0x14e0ea -> 0x17760 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::terrain_edit::TerrainEditEditStroke> @ 0x1501fa -> 0x17930 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::terrain_edit::TerrainEditNewStroke> @ 0x10925da -> 0x108acc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<country_core::resources::history::terrain_edit::TerrainEditEditStroke> @ 0x109513a -> 0x108af60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$0::fmt
// 信息 ID: 0ba224c9dd5fc7ab846834b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::sync::impl$50::fmt<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> @ 0xea21c7 -> 0x6b9520 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::duration::closure$0
// 信息 ID: 4166e7578e7da9da011a9c0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::duration", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; module=1321, inline_record=35824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
// 信息 ID: 76beba4e6e18652310222fe8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::duration", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; module=1321, inline_record=35928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
// 信息 ID: 070fcc567a67933a0ff95e60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::reapply", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::reapply; module=493, inline_record=6576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$0
// 信息 ID: 9f05e90a33e9f1e3a04d734d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=493, inline_record=4260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$1
// 信息 ID: b4f0c14b37c7ed3826e0e594
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=493, inline_record=5864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
// 信息 ID: b5d461c7c8cfb71a91ec17a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::update_terrain_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve; module=493, inline_record=12056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
