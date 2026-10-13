// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/valid_enclosure/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: dd4d665b65ade67a11a25f03e592752a9e32776e56b2e9e9b9c8df358ace1d47
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 55, 100, 114, 399, 411, 423, 434, 435, 445, 474, 480, 486, 491, 492, 493, 1302, 1326, 1350, 1389, 1390, 1421, 1426, 1432, 1461, 2269, 2830, 2831, 3194, 3196, 4108
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
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 11, 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1326,"record_offset":14796,"rva":15226032,"va":5383935152}
// 编译实例 RVAs: 0xe854b0, 0xe86020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11621377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e8ffab4eff","size":5,"soft_fail":false,"target_va":5372314208,"va":5383935580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11185797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e87b5155ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383935504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141065370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1962522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383938080,"raw_hex":"e81af21d00","size":5,"soft_fail":false,"target_va":5385900912,"va":5383938385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141065410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1965420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e86cfd1d00","size":5,"soft_fail":false,"target_va":5385901072,"va":5383935647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141065510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1962761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383938080,"raw_hex":"e809f31d00","size":5,"soft_fail":false,"target_va":5385901328,"va":5383938562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410655b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1966039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e8d7ff1d00","size":5,"soft_fail":false,"target_va":5385901488,"va":5383935444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410bdf80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2329211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e87b8a2300","size":5,"soft_fail":false,"target_va":5386264448,"va":5383935232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2377030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e846452400","size":5,"soft_fail":false,"target_va":5386312384,"va":5383935349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9700
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19021745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e8b13f2201","size":5,"soft_fail":false,"target_va":5402957568,"va":5383935818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19040065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383938080,"raw_hex":"e841872201","size":5,"soft_fail":false,"target_va":5402978944,"va":5383938874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-6479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383935152,"raw_hex":"e8b1e6ffff","size":5,"soft_fail":false,"target_va":5383929056,"va":5383935530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5970835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383938080,"raw_hex":"e86de4a4ff","size":5,"soft_fail":false,"target_va":5377967856,"va":5383938686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":16308,"parent_record_offset":14796,"ancestor_record_offset":14796,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":14988,"parent_record_offset":14796,"ancestor_record_offset":14796,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":14796,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":14796,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1326,"record_offset":15500,"parent_record_offset":15472,"ancestor_record_offset":14796,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1326,"record_offset":22692,"parent_record_offset":22648,"ancestor_record_offset":21064,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1326,"record_offset":16344,"parent_record_offset":16308,"ancestor_record_offset":14796,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1326,"record_offset":15320,"parent_record_offset":14988,"ancestor_record_offset":14796,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":15284,"parent_record_offset":14988,"ancestor_record_offset":14796,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":15220,"parent_record_offset":14988,"ancestor_record_offset":14796,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":15256,"parent_record_offset":14988,"ancestor_record_offset":14796,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":16372,"parent_record_offset":16344,"ancestor_record_offset":14796,"inlinee":27596,"source_line":44,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":22648,"parent_record_offset":22616,"ancestor_record_offset":21064,"inlinee":27609,"source_line":280,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":15024,"parent_record_offset":14988,"ancestor_record_offset":14796,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":15180,"parent_record_offset":15080,"ancestor_record_offset":14796,"inlinee":25263,"source_line":335,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":15152,"parent_record_offset":15080,"ancestor_record_offset":14796,"inlinee":25262,"source_line":433,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1326,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14796,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1326,"record_offset":15124,"parent_record_offset":15080,"ancestor_record_offset":14796,"inlinee":16500,"source_line":470,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1326,"record_offset":15080,"parent_record_offset":15052,"ancestor_record_offset":14796,"inlinee":16497,"source_line":212,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1326,"record_offset":16448,"rva":15226928,"va":5383936048}
// 编译实例 RVAs: 0xe85830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a93b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19020653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936048,"raw_hex":"e86d3b2201","size":5,"soft_fail":false,"target_va":5402956720,"va":5383936062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":16804,"parent_record_offset":16448,"ancestor_record_offset":16448,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":16696,"parent_record_offset":16448,"ancestor_record_offset":16448,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":16716,"parent_record_offset":16696,"ancestor_record_offset":16448,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":16764,"parent_record_offset":16740,"ancestor_record_offset":16448,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1326,"record_offset":16740,"parent_record_offset":16716,"ancestor_record_offset":16448,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":114,"record_offset":232,"rva":1722784,"va":5370431904}
// 编译实例 RVAs: 0x1a49a0, 0xe85260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeStructVariant$u20$as$u20$serde..ser..SerializeStructVariant$GT$::end
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5594265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8995c5500","size":5,"soft_fail":false,"target_va":5376026640,"va":5370432370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct_variant
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5593685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8555a5500","size":5,"soft_fail":false,"target_va":5376025840,"va":5370432150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401480f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-379395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8fd35faff","size":5,"soft_fail":false,"target_va":5370052848,"va":5370432238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1821406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8deca1b00","size":5,"soft_fail":false,"target_va":5372253808,"va":5370432397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1965686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e876fe1d00","size":5,"soft_fail":false,"target_va":5372398176,"va":5370432485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2317712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8905d2300","size":5,"soft_fail":false,"target_va":5372749696,"va":5370431979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2317433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8795c2300","size":5,"soft_fail":false,"target_va":5372749712,"va":5370432274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8043186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383934560,"raw_hex":"e84e4585ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5383935053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7984442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383934560,"raw_hex":"e8c62a86ff","size":5,"soft_fail":false,"target_va":5375950704,"va":5383935141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108d800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2131021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383934560,"raw_hex":"e84d842000","size":5,"soft_fail":false,"target_va":5386065920,"va":5383934894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32545713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370431904,"raw_hex":"e8b19bf001","size":5,"soft_fail":false,"target_va":5402978144,"va":5370432426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":1632,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":114,"record_offset":1544,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":114,"record_offset":1576,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":114,"record_offset":432,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7826,"source_line":170,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":1792,"parent_record_offset":1736,"ancestor_record_offset":232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":232,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":1980,"parent_record_offset":1920,"ancestor_record_offset":232,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":232,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":864,"parent_record_offset":692,"ancestor_record_offset":232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":232,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":232,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":544,"parent_record_offset":516,"ancestor_record_offset":232,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":232,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":516,"parent_record_offset":488,"ancestor_record_offset":232,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":114,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":232,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":664,"parent_record_offset":628,"ancestor_record_offset":232,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":114,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":114,"record_offset":2012,"parent_record_offset":1920,"ancestor_record_offset":232,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":114,"record_offset":900,"parent_record_offset":628,"ancestor_record_offset":232,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":1660,"parent_record_offset":1632,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":1948,"parent_record_offset":1920,"ancestor_record_offset":232,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":114,"record_offset":460,"parent_record_offset":432,"ancestor_record_offset":232,"inlinee":7827,"source_line":150,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FE9
// 首条位置: {"module":423,"inlinee":24553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$4::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":435,"record_offset":54224,"rva":6138960,"va":5374848080}
// 编译实例 RVAs: 0x5dac50, 0x19886b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2735786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374848080,"raw_hex":"e8aabe2900","size":5,"soft_fail":false,"target_va":5377584016,"va":5374848225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2735779","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374848080,"raw_hex":"e9a3be2900","size":5,"soft_fail":false,"target_va":5377583920,"va":5374848136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 20, 30, 36, 253, 259, 265, 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1F7
// 首条位置: {"module":1326,"inlinee":61943}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1326,"record_offset":15320,"parent_record_offset":14988,"ancestor_record_offset":14988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":15284,"parent_record_offset":14988,"ancestor_record_offset":14988,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":15220,"parent_record_offset":14988,"ancestor_record_offset":14988,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":15256,"parent_record_offset":14988,"ancestor_record_offset":14988,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":15024,"parent_record_offset":14988,"ancestor_record_offset":14988,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":14988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":14988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1326,"record_offset":15500,"parent_record_offset":15472,"ancestor_record_offset":14988,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1326,"record_offset":15352,"parent_record_offset":15320,"ancestor_record_offset":14988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":19736,"inlinee":51267,"source_line":2061,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":15180,"parent_record_offset":15080,"ancestor_record_offset":14988,"inlinee":25263,"source_line":335,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":15152,"parent_record_offset":15080,"ancestor_record_offset":14988,"inlinee":25262,"source_line":433,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":16764,"parent_record_offset":16740,"ancestor_record_offset":16696,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1326,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14988,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1326,"record_offset":15124,"parent_record_offset":15080,"ancestor_record_offset":14988,"inlinee":16500,"source_line":470,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1326,"record_offset":15080,"parent_record_offset":15052,"ancestor_record_offset":14988,"inlinee":16497,"source_line":212,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=3
// 源码行: 20, 30, 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1326,"record_offset":17632,"rva":15227200,"va":5383936320}
// 编译实例 RVAs: 0xe85940, 0xe85bc0, 0xe86550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11186998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936320,"raw_hex":"e8ca4c55ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383936705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9520
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19020417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936320,"raw_hex":"e8813a2201","size":5,"soft_fail":false,"target_va":5402957088,"va":5383936666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::__private::string::from_utf8_lossy
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6575297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936320,"raw_hex":"e83fab9bff","size":5,"soft_fail":false,"target_va":5377361344,"va":5383936636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1326,"record_offset":17832,"parent_record_offset":17632,"ancestor_record_offset":17632,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":17976,"parent_record_offset":17952,"ancestor_record_offset":17632,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":17632,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1326,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":17632,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1326,"record_offset":17856,"parent_record_offset":17832,"ancestor_record_offset":17632,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1326,"record_offset":16836,"rva":15226976,"va":5383936096}
// 编译实例 RVAs: 0xe85860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19020799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936096,"raw_hex":"e8ff3b2201","size":5,"soft_fail":false,"target_va":5402957088,"va":5383936284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1326,"record_offset":17036,"parent_record_offset":16836,"ancestor_record_offset":16836,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1326,"record_offset":17056,"parent_record_offset":17036,"ancestor_record_offset":16836,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1326,"record_offset":17076,"parent_record_offset":17056,"ancestor_record_offset":16836,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1326,"record_offset":17096,"parent_record_offset":17076,"ancestor_record_offset":16836,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 20, 30, 253, 259, 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1DE
// 首条位置: {"module":1326,"inlinee":61918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 20, 30, 253, 259, 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1023D
// 首条位置: {"module":1421,"inlinee":66109}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":66110,"source_line":2039,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1421,"record_offset":676,"parent_record_offset":576,"ancestor_record_offset":548,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":548,"inlinee":61918,"source_line":265,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":624,"parent_record_offset":576,"ancestor_record_offset":548,"inlinee":9828,"source_line":1383,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1DD
// 首条位置: {"module":1326,"inlinee":61917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1FE
// 首条位置: {"module":1421,"inlinee":61950}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1FD
// 首条位置: {"module":1421,"inlinee":61949}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1421,"record_offset":23004,"parent_record_offset":22980,"ancestor_record_offset":22980,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1421,"record_offset":23028,"parent_record_offset":23004,"ancestor_record_offset":22980,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22980,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22980,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1DF
// 首条位置: {"module":1326,"inlinee":61919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10252
// 首条位置: {"module":1421,"inlinee":66130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":22736,"parent_record_offset":22696,"ancestor_record_offset":22696,"inlinee":66110,"source_line":2039,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1421,"record_offset":23264,"parent_record_offset":22736,"ancestor_record_offset":22696,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1421,"record_offset":23028,"parent_record_offset":23004,"ancestor_record_offset":22696,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22696,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22696,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1421,"record_offset":23004,"parent_record_offset":22980,"ancestor_record_offset":22696,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22872,"parent_record_offset":22736,"ancestor_record_offset":22696,"inlinee":61950,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22980,"parent_record_offset":22956,"ancestor_record_offset":22696,"inlinee":61949,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22696,"inlinee":61919,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22900,"parent_record_offset":22736,"ancestor_record_offset":22696,"inlinee":9773,"source_line":1563,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22956,"parent_record_offset":22736,"ancestor_record_offset":22696,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22792,"parent_record_offset":22736,"ancestor_record_offset":22696,"inlinee":9828,"source_line":1383,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE4
// 首条位置: {"module":1326,"inlinee":27620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1024E
// 首条位置: {"module":1421,"inlinee":66126}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":23312,"parent_record_offset":22460,"ancestor_record_offset":22460,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":23424,"parent_record_offset":22460,"ancestor_record_offset":22460,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":23456,"parent_record_offset":22460,"ancestor_record_offset":22460,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22492,"parent_record_offset":22460,"ancestor_record_offset":22460,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":23340,"parent_record_offset":22460,"ancestor_record_offset":22460,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1421,"record_offset":23264,"parent_record_offset":22736,"ancestor_record_offset":22460,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1421,"record_offset":23028,"parent_record_offset":23004,"ancestor_record_offset":22460,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1421,"record_offset":22620,"parent_record_offset":22580,"ancestor_record_offset":22460,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22460,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22460,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":22580,"parent_record_offset":22532,"ancestor_record_offset":22460,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1421,"record_offset":23004,"parent_record_offset":22980,"ancestor_record_offset":22460,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22872,"parent_record_offset":22736,"ancestor_record_offset":22460,"inlinee":61950,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22980,"parent_record_offset":22956,"ancestor_record_offset":22460,"inlinee":61949,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22460,"inlinee":61919,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":22696,"parent_record_offset":22656,"ancestor_record_offset":22460,"inlinee":66130,"source_line":20,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":22736,"parent_record_offset":22696,"ancestor_record_offset":22460,"inlinee":66110,"source_line":2039,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$31::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":22532,"parent_record_offset":22492,"ancestor_record_offset":22460,"inlinee":66128,"source_line":2304,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$31::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22460,"inlinee":66131,"source_line":2317,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22900,"parent_record_offset":22736,"ancestor_record_offset":22460,"inlinee":9773,"source_line":1563,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22956,"parent_record_offset":22736,"ancestor_record_offset":22460,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22792,"parent_record_offset":22736,"ancestor_record_offset":22460,"inlinee":9828,"source_line":1383,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":22656,"parent_record_offset":22532,"ancestor_record_offset":22460,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 20, 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10256
// 首条位置: {"module":1421,"inlinee":66134}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":23600,"parent_record_offset":23572,"ancestor_record_offset":23572,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":23704,"parent_record_offset":23648,"ancestor_record_offset":23572,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$29::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":23648,"parent_record_offset":23624,"ancestor_record_offset":23572,"inlinee":66136,"source_line":2259,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":23572,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":23676,"parent_record_offset":23648,"ancestor_record_offset":23572,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 20, 253, 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1326,"record_offset":18256,"rva":15227664,"va":5383936784}
// 编译实例 RVAs: 0xe85b10, 0xe86420, 0xe86680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t19020993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383939104,"raw_hex":"e8c13c2201","size":5,"soft_fail":false,"target_va":5402960384,"va":5383939386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1960030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383939104,"raw_hex":"e85ee81d00","size":5,"soft_fail":false,"target_va":5385899152,"va":5383939117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410650f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1963470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936784,"raw_hex":"e8cef51d00","size":5,"soft_fail":false,"target_va":5385900272,"va":5383936797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410651f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1960798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383939712,"raw_hex":"e85eeb1d00","size":5,"soft_fail":false,"target_va":5385900528,"va":5383939725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410656f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1961998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383939712,"raw_hex":"e80ef01d00","size":5,"soft_fail":false,"target_va":5385901808,"va":5383939805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141065db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1964327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383939104,"raw_hex":"e827f91d00","size":5,"soft_fail":false,"target_va":5385903536,"va":5383939204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141066470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1965988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383939104,"raw_hex":"e8a4ff1d00","size":5,"soft_fail":false,"target_va":5385905264,"va":5383939271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410679a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1973817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936784,"raw_hex":"e8391e1e00","size":5,"soft_fail":false,"target_va":5385910688,"va":5383936866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209fb90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18980836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383936784,"raw_hex":"e8e49f2101","size":5,"soft_fail":false,"target_va":5402917776,"va":5383936935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":23224,"parent_record_offset":22764,"ancestor_record_offset":22764,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":18548,"parent_record_offset":18256,"ancestor_record_offset":18256,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":18660,"parent_record_offset":18256,"ancestor_record_offset":18256,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":18576,"parent_record_offset":18256,"ancestor_record_offset":18256,"inlinee":25270,"source_line":2135,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1326,"record_offset":18520,"parent_record_offset":18256,"ancestor_record_offset":18256,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1326,"record_offset":23116,"parent_record_offset":22764,"ancestor_record_offset":22764,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":24008,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":18624,"parent_record_offset":18600,"ancestor_record_offset":18256,"inlinee":25271,"source_line":1905,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$26::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":23140,"parent_record_offset":23116,"ancestor_record_offset":22764,"inlinee":25269,"source_line":2142,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1326,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":22764,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1326,"record_offset":18600,"parent_record_offset":18576,"ancestor_record_offset":18256,"inlinee":9831,"source_line":33,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1326,"record_offset":23188,"parent_record_offset":23164,"ancestor_record_offset":22764,"inlinee":61944,"source_line":12,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1326,"record_offset":23332,"parent_record_offset":23308,"ancestor_record_offset":22764,"inlinee":27409,"source_line":91,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1FC
// 首条位置: {"module":1421,"inlinee":61948}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":3608,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":3636,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":3664,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":25270,"source_line":2135,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":3556,"inlinee":25271,"source_line":1905,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":3692,"parent_record_offset":3664,"ancestor_record_offset":3556,"inlinee":9831,"source_line":33,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BCC
// 首条位置: {"module":1326,"inlinee":27596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1326,"record_offset":19496,"rva":15228080,"va":5383937200}
// 编译实例 RVAs: 0xe85cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11623268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e89ca44eff","size":5,"soft_fail":false,"target_va":5372314208,"va":5383937471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11187707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e8054a55ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383937414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141065370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1963553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e821f61d00","size":5,"soft_fail":false,"target_va":5385900912,"va":5383937354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141065510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1963792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e810f71d00","size":5,"soft_fail":false,"target_va":5385901328,"va":5383937531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141067f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1974815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e81f221e00","size":5,"soft_fail":false,"target_va":5385912064,"va":5383937244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19019824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e830382201","size":5,"soft_fail":false,"target_va":5402957568,"va":5383937739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19041099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e84b8b2201","size":5,"soft_fail":false,"target_va":5402978944,"va":5383937840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e834dfffff","size":5,"soft_fail":false,"target_va":5383929056,"va":5383937447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5969804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383937200,"raw_hex":"e874e8a4ff","size":5,"soft_fail":false,"target_va":5377967856,"va":5383937655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":20904,"parent_record_offset":19496,"ancestor_record_offset":19496,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":19736,"parent_record_offset":19496,"ancestor_record_offset":19496,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":20148,"parent_record_offset":20124,"ancestor_record_offset":19496,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1326,"record_offset":20124,"parent_record_offset":20096,"ancestor_record_offset":19496,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1326,"record_offset":20096,"parent_record_offset":20068,"ancestor_record_offset":19496,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1326,"record_offset":21020,"parent_record_offset":20976,"ancestor_record_offset":19496,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1326,"record_offset":20944,"parent_record_offset":20904,"ancestor_record_offset":19496,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1326,"record_offset":19916,"parent_record_offset":19736,"ancestor_record_offset":19496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":19880,"parent_record_offset":19736,"ancestor_record_offset":19496,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1326,"record_offset":19828,"parent_record_offset":19736,"ancestor_record_offset":19496,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":19856,"parent_record_offset":19736,"ancestor_record_offset":19496,"inlinee":61943,"source_line":265,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1326,"record_offset":20976,"parent_record_offset":20944,"ancestor_record_offset":19496,"inlinee":27609,"source_line":280,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":19772,"parent_record_offset":19736,"ancestor_record_offset":19496,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1326,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":19496,"inlinee":51267,"source_line":2061,"archive":"_pdb/A/modules/01326.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E95
// 首条位置: {"module":55,"inlinee":7829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$6::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x624A
// 首条位置: {"module":435,"record_offset":54360,"rva":6139120,"va":5374848240}
// 编译实例 RVAs: 0x5dacf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2737552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374848240,"raw_hex":"e890c52900","size":5,"soft_fail":false,"target_va":5377585888,"va":5374848331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$9::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D22
// 首条位置: {"module":2269,"inlinee":81186}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::circle::impl$6::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":2269,"record_offset":15928,"parent_record_offset":15800,"ancestor_record_offset":15800,"inlinee":81190,"source_line":12,"archive":"_pdb/A/modules/02269.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2269,"record_offset":15832,"parent_record_offset":15800,"ancestor_record_offset":15800,"inlinee":81188,"source_line":91,"archive":"_pdb/A/modules/02269.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2269,"record_offset":15868,"parent_record_offset":15832,"ancestor_record_offset":15800,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/02269.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::set_center
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BCD
// 首条位置: {"module":1390,"inlinee":27597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=12
// 源码行: 79, 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C26
// 首条位置: {"module":411,"inlinee":23590}
// 编译实例 RVAs: 0x6b0920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C1C
// 首条位置: {"module":411,"inlinee":23580}
// 编译实例 RVAs: 0x6b0930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2245490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723824,"raw_hex":"e872432200","size":5,"soft_fail":false,"target_va":5377969360,"va":5375723865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2230212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723824,"raw_hex":"e8c4072200","size":5,"soft_fail":false,"target_va":5377954064,"va":5375723847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":491,"record_offset":31312,"rva":7014768,"va":5375723888}
// 编译实例 RVAs: 0x6b0970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2245612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723888,"raw_hex":"e8ec432200","size":5,"soft_fail":false,"target_va":5377969552,"va":5375723935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2230734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723888,"raw_hex":"e8ce092200","size":5,"soft_fail":false,"target_va":5377954656,"va":5375723917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=16
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x59AE
// 首条位置: {"module":399,"inlinee":22958}
// 编译实例 RVAs: 0x6b09b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2246075","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375723952,"raw_hex":"e9bb452200","size":5,"soft_fail":false,"target_va":5377970048,"va":5375723968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":491,"record_offset":31616,"rva":7014864,"va":5375723984}
// 编译实例 RVAs: 0x6b09d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t2245864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723984,"raw_hex":"e8e8442200","size":5,"soft_fail":false,"target_va":5377970048,"va":5375724179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t2243614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723984,"raw_hex":"e81e3c2200","size":5,"soft_fail":false,"target_va":5377967920,"va":5375724301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t2235543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723984,"raw_hex":"e8971c2200","size":5,"soft_fail":false,"target_va":5377959584,"va":5375724036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t2231605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375723984,"raw_hex":"e8350d2200","size":5,"soft_fail":false,"target_va":5377955632,"va":5375724022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2245753","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375723984,"raw_hex":"e979442200","size":5,"soft_fail":false,"target_va":5377970048,"va":5375724290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2235432","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375723984,"raw_hex":"e9281c2200","size":5,"soft_fail":false,"target_va":5377959584,"va":5375724147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":491,"record_offset":31772,"rva":7015584,"va":5375724704}
// 编译实例 RVAs: 0x6b0ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2243659","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375724704,"raw_hex":"e94b3c2200","size":5,"soft_fail":false,"target_va":5377968384,"va":5375724720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":491,"record_offset":31928,"rva":7015616,"va":5375724736}
// 编译实例 RVAs: 0x6b0cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2243163","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375724736,"raw_hex":"e95b3a2200","size":5,"soft_fail":false,"target_va":5377967920,"va":5375724752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C1D
// 首条位置: {"module":411,"inlinee":23581}
// 编译实例 RVAs: 0x6b0ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2243675","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375724768,"raw_hex":"e95b3c2200","size":5,"soft_fail":false,"target_va":5377968464,"va":5375724784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":491,"record_offset":32228,"rva":7015680,"va":5375724800}
// 编译实例 RVAs: 0x6b0d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::get_closest_u_from_pos
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t2243211","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375724800,"raw_hex":"e98b3a2200","size":5,"soft_fail":false,"target_va":5377968032,"va":5375724816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::expand
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C1B
// 首条位置: {"module":411,"inlinee":23579}
// 编译实例 RVAs: 0x6b0d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BD5
// 首条位置: {"module":491,"record_offset":32540,"rva":7015792,"va":5375724912}
// 编译实例 RVAs: 0x6b0d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27253953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375724912,"raw_hex":"e8c1dc9f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5375724986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":491,"record_offset":32696,"parent_record_offset":32540,"ancestor_record_offset":32540,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BD6
// 首条位置: {"module":491,"record_offset":32728,"rva":7015872,"va":5375724992}
// 编译实例 RVAs: 0x6b0dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27253873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375724992,"raw_hex":"e871dc9f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5375725066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":491,"record_offset":32888,"parent_record_offset":32728,"ancestor_record_offset":32728,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE0
// 首条位置: {"module":1326,"inlinee":27616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1FB
// 首条位置: {"module":1421,"inlinee":61947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1FA
// 首条位置: {"module":1421,"inlinee":61946}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1421,"record_offset":19580,"parent_record_offset":19560,"ancestor_record_offset":19560,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1421,"record_offset":19604,"parent_record_offset":19580,"ancestor_record_offset":19560,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":19628,"parent_record_offset":19604,"ancestor_record_offset":19560,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1421,"record_offset":19652,"parent_record_offset":19628,"ancestor_record_offset":19560,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE2
// 首条位置: {"module":1326,"inlinee":27618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10241
// 首条位置: {"module":1421,"inlinee":66113}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":5768,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":6760,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":6792,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":4964,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":5800,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":4924,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":6368,"parent_record_offset":6332,"ancestor_record_offset":4924,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":5368,"parent_record_offset":5292,"ancestor_record_offset":4924,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":5292,"parent_record_offset":5216,"ancestor_record_offset":4924,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1421,"record_offset":5524,"parent_record_offset":5368,"ancestor_record_offset":4924,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":5444,"parent_record_offset":5368,"ancestor_record_offset":4924,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1421,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":4924,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":5648,"parent_record_offset":5124,"ancestor_record_offset":4924,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":4924,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":5548,"parent_record_offset":5524,"ancestor_record_offset":4924,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1421,"record_offset":5720,"parent_record_offset":5692,"ancestor_record_offset":4924,"inlinee":66109,"source_line":265,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1421,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":4924,"inlinee":24930,"source_line":16,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":6208,"parent_record_offset":6140,"ancestor_record_offset":4924,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":6140,"parent_record_offset":6072,"ancestor_record_offset":4924,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":4924,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1421,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":4924,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1421,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":4924,"inlinee":11581,"source_line":126,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":5044,"parent_record_offset":4964,"ancestor_record_offset":4924,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":5868,"parent_record_offset":5800,"ancestor_record_offset":4924,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":5692,"parent_record_offset":5124,"ancestor_record_offset":4924,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":6072,"parent_record_offset":6004,"ancestor_record_offset":4924,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":6332,"parent_record_offset":6208,"ancestor_record_offset":4924,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":6532,"parent_record_offset":6208,"ancestor_record_offset":4924,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":6500,"parent_record_offset":6208,"ancestor_record_offset":4924,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":6564,"parent_record_offset":6208,"ancestor_record_offset":4924,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":6468,"parent_record_offset":6208,"ancestor_record_offset":4924,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":5216,"parent_record_offset":5124,"ancestor_record_offset":4924,"inlinee":7217,"source_line":1146,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":5124,"parent_record_offset":5044,"ancestor_record_offset":4924,"inlinee":7249,"source_line":1210,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":5936,"parent_record_offset":5868,"ancestor_record_offset":4924,"inlinee":7251,"source_line":1223,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1421,"record_offset":6432,"parent_record_offset":6332,"ancestor_record_offset":4924,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10242
// 首条位置: {"module":1421,"inlinee":66114}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":7124,"parent_record_offset":7080,"ancestor_record_offset":7080,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":7080,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":7784,"parent_record_offset":7756,"ancestor_record_offset":7080,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":7080,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":7220,"parent_record_offset":7188,"ancestor_record_offset":7080,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":7080,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":7360,"parent_record_offset":7188,"ancestor_record_offset":7080,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1421,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":7080,"inlinee":24930,"source_line":16,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":7080,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":7656,"parent_record_offset":7624,"ancestor_record_offset":7080,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":7156,"parent_record_offset":7124,"ancestor_record_offset":7080,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":7304,"parent_record_offset":7188,"ancestor_record_offset":7080,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":7624,"parent_record_offset":7592,"ancestor_record_offset":7080,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":7756,"parent_record_offset":7688,"ancestor_record_offset":7080,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":7956,"parent_record_offset":7688,"ancestor_record_offset":7080,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":7928,"parent_record_offset":7688,"ancestor_record_offset":7080,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":7872,"parent_record_offset":7688,"ancestor_record_offset":7080,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":7900,"parent_record_offset":7688,"ancestor_record_offset":7080,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":7188,"parent_record_offset":7156,"ancestor_record_offset":7080,"inlinee":7253,"source_line":963,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1421,"record_offset":7840,"parent_record_offset":7756,"ancestor_record_offset":7080,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BD9
// 首条位置: {"module":1326,"inlinee":27609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1326,"record_offset":21020,"parent_record_offset":20976,"ancestor_record_offset":20976,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01326.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::valid_enclosure::IsEnclosed
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>", "constructor": "enum2$", "arguments": ["country_core::systems::wall::valid_enclosure::IsEnclosed"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::valid_enclosure::ShapeParameters
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 17; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>", "constructor": "enum2$", "arguments": ["country_core::systems::wall::valid_enclosure::ShapeParameters"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "ref$", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 58; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle
// 信息 ID: 084998dd30a66e3dfaad39fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c5b0 -> 0x6b0d70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=17344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=18880; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
// 信息 ID: 5008699a4e7d90f8dadee217
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f08c9 -> 0x6b0930 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f0b92 -> 0x6b0930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x571e61 -> 0x6b0930 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x14329cd -> 0x6b0930 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x1b5ba55 -> 0x6b0930 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=13704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=65140; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
// 信息 ID: 6412cfcbd482e201c2ce4ddd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf498b -> 0x6b0970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf4f62 -> 0x6b0970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
// 信息 ID: 82a1ed8eff00ee9238595ff1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea85e -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd379 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501d8b -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd06b -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccfb0c -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccfc96 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd1f4e -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd1f83 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6a4a -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6c47 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x17d580c -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::is_half_dormer @ 0x17d798c -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x17d9bb0 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x1e670a7 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x1e68d58 -> 0x6b0dc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims; module=1461, inline_record=75192; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::center
// 信息 ID: 19dd5cd5c64fc8a7473e9d32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x4e5ace -> 0x6b0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xa520f8 -> 0x6b0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x1988ca8 -> 0x6b0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_beam_count_change_audio::timberframe_beam_count_change_audio @ 0x198fe50 -> 0x6b0920 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=31160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=31280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=474, inline_record=62360; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::pivot_3d; module=493, inline_record=13044; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=1390, inline_record=31756; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::systems::nani_update_stone_floor; module=2830, inline_record=64880; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
// 信息 ID: ac82f6391d91bcbc4673b4ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xa520d9 -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc9338d -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0xc9e2e4 -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9ed4d -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca015e -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0xca31ad -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4d54 -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7b46 -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings @ 0x1b37f15 -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings @ 0x1b3a266 -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6e3ec -> 0x6b09b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=399, inline_record=32468; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=399, inline_record=34720; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=399, inline_record=37016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=434, inline_record=10264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=434, inline_record=10300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls; module=492, inline_record=37444; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_continue_wall::ui_continue_wall; module=492, inline_record=60268; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_continue_wall::ui_continue_wall; module=492, inline_record=60400; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape
// 信息 ID: 5fa8bf1814fab1715e385fd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc587 -> 0x6b09d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc6e0 -> 0x6b09d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::expand
// 信息 ID: dfc2c8db124135471467cb02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xa5207c -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd1780a -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd1787d -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc4f8 -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc51a -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc55e -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc6b5 -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e1062 -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e1079 -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6550 -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a65ec -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17a9da5 -> 0x6b0d20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=13660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=26788; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers; module=1302, inline_record=56412; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
// 信息 ID: 67265ddbf19fd5d0373fd918
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf4907 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd1739d -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17459 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17897 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd178b6 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e181f -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e1843 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5f65 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5f83 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6604 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6627 -> 0x6b0d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
// 信息 ID: 9e35be4e3d88e1e1bb624af1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4c83 -> 0x6b0ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7a5c -> 0x6b0ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0b25 -> 0x6b0ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd00c2 -> 0x6b0ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> @ 0x177574d -> 0x6b0ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u
// 信息 ID: 11f70d15eec3058903585fc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd173a9 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17465 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17d4a -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17dcd -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17e97 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17f13 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18071 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd180fb -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18143 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd181be -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd181f1 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5f95 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5fae -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6af0 -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6b8a -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a6c0d -> 0x6b0cc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
// 信息 ID: 4b670b5dcb1c7fe981c0ebf9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 19 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x4f4614 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x571ec1 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf4934 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf4f08 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17602 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd178ca -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd178d6 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc5b0 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc5e5 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc709 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc73e -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e1869 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e1897 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e1976 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x17a5cdd -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x17d5872 -> 0x6b0ce0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=16044; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::set_center
// 信息 ID: 7e49fcd78321dc6465ae6b45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=1390, inline_record=27928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
// 信息 ID: ec79b03eaf21bd3e2e6b208b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1326, inline_record=14988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1326, inline_record=15256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1326, inline_record=15780; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >; module=1326, inline_record=16696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=1326, inline_record=19736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=1326, inline_record=19856; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=1326, inline_record=20376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1326, inline_record=21256; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 0f8665fed0830c233fffc0cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1326, inline_record=900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
// 信息 ID: 9eeb1058ed396297939e0680
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19720; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: 3d5702768ac4166fa4578a62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: cc3f5984681a28d0577459f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19560; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 5d21f9aa61e4030bac34110e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 7576260fb14e9405e3743392
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=648; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=704; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=1776; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=1832; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=2828; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=2884; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19532; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 706641f81593029a087289c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=548; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=1676; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=2240; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1421, inline_record=2728; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=5720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: dbfa5bb79d0ea441d4eaa069
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1326, inline_record=440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 6b14bea2e472632a72f7152f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=3556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::expecting
// 信息 ID: 00db190aa403c134720fe824
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=1326, inline_record=680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
// 信息 ID: 37d7c0560941f52ea7ed87d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22872; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22924; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24528; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: b42332f0bf862898db4e62af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22980; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=23124; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24636; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 信息 ID: 762cce4aca9ae7853d3e85b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22816; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22844; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24472; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: f92efd703dfe75083aae8506
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22696; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 信息 ID: abd439e1f9f73a121822a8c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1326, inline_record=1136; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1326, inline_record=1372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: b85e7a9fd1a0647d8ea13b1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=22460; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=24116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: 5411220cbfa4ad5bb22cbff4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=23572; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1421, inline_record=25228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >
// 信息 ID: e2215ea64cb8bd2915439cc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::__private::de::content::EnumRefDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x10655e3 -> 0xe85b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x10653a3 -> 0xe86420 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1065543 -> 0xe86680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 1c605655f3d47a1d2b130b58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=4924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 94a25598a10a8578ffa5d64e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=7080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 7c4945a516aaa07be2a60a1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x10bcf58 -> 0xe854b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e402d -> 0xe854b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e40ee -> 0xe854b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::roofs::roof_shape::_::impl$0::deserialize::__Visitor> @ 0x10c037d -> 0xe86020 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e922d -> 0xe86020 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e9f5d -> 0xe86020 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10ea01e -> 0xe86020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >
// 信息 ID: c76c24fe37b1ef5a02530476
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::__private::de::content::ContentRefDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10677bd -> 0xe85cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10677fa -> 0xe85cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1067acd -> 0xe85cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1067b0a -> 0xe85cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >
// 信息 ID: db47a2f730c9bc937df173d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::__private::de::missing_field::MissingFieldDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1067863 -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1067b73 -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x10c4000 -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::roofs::roof_shape::_::impl$0::deserialize::__Visitor> @ 0x10d1091 -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e427b -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e43bb -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10e9459 -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10ea1ab -> 0xe85830 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x10ea2eb -> 0xe85830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize
// 信息 ID: 8a4f9512f58262226ae59c22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=100, inline_record=15144; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1432, inline_record=31896; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1432, inline_record=164800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 261e402dff0db75c23c634d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x148b77 -> 0x1a49a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x15536b -> 0x1a49a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x108e4d7 -> 0xe85260 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x109948b -> 0xe85260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$0::from
// 信息 ID: dffb67541ecbafe6d22a984b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1326, inline_record=16372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$2::from
// 信息 ID: 3d9f86838a20d8c3ac0db799
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=1326, inline_record=20976; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1326, inline_record=22648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$3::clone
// 信息 ID: 958a15b5a939e7d5a1bf9b8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=18724; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=31104; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted; module=435, inline_record=10680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall; module=435, inline_record=17120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=37396; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=38632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=40224; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=44432; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$4::fmt
// 信息 ID: c5db596a562095a61ea7f748
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$6::fmt
// 信息 ID: b2e91b4bf0744470dcfb3380
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1326, inline_record=1580; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=4108, inline_record=428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$9::eq
// 信息 ID: 64add84caff39e34ee842e89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2269, inline_record=15800; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3194, inline_record=1880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
