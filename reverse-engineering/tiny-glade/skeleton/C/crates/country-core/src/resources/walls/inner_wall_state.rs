// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/walls/inner_wall_state.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e762ce13ddd2988ddfe6fb3dd8595847a20d93a3633a76bcd8e5bb5ad95fe880
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: inner_wall_state.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 126, 144, 210, 225, 412, 509, 520, 528, 535, 538, 549, 552, 561, 568, 569, 570, 574, 575, 578, 579, 582, 584, 588, 600, 602, 1636, 1643, 1669, 1682, 1694, 1701, 1720, 1726, 1770, 1802, 1803, 1804, 1812, 2695, 2709, 3273, 4001, 4029
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use zmij; // namespace candidate
//   package identities: [{"name":"zmij","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.23"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 0, 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1669,"record_offset":28772,"rva":25014048,"va":5393723168}
// 编译实例 RVAs: 0x17daf20, 0x17db200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22662355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393723904,"raw_hex":"e8d3cc5901","size":5,"soft_fail":false,"target_va":5416386464,"va":5393724104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1669,"record_offset":28976,"parent_record_offset":28772,"ancestor_record_offset":28772,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":28996,"parent_record_offset":28976,"ancestor_record_offset":28772,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":29016,"parent_record_offset":28996,"ancestor_record_offset":28772,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":29040,"parent_record_offset":29016,"ancestor_record_offset":28772,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 20, 28, 50, 58, 87, 115, 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14261
// 首条位置: {"module":1770,"inlinee":82529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":8312,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":81016,"parent_record_offset":80992,"ancestor_record_offset":80992,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":78452,"parent_record_offset":78428,"ancestor_record_offset":78428,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":241124,"parent_record_offset":241100,"ancestor_record_offset":241100,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":241204,"parent_record_offset":241152,"ancestor_record_offset":241100,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1803,"record_offset":241152,"parent_record_offset":241124,"ancestor_record_offset":241100,"inlinee":82524,"source_line":184,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":8360,"parent_record_offset":8336,"ancestor_record_offset":8312,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":8384,"parent_record_offset":8360,"ancestor_record_offset":8312,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":241176,"parent_record_offset":241152,"ancestor_record_offset":241100,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 20, 28, 50, 58, 87, 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1425B
// 首条位置: {"module":1669,"inlinee":82523}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":8900,"parent_record_offset":8844,"ancestor_record_offset":8844,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":8932,"parent_record_offset":8844,"ancestor_record_offset":8844,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":8872,"parent_record_offset":8844,"ancestor_record_offset":8844,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=6
// 源码行: 20, 28, 50, 58, 87, 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1669,"record_offset":8592,"rva":25005392,"va":5393714512}
// 编译实例 RVAs: 0x17d8d50, 0x17d9670, 0x17d9820, 0x17d99d0, 0x17d9b80, 0x17d9d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t22845708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714512,"raw_hex":"e80c995c01","size":5,"soft_fail":false,"target_va":5416560512,"va":5393714799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde_core..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-10464156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714512,"raw_hex":"e8645460ff","size":5,"soft_fail":false,"target_va":5383250560,"va":5393714711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=18
//     target_aliases=16 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14658569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714512,"raw_hex":"e8f75320ff","size":5,"soft_fail":false,"target_va":5379056256,"va":5393714820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bac110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4010787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714512,"raw_hex":"e823333d00","size":5,"soft_fail":false,"target_va":5397725456,"va":5393714664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bac6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4008579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393718144,"raw_hex":"e8832a3d00","size":5,"soft_fail":false,"target_va":5397726880,"va":5393718296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bad4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4012675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393717712,"raw_hex":"e8833a3d00","size":5,"soft_fail":false,"target_va":5397730544,"va":5393717864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bae070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4016483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393716848,"raw_hex":"e863493d00","size":5,"soft_fail":false,"target_va":5397733488,"va":5393717000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141baef90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4019923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393717280,"raw_hex":"e8d3563d00","size":5,"soft_fail":false,"target_va":5397737360,"va":5393717432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141be7800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4251930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393716848,"raw_hex":"e81ae14000","size":5,"soft_fail":false,"target_va":5397968896,"va":5393716961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141be97d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4259210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393717712,"raw_hex":"e88afd4000","size":5,"soft_fail":false,"target_va":5397977040,"va":5393717825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bea150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4262074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393717280,"raw_hex":"e8ba084100","size":5,"soft_fail":false,"target_va":5397979472,"va":5393717393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bee990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4283338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714512,"raw_hex":"e8ca5b4100","size":5,"soft_fail":false,"target_va":5397997968,"va":5393714625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bef340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4282186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393718144,"raw_hex":"e84a574100","size":5,"soft_fail":false,"target_va":5398000448,"va":5393718257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t22846404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714512,"raw_hex":"e8c49b5c01","size":5,"soft_fail":false,"target_va":5416561184,"va":5393714775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_enum
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1669,"record_offset":8784,"parent_record_offset":8592,"ancestor_record_offset":8592,"inlinee":33271,"source_line":328,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":8900,"parent_record_offset":8844,"ancestor_record_offset":8592,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":8932,"parent_record_offset":8844,"ancestor_record_offset":8592,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":8844,"parent_record_offset":8784,"ancestor_record_offset":8592,"inlinee":82523,"source_line":28,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":8872,"parent_record_offset":8844,"ancestor_record_offset":8592,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=65
// 源码行: 20, 28, 50, 58, 87, 184, 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33A8
// 首条位置: {"module":144,"inlinee":13224}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=36
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":16816,"parent_record_offset":16796,"ancestor_record_offset":16796,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_newtype_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":225,"record_offset":19208,"parent_record_offset":19184,"ancestor_record_offset":19184,"inlinee":13218,"source_line":198,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=git_dependency_definition; count=21
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":225,"record_offset":30864,"parent_record_offset":30836,"ancestor_record_offset":30836,"inlinee":7422,"source_line":188,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":225,"record_offset":31208,"parent_record_offset":31184,"ancestor_record_offset":30836,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":225,"record_offset":31232,"parent_record_offset":31208,"ancestor_record_offset":30836,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":225,"record_offset":31256,"parent_record_offset":31232,"ancestor_record_offset":30836,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":225,"record_offset":31184,"parent_record_offset":31156,"ancestor_record_offset":30836,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":225,"record_offset":31096,"parent_record_offset":31068,"ancestor_record_offset":30836,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":225,"record_offset":31156,"parent_record_offset":31124,"ancestor_record_offset":30836,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":225,"record_offset":31124,"parent_record_offset":31096,"ancestor_record_offset":30836,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":225,"record_offset":30976,"parent_record_offset":30948,"ancestor_record_offset":30836,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":225,"record_offset":31004,"parent_record_offset":30976,"ancestor_record_offset":30836,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":225,"record_offset":31032,"parent_record_offset":31004,"ancestor_record_offset":30836,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":225,"record_offset":30948,"parent_record_offset":30920,"ancestor_record_offset":30836,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":225,"record_offset":30920,"parent_record_offset":30892,"ancestor_record_offset":30836,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":225,"record_offset":31068,"parent_record_offset":31032,"ancestor_record_offset":30836,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":225,"record_offset":31312,"parent_record_offset":31032,"ancestor_record_offset":30836,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":225,"record_offset":31336,"parent_record_offset":31312,"ancestor_record_offset":30836,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$51::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":225,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":19184,"inlinee":7136,"source_line":11,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=36
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":16840,"parent_record_offset":16816,"ancestor_record_offset":16796,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_f32
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":225,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":19184,"inlinee":13165,"source_line":151,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=git_dependency_definition; count=21
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":225,"record_offset":30892,"parent_record_offset":30864,"ancestor_record_offset":30836,"inlinee":7423,"source_line":168,"archive":"_pdb/C/modules/00225.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3393
// 首条位置: {"module":126,"inlinee":13203}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$9::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":16496,"rva":8481472,"va":5377190592}
// 编译实例 RVAs: 0x816ac0, 0xc4d750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t7633446","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377190592,"raw_hex":"e9267a7400","size":5,"soft_fail":false,"target_va":5384824080,"va":5377190629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 28, 50, 58, 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14267
// 首条位置: {"module":1803,"inlinee":82535}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":478496,"parent_record_offset":478460,"ancestor_record_offset":478460,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":478524,"parent_record_offset":478496,"ancestor_record_offset":478460,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":478552,"parent_record_offset":478524,"ancestor_record_offset":478460,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":478584,"parent_record_offset":478552,"ancestor_record_offset":478460,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 28, 50, 58, 87, 115, 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15703
// 首条位置: {"module":1803,"inlinee":87811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$30::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":478356,"parent_record_offset":478328,"ancestor_record_offset":478328,"inlinee":87849,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":409760,"parent_record_offset":409736,"ancestor_record_offset":409736,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":410036,"parent_record_offset":410012,"ancestor_record_offset":409736,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":410060,"parent_record_offset":410036,"ancestor_record_offset":409736,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":410084,"parent_record_offset":410060,"ancestor_record_offset":409736,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":410012,"parent_record_offset":409956,"ancestor_record_offset":409736,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":409984,"parent_record_offset":409956,"ancestor_record_offset":409736,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":409956,"parent_record_offset":409932,"ancestor_record_offset":409736,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":409932,"parent_record_offset":409908,"ancestor_record_offset":409736,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":478524,"parent_record_offset":478496,"ancestor_record_offset":478328,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":409860,"parent_record_offset":409832,"ancestor_record_offset":409736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":478552,"parent_record_offset":478524,"ancestor_record_offset":478328,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":478584,"parent_record_offset":478552,"ancestor_record_offset":478328,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":478496,"parent_record_offset":478460,"ancestor_record_offset":478328,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1803,"record_offset":478460,"parent_record_offset":478412,"ancestor_record_offset":478328,"inlinee":82535,"source_line":58,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":409832,"parent_record_offset":409808,"ancestor_record_offset":409736,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$30::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":1803,"record_offset":478384,"parent_record_offset":478356,"ancestor_record_offset":478328,"inlinee":87850,"source_line":683,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":409784,"parent_record_offset":409760,"ancestor_record_offset":409736,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":409808,"parent_record_offset":409784,"ancestor_record_offset":409736,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$17::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":1028,"rva":12080048,"va":5380789168}
// 编译实例 RVAs: 0xb853b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t4034870","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380789168,"raw_hex":"e936913d00","size":5,"soft_fail":false,"target_va":5384824080,"va":5380789205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$19::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x339B
// 首条位置: {"module":126,"inlinee":13211}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":602,"record_offset":146172,"rva":12932992,"va":5381642112}
// 编译实例 RVAs: 0xc55780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$25::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":748,"rva":12079920,"va":5380789040}
// 编译实例 RVAs: 0xb85330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t4034991","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380789040,"raw_hex":"e9af913d00","size":5,"soft_fail":false,"target_va":5384824080,"va":5380789084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$27::eq
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x339F
// 首条位置: {"module":126,"inlinee":13215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$33::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":101564,"rva":12126848,"va":5380835968}
// 编译实例 RVAs: 0xb90a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3988070","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380835968,"raw_hex":"e966da3c00","size":5,"soft_fail":false,"target_va":5384824080,"va":5380836005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$35::eq
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3397
// 首条位置: {"module":126,"inlinee":13207}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallMaterial::is_wood
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=23
// 源码行: 67, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8108
// 首条位置: {"module":538,"inlinee":33032}
// 编译实例 RVAs: 0x8f9b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallMaterial::is_stone
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":602,"record_offset":146024,"rva":9418016,"va":5378127136}
// 编译实例 RVAs: 0x8fb520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallMaterial::gable_roof_style
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 75, 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F60
// 首条位置: {"module":535,"inlinee":32608}
// 编译实例 RVAs: 0xc55770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=3
// 源码行: 87, 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":126,"record_offset":4996,"rva":4192832,"va":5372901952}
// 编译实例 RVAs: 0x3ffa40, 0x17d9270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10488822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e80af45fff","size":5,"soft_fail":false,"target_va":5383227792,"va":5393716609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2776690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901952,"raw_hex":"e8725e2a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5372901993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4179777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901952,"raw_hex":"e8bf38c0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5372901964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417db6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e836230000","size":5,"soft_fail":false,"target_va":5393725088,"va":5393716069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b818a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3835284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e894853a00","size":5,"soft_fail":false,"target_va":5397551264,"va":5393715975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b82d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3840117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e875983a00","size":5,"soft_fail":false,"target_va":5397556592,"va":5393716470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84570
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3846720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e840b23a00","size":5,"soft_fail":false,"target_va":5397562736,"va":5393716011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3847216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e830b43a00","size":5,"soft_fail":false,"target_va":5397563536,"va":5393716315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3848314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e87ab83a00","size":5,"soft_fail":false,"target_va":5397564544,"va":5393716225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3849109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e895bb3a00","size":5,"soft_fail":false,"target_va":5397565520,"va":5393716406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3849808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e850be3a00","size":5,"soft_fail":false,"target_va":5397566544,"va":5393716731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3852066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e822c73a00","size":5,"soft_fail":false,"target_va":5397568432,"va":5393716361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b87700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3859149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e8cde23a00","size":5,"soft_fail":false,"target_va":5397575424,"va":5393716270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b892b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3865991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e887fd3a00","size":5,"soft_fail":false,"target_va":5397582512,"va":5393716516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b89530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3867100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e8dc013b00","size":5,"soft_fail":false,"target_va":5397583152,"va":5393716047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8b310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3874614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715824,"raw_hex":"e8361f3b00","size":5,"soft_fail":false,"target_va":5397590800,"va":5393716181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t43718636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901952,"raw_hex":"e8ec179b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5372902153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1669,"record_offset":12104,"parent_record_offset":11072,"ancestor_record_offset":11072,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1669,"record_offset":12160,"parent_record_offset":11072,"ancestor_record_offset":11072,"inlinee":28372,"source_line":220,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11276,"parent_record_offset":11072,"ancestor_record_offset":11072,"inlinee":13201,"source_line":180,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default_arch_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":12132,"parent_record_offset":11072,"ancestor_record_offset":11072,"inlinee":38317,"source_line":214,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":126,"record_offset":5188,"parent_record_offset":4996,"ancestor_record_offset":4996,"inlinee":7422,"source_line":188,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":4996,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":4996,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":4996,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":5504,"parent_record_offset":5476,"ancestor_record_offset":4996,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":4996,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":4996,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":5444,"parent_record_offset":5416,"ancestor_record_offset":4996,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":4996,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":5328,"parent_record_offset":5300,"ancestor_record_offset":4996,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":5356,"parent_record_offset":5328,"ancestor_record_offset":4996,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":5272,"parent_record_offset":5244,"ancestor_record_offset":4996,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":126,"record_offset":5244,"parent_record_offset":5216,"ancestor_record_offset":4996,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":5388,"parent_record_offset":5356,"ancestor_record_offset":4996,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":12068,"parent_record_offset":12040,"ancestor_record_offset":11072,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":11072,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":11072,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":126,"record_offset":5632,"parent_record_offset":5356,"ancestor_record_offset":4996,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":126,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":4996,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11516,"parent_record_offset":11488,"ancestor_record_offset":11072,"inlinee":13203,"source_line":20,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11884,"parent_record_offset":11856,"ancestor_record_offset":11072,"inlinee":13211,"source_line":28,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$27::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11976,"parent_record_offset":11948,"ancestor_record_offset":11072,"inlinee":13215,"source_line":50,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$35::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11700,"parent_record_offset":11672,"ancestor_record_offset":11072,"inlinee":13207,"source_line":58,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11608,"parent_record_offset":11580,"ancestor_record_offset":11072,"inlinee":13205,"source_line":87,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$53::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":11072,"inlinee":13213,"source_line":184,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$60::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":11072,"inlinee":13209,"source_line":188,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":126,"record_offset":5216,"parent_record_offset":5188,"ancestor_record_offset":4996,"inlinee":7423,"source_line":168,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$41::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":101288,"rva":12126672,"va":5380835792}
// 编译实例 RVAs: 0xb909d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3988249","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380835792,"raw_hex":"e919db3c00","size":5,"soft_fail":false,"target_va":5384824080,"va":5380835826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$43::eq
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3395
// 首条位置: {"module":126,"inlinee":13205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 100, 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7780
// 首条位置: {"module":520,"inlinee":30592}
// 编译实例 RVAs: 0xb05d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 107, 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7209
// 首条位置: {"module":509,"inlinee":29193}
// 编译实例 RVAs: 0x8f9b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x95BA
// 首条位置: {"module":1669,"inlinee":38330}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15769
// 首条位置: {"module":1804,"inlinee":87913}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":124580,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":129000,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":124544,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":129776,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":128936,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":129832,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":130252,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":124488,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":124608,"parent_record_offset":124444,"ancestor_record_offset":124444,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":129236,"parent_record_offset":129212,"ancestor_record_offset":124444,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":129260,"parent_record_offset":129236,"ancestor_record_offset":124444,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":129284,"parent_record_offset":129260,"ancestor_record_offset":124444,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":129212,"parent_record_offset":129112,"ancestor_record_offset":124444,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":129144,"parent_record_offset":129112,"ancestor_record_offset":124444,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":129112,"parent_record_offset":129084,"ancestor_record_offset":124444,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":129084,"parent_record_offset":129056,"ancestor_record_offset":124444,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":124724,"parent_record_offset":124700,"ancestor_record_offset":124444,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":129180,"parent_record_offset":129144,"ancestor_record_offset":124444,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":124700,"parent_record_offset":124664,"ancestor_record_offset":124444,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":129028,"parent_record_offset":129000,"ancestor_record_offset":124444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":126276,"parent_record_offset":126224,"ancestor_record_offset":124444,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":124808,"parent_record_offset":124784,"ancestor_record_offset":124444,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1804,"record_offset":126224,"parent_record_offset":126200,"ancestor_record_offset":124444,"inlinee":35346,"source_line":242,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":125352,"parent_record_offset":125328,"ancestor_record_offset":124444,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":125400,"parent_record_offset":125376,"ancestor_record_offset":124444,"inlinee":82524,"source_line":184,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":124784,"parent_record_offset":124760,"ancestor_record_offset":124444,"inlinee":82540,"source_line":115,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":128356,"parent_record_offset":128332,"ancestor_record_offset":124444,"inlinee":82538,"source_line":115,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":129708,"parent_record_offset":129684,"ancestor_record_offset":124444,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":129684,"parent_record_offset":129660,"ancestor_record_offset":124444,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":129732,"parent_record_offset":129708,"ancestor_record_offset":124444,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":124512,"parent_record_offset":124488,"ancestor_record_offset":124444,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":124636,"parent_record_offset":124608,"ancestor_record_offset":124444,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":126756,"parent_record_offset":126732,"ancestor_record_offset":124444,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":124760,"parent_record_offset":124664,"ancestor_record_offset":124444,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":128076,"parent_record_offset":128052,"ancestor_record_offset":124444,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":126248,"parent_record_offset":126224,"ancestor_record_offset":124444,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":125068,"parent_record_offset":125044,"ancestor_record_offset":124444,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":124664,"parent_record_offset":124636,"ancestor_record_offset":124444,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":125376,"parent_record_offset":125352,"ancestor_record_offset":124444,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1426C
// 首条位置: {"module":1803,"inlinee":82540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":124808,"parent_record_offset":124784,"ancestor_record_offset":124784,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1426A
// 首条位置: {"module":1803,"inlinee":82538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":128380,"parent_record_offset":128356,"ancestor_record_offset":128356,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156B3
// 首条位置: {"module":1803,"inlinee":87731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":207448,"parent_record_offset":206736,"ancestor_record_offset":206736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":209332,"parent_record_offset":206736,"ancestor_record_offset":206736,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":206784,"parent_record_offset":206736,"ancestor_record_offset":206736,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":207656,"parent_record_offset":207632,"ancestor_record_offset":206736,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":207680,"parent_record_offset":207656,"ancestor_record_offset":206736,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":207704,"parent_record_offset":207680,"ancestor_record_offset":206736,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":207632,"parent_record_offset":207544,"ancestor_record_offset":206736,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":207572,"parent_record_offset":207544,"ancestor_record_offset":206736,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":207544,"parent_record_offset":207520,"ancestor_record_offset":206736,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":207520,"parent_record_offset":207496,"ancestor_record_offset":206736,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":206872,"parent_record_offset":206840,"ancestor_record_offset":206736,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1803,"record_offset":207600,"parent_record_offset":207572,"ancestor_record_offset":206736,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":207472,"parent_record_offset":207448,"ancestor_record_offset":206736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":206928,"parent_record_offset":206872,"ancestor_record_offset":206736,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":206956,"parent_record_offset":206872,"ancestor_record_offset":206736,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":206900,"parent_record_offset":206872,"ancestor_record_offset":206736,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":206980,"parent_record_offset":206956,"ancestor_record_offset":206736,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":207408,"parent_record_offset":207172,"ancestor_record_offset":206736,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":207064,"parent_record_offset":207040,"ancestor_record_offset":206736,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1803,"record_offset":207040,"parent_record_offset":207016,"ancestor_record_offset":206736,"inlinee":87733,"source_line":115,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":206812,"parent_record_offset":206784,"ancestor_record_offset":206736,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":207016,"parent_record_offset":206840,"ancestor_record_offset":206736,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":207376,"parent_record_offset":207352,"ancestor_record_offset":206736,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":206840,"parent_record_offset":206812,"ancestor_record_offset":206736,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156B5
// 首条位置: {"module":1803,"inlinee":87733}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":207064,"parent_record_offset":207040,"ancestor_record_offset":207040,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156B7
// 首条位置: {"module":1803,"inlinee":87735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":243424,"parent_record_offset":243396,"ancestor_record_offset":243396,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33AC
// 首条位置: {"module":1636,"inlinee":13228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=4
// 源码行: 115, 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":126,"record_offset":2960,"rva":4191936,"va":5372901056}
// 编译实例 RVAs: 0x193ae0, 0x3ff6c0, 0x17d8f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$f32$u20$as$u20$zmij..private..Sealed$GT$::write_to_zmij_buffer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12893503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362592,"raw_hex":"e83fbdc400","size":5,"soft_fail":false,"target_va":5383256144,"va":5370362636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1073782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e88a9defff","size":5,"soft_fail":false,"target_va":5371827984,"va":5372901761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045e070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t387323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8fbe80500","size":5,"soft_fail":false,"target_va":5373288560,"va":5372901232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t391399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8e7f80500","size":5,"soft_fail":false,"target_va":5373293056,"va":5372901652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t414397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8bd520600","size":5,"soft_fail":false,"target_va":5373315680,"va":5372901278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t415643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e89b570600","size":5,"soft_fail":false,"target_va":5373316992,"va":5372901344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5226004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e814be4f00","size":5,"soft_fail":false,"target_va":5378127136,"va":5372901127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b1780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t886761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714944,"raw_hex":"e8e9870d00","size":5,"soft_fail":false,"target_va":5394601856,"va":5393715090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b95c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t918660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714944,"raw_hex":"e884040e00","size":5,"soft_fail":false,"target_va":5394634176,"va":5393715511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t921531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714944,"raw_hex":"e8bb0f0e00","size":5,"soft_fail":false,"target_va":5394636672,"va":5393715136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bbd80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t929145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714944,"raw_hex":"e8792d0e00","size":5,"soft_fail":false,"target_va":5394644352,"va":5393715202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45990305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362592,"raw_hex":"e8a1c1bd02","size":5,"soft_fail":false,"target_va":5416353040,"va":5370362730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43656490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e82a259a02","size":5,"soft_fail":false,"target_va":5416557744,"va":5372901249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46694369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362592,"raw_hex":"e8e17fc802","size":5,"soft_fail":false,"target_va":5417057056,"va":5370362682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::is_default_arch_height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8741023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e89f608500","size":5,"soft_fail":false,"target_va":5381642128,"va":5372901100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::decorator::ClutterVersion>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e88b060000","size":5,"soft_fail":false,"target_va":5372903200,"va":5372901520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::ArchType>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8a1060000","size":5,"soft_fail":false,"target_va":5372903104,"va":5372901402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::FreehandHeightType>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8ed050000","size":5,"soft_fail":false,"target_va":5372903072,"va":5372901550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e858040000","size":5,"soft_fail":false,"target_va":5372903040,"va":5372901923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::StructureType>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e871060000","size":5,"soft_fail":false,"target_va":5372903136,"va":5372901482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallMaterial>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8ed060000","size":5,"soft_fail":false,"target_va":5372903232,"va":5372901454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallStyle>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e8c9060000","size":5,"soft_fail":false,"target_va":5372903168,"va":5372901426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WoodBoardLayout>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372901056,"raw_hex":"e89d060000","size":5,"soft_fail":false,"target_va":5372903296,"va":5372901598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1669,"record_offset":10768,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":28372,"source_line":220,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3256,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":13201,"source_line":180,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default_arch_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":10740,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":38317,"source_line":214,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":7108,"parent_record_offset":6824,"ancestor_record_offset":6824,"inlinee":7390,"source_line":225,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4084,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4784,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4628,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":4480,"parent_record_offset":4448,"ancestor_record_offset":2960,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":4348,"parent_record_offset":4320,"ancestor_record_offset":2960,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":7816,"parent_record_offset":7788,"ancestor_record_offset":6824,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":4408,"parent_record_offset":4376,"ancestor_record_offset":2960,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":2960,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2960,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":4376,"parent_record_offset":4348,"ancestor_record_offset":2960,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":4280,"parent_record_offset":4252,"ancestor_record_offset":2960,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":4448,"parent_record_offset":4280,"ancestor_record_offset":2960,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":4224,"parent_record_offset":4196,"ancestor_record_offset":2960,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":4568,"parent_record_offset":4280,"ancestor_record_offset":2960,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":4320,"parent_record_offset":4280,"ancestor_record_offset":2960,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":126,"record_offset":4252,"parent_record_offset":4224,"ancestor_record_offset":2960,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":2960,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":2960,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":2960,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::classify
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":126,"record_offset":7204,"parent_record_offset":7164,"ancestor_record_offset":6824,"inlinee":8143,"source_line":725,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":126,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":6824,"inlinee":7456,"source_line":612,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":126,"record_offset":7868,"parent_record_offset":7656,"ancestor_record_offset":6824,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":126,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":6824,"inlinee":8147,"source_line":793,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$11::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":2960,"inlinee":13203,"source_line":20,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$19::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":2960,"inlinee":13211,"source_line":28,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$27::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3956,"parent_record_offset":3928,"ancestor_record_offset":2960,"inlinee":13215,"source_line":50,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$35::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":2960,"inlinee":13207,"source_line":58,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":2960,"inlinee":13205,"source_line":87,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$53::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":2960,"inlinee":13213,"source_line":184,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$60::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3680,"parent_record_offset":3652,"ancestor_record_offset":2960,"inlinee":13209,"source_line":188,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$51::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":126,"record_offset":7136,"parent_record_offset":7108,"ancestor_record_offset":6824,"inlinee":7136,"source_line":11,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":2960,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":2960,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_f32
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":7232,"parent_record_offset":7164,"ancestor_record_offset":6824,"inlinee":8144,"source_line":1691,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_f32
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":7164,"parent_record_offset":7136,"ancestor_record_offset":6824,"inlinee":8142,"source_line":155,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":2960,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":4812,"parent_record_offset":4784,"ancestor_record_offset":2960,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":126,"record_offset":4196,"parent_record_offset":4168,"ancestor_record_offset":2960,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":126,"record_offset":4168,"parent_record_offset":4140,"ancestor_record_offset":2960,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] zmij::Buffer::format_finite
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"zmij","version":"1.0.23","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zmij-1.0.23/src/lib.rs
//     evidence={"module":126,"record_offset":7260,"parent_record_offset":7232,"ancestor_record_offset":6824,"inlinee":8146,"source_line":1764,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$45::clone
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=45
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7774
// 首条位置: {"module":520,"inlinee":30580}
// 编译实例 RVAs: 0xa42ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3789952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379468512,"raw_hex":"e8802bc6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379468635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3790469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379468512,"raw_hex":"e87b29c6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379469168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10746429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379468512,"raw_hex":"e8c3055cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379468616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37588383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379468512,"raw_hex":"e89f8d3d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379468668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37152194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379468512,"raw_hex":"e8c2e53602","size":5,"soft_fail":false,"target_va":5416620794,"va":5379468595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=30
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":520,"record_offset":24176,"parent_record_offset":22364,"ancestor_record_offset":22364,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=33
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":520,"record_offset":22428,"parent_record_offset":22364,"ancestor_record_offset":22364,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":22920,"parent_record_offset":22896,"ancestor_record_offset":22364,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":22944,"parent_record_offset":22920,"ancestor_record_offset":22364,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":24068,"parent_record_offset":24044,"ancestor_record_offset":22364,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":24092,"parent_record_offset":24068,"ancestor_record_offset":22364,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":22968,"parent_record_offset":22944,"ancestor_record_offset":22364,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":24116,"parent_record_offset":24092,"ancestor_record_offset":22364,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":22896,"parent_record_offset":22712,"ancestor_record_offset":22364,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":24044,"parent_record_offset":23956,"ancestor_record_offset":22364,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":22648,"parent_record_offset":22620,"ancestor_record_offset":22364,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":23984,"parent_record_offset":23956,"ancestor_record_offset":22364,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":23956,"parent_record_offset":23932,"ancestor_record_offset":22364,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":22712,"parent_record_offset":22676,"ancestor_record_offset":22364,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":22676,"parent_record_offset":22648,"ancestor_record_offset":22364,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":23932,"parent_record_offset":23908,"ancestor_record_offset":22364,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":22752,"parent_record_offset":22712,"ancestor_record_offset":22364,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":520,"record_offset":46704,"parent_record_offset":46672,"ancestor_record_offset":46556,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":520,"record_offset":23192,"parent_record_offset":23124,"ancestor_record_offset":22364,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":520,"record_offset":22580,"parent_record_offset":22512,"ancestor_record_offset":22364,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":22552,"parent_record_offset":22512,"ancestor_record_offset":22364,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":22620,"parent_record_offset":22580,"ancestor_record_offset":22364,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":22480,"parent_record_offset":22428,"ancestor_record_offset":22364,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":22512,"parent_record_offset":22480,"ancestor_record_offset":22364,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":520,"record_offset":22776,"parent_record_offset":22752,"ancestor_record_offset":22364,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":520,"record_offset":22856,"parent_record_offset":22800,"ancestor_record_offset":22364,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":520,"record_offset":23688,"parent_record_offset":23636,"ancestor_record_offset":22364,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":520,"record_offset":23636,"parent_record_offset":23192,"ancestor_record_offset":22364,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":520,"record_offset":23840,"parent_record_offset":23192,"ancestor_record_offset":22364,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":520,"record_offset":22800,"parent_record_offset":22776,"ancestor_record_offset":22364,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":520,"record_offset":22828,"parent_record_offset":22800,"ancestor_record_offset":22364,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":520,"record_offset":24012,"parent_record_offset":23984,"ancestor_record_offset":22364,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":520,"record_offset":23024,"parent_record_offset":22580,"ancestor_record_offset":22364,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":520,"record_offset":23048,"parent_record_offset":23024,"ancestor_record_offset":22364,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":520,"record_offset":23884,"parent_record_offset":22428,"ancestor_record_offset":22364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":520,"record_offset":23800,"parent_record_offset":23732,"ancestor_record_offset":22364,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":520,"record_offset":23772,"parent_record_offset":23732,"ancestor_record_offset":22364,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":520,"record_offset":23732,"parent_record_offset":23688,"ancestor_record_offset":22364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=45
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3391
// 首条位置: {"module":126,"inlinee":13201}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3992,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":126,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":3256,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$11::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":3348,"inlinee":13203,"source_line":20,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$19::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":3716,"inlinee":13211,"source_line":28,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$27::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3956,"parent_record_offset":3928,"ancestor_record_offset":3900,"inlinee":13215,"source_line":50,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$35::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":3440,"inlinee":13207,"source_line":58,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$43::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":3532,"inlinee":13205,"source_line":87,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$53::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":3808,"inlinee":13213,"source_line":184,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$60::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":126,"record_offset":3680,"parent_record_offset":3652,"ancestor_record_offset":3624,"inlinee":13209,"source_line":188,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1425C
// 首条位置: {"module":1669,"inlinee":82524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":241204,"parent_record_offset":241152,"ancestor_record_offset":241152,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":241176,"parent_record_offset":241152,"ancestor_record_offset":241152,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$50::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":888,"rva":12079984,"va":5380789104}
// 编译实例 RVAs: 0xb85370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4028510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380789104,"raw_hex":"e85e783d00","size":5,"soft_fail":false,"target_va":5384817664,"va":5380789149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$53::eq
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x339D
// 首条位置: {"module":126,"inlinee":13213}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$60::eq
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3399
// 首条位置: {"module":126,"inlinee":13209}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::StructureType::is_fence
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B2C
// 首条位置: {"module":528,"inlinee":31532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::StructureType::is_wall
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9275
// 首条位置: {"module":588,"inlinee":37493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::is_default_arch_height
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x95AD
// 首条位置: {"module":602,"record_offset":146324,"rva":12933008,"va":5381642128}
// 编译实例 RVAs: 0xc55790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F49
// 首条位置: {"module":535,"inlinee":32585}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":535,"record_offset":69336,"parent_record_offset":69308,"ancestor_record_offset":69308,"inlinee":26924,"source_line":205,"archive":"_pdb/C/modules/00535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E84
// 首条位置: {"module":575,"inlinee":36484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 256, 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F40
// 首条位置: {"module":535,"inlinee":32576}
// 编译实例 RVAs: 0xc557b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
// 记录形式: compiled_procedure; instances=1
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":602,"record_offset":146628,"rva":12933056,"va":5381642176}
// 编译实例 RVAs: 0xc557c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::is_enclosed
// 记录形式: inlinee_declaration; instances=1
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x959E
// 首条位置: {"module":602,"inlinee":38302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x95B4
// 首条位置: {"module":1720,"inlinee":38324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134836,"parent_record_offset":134808,"ancestor_record_offset":134808,"inlinee":38303,"source_line":176,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134860,"parent_record_offset":134836,"ancestor_record_offset":134808,"inlinee":38305,"source_line":74,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1669,"record_offset":17068,"rva":25009888,"va":5393719008}
// 编译实例 RVAs: 0x17d9ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t22841009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393719008,"raw_hex":"e8b1865c01","size":5,"soft_fail":false,"target_va":5416560512,"va":5393719498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_value
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t22841272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393719008,"raw_hex":"e8b8875c01","size":5,"soft_fail":false,"target_va":5416560624,"va":5393719347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18040694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393719008,"raw_hex":"e88ab8ecfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393719393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142dd9100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t23064936","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393719008,"raw_hex":"e968f15f01","size":5,"soft_fail":false,"target_va":5416784128,"va":5393719187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142dd9210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t23065223","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393719008,"raw_hex":"e987f25f01","size":5,"soft_fail":false,"target_va":5416784400,"va":5393719172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1669,"record_offset":17260,"parent_record_offset":17068,"ancestor_record_offset":17068,"inlinee":21143,"source_line":286,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1669,"record_offset":18328,"parent_record_offset":18304,"ancestor_record_offset":17068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1669,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":17068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1669,"record_offset":18376,"parent_record_offset":18352,"ancestor_record_offset":17068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1669,"record_offset":18304,"parent_record_offset":18240,"ancestor_record_offset":17068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1669,"record_offset":18272,"parent_record_offset":18240,"ancestor_record_offset":17068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1669,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":17068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1669,"record_offset":18212,"parent_record_offset":18184,"ancestor_record_offset":17068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":17068,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1669,"record_offset":18128,"parent_record_offset":17344,"ancestor_record_offset":17068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":17068,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":17508,"parent_record_offset":17480,"ancestor_record_offset":17068,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1669,"record_offset":17432,"parent_record_offset":17392,"ancestor_record_offset":17068,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":17316,"parent_record_offset":17260,"ancestor_record_offset":17068,"inlinee":82525,"source_line":327,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1669,"record_offset":17392,"parent_record_offset":17344,"ancestor_record_offset":17068,"inlinee":82526,"source_line":337,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":18840,"parent_record_offset":18752,"ancestor_record_offset":17068,"inlinee":7617,"source_line":1494,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":18784,"parent_record_offset":18752,"ancestor_record_offset":17068,"inlinee":7618,"source_line":1394,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":17344,"parent_record_offset":17260,"ancestor_record_offset":17068,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":18812,"parent_record_offset":18752,"ancestor_record_offset":17068,"inlinee":7616,"source_line":1456,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1669,"record_offset":18724,"parent_record_offset":17260,"ancestor_record_offset":17068,"inlinee":13567,"source_line":1658,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":1669,"record_offset":18752,"parent_record_offset":17260,"ancestor_record_offset":17068,"inlinee":21146,"source_line":538,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 323
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x95B6
// 首条位置: {"module":1669,"inlinee":38326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1425D
// 首条位置: {"module":1669,"inlinee":82525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1425E
// 首条位置: {"module":1669,"inlinee":82526}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1669,"record_offset":17432,"parent_record_offset":17392,"ancestor_record_offset":17392,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1669,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":17392,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":17392,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01669.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1669,"record_offset":17508,"parent_record_offset":17480,"ancestor_record_offset":17392,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01669.jsonl.gz"}
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
// 完整结构及原始类型记录: inner_wall_state.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::ArchType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::FreehandHeightType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde_core::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::PlasterPeelAmount
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde_core::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::StructureType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde_core::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallMaterial
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallMaterial>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WallMaterial"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallMaterial>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WallMaterial"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallMaterial>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::WallMaterial"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallMaterial>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WallMaterial"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::WallMaterial>", "constructor": "serde_core::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::WallMaterial"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 47
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "core::option::Option", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::Drain", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::zip::Zip<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "country_core::resources::walls::WallId", "country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3>", "constructor": "core::slice::iter::impl$171::all", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde_core::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_tuple<serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_json::value::de::impl$4::deserialize_tuple", "arguments": ["serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "ref$", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "ref$", "arguments": ["ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "country_core::resources::walls::inner_walls::impl$1::all::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::next<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>", "constructor": "core::iter::adapters::map::impl$2::next", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>", "country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>", "country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::inner_wall_state::WallState,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::WallId", "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::contains_key<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,country_core::resources::walls::WallId>", "constructor": "hashbrown::map::HashMap::contains_key", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::resources::walls::WallId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::get_mut<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,country_core::resources::walls::WallId>", "constructor": "hashbrown::map::HashMap::get_mut", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::resources::walls::WallId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::resources::walls::inner_wall_state::WallState,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "core::option::Option", "arguments": ["country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> > >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> > >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> > >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$16::deserialize<country_core::resources::walls::inner_wall_state::WallStyle,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$16::deserialize", "arguments": ["country_core::resources::walls::inner_wall_state::WallStyle", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WoodBoardLayout
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WoodBoardLayout>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WoodBoardLayout"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WoodBoardLayout>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::walls::inner_wall_state::WoodBoardLayout"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WoodBoardLayout>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::WoodBoardLayout"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WoodBoardLayout>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WoodBoardLayout"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::WoodBoardLayout>", "constructor": "serde_core::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::WoodBoardLayout"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde_json::value::de::visit_array", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde_json::value::de::impl$3::deserialize_any", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: inner_wall_state.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: inner_wall_state.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 55; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle
// 信息 ID: b56ef59f40b4c4f3a925613f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::color_menu::color_menu @ 0x2179fad -> 0xc55780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::StructureType::is_fence
// 信息 ID: eb6aa4195d86dd8c634f0d71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute; module=528, inline_record=24348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::merge_walls::merge_walls; module=538, inline_record=91316; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls; module=575, inline_record=23696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=61168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::StructureType::is_wall
// 信息 ID: 715008681fa93eb9580c06e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=33444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallMaterial::gable_roof_style
// 信息 ID: 12e1d86a23d3e2dc8325836d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804603 -> 0xc55770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x840a9a -> 0xc55770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x840b89 -> 0xc55770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x2826e12 -> 0xc55770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::update_roof_spatial::update_roof_spatial @ 0x28277b2 -> 0xc55770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_sample::StructureSample::from_wall @ 0x288e4a0 -> 0xc55770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_sample::StructureSample::from_wall @ 0x288e522 -> 0xc55770 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors; module=535, inline_record=97160; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallMaterial::is_stone
// 信息 ID: d41a300f4f4720a6ea290235
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 42 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x3ff707 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x3ff87f -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_settings_file @ 0x47ab7b -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49efea -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f15c -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ebb72 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804063 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804433 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x805207 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8054e7 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804103 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::audio::op_stairs_audio::op_stairs_audio @ 0x8e6ebe -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::audio::op_stairs_audio::op_stairs_audio @ 0x8e78a1 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::audio::op_stairs_audio::op_stairs_audio @ 0x8e6c13 -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::audio::op_stairs_audio::op_stairs_audio @ 0x8e709b -> 0x8fb520 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143f60a -> 0x8fb520 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallMaterial::is_wood
// 信息 ID: e74fb5315ac4796d80848780
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 143 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3525cf -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x367900 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37a586 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_requests<true> @ 0x4432c0 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_requests<false> @ 0x44a41a -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49effb -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f013 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f190 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f1e4 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$2<true> >,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$3<true> >,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$2<true> >,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$3<true> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$2<true> >,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$3<true> >,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$2<true> >,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$3<true> > > > > > > @ 0x5e0b35 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67acf1 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x7294fc -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ebb99 -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f1eab -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff8db -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ffa4d -> 0x8f9b20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle; module=538, inline_record=103704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle; module=538, inline_record=104120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_raise_shape::impl_raise_shape; module=569, inline_record=62504; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=33016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::play_bulldoze_sound; module=570, inline_record=46504; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=570, inline_record=94564; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=23848; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl; module=584, inline_record=9580; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
// 信息 ID: 47bea5ee1c492f90374d96df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=134808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=134964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
// 信息 ID: bad4a8070d2e83787a88130c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x8e19ee -> 0xc557c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::height
// 信息 ID: bee94cd15f13b5d32f40c8e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=64212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::is_enclosed
// 信息 ID: 2b07e53ef5d798e3b86c8ad3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
// 信息 ID: c79a8302ebec332f2920c6c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_changes::StyleScene::sample @ 0x214362a -> 0xc557b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_changes::StyleScene::changes @ 0x21442f3 -> 0xc557b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::remember_color_customization::remember_color_customization @ 0x28afdc4 -> 0xc557b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=45928; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=53264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=64368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=579, inline_record=45980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl; module=584, inline_record=8112; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::new
// 信息 ID: 710e6d5ee115fe6a6fb85795
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::create_circle::create_circle; module=535, inline_record=69308; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle; module=535, inline_record=83336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=30836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=30928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
// 信息 ID: e1a342bfbfe3dfb9c70cda98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::lantern::maintain_lantern_lights @ 0x8470ee -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::window_lights::window_lights @ 0x87179d -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_changes::StyleScene::changes @ 0x2144790 -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x2346c46 -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::process_tree_cmds::process_tree_cmds @ 0x285ecee -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x2860f47 -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_sample::StyleSample::remember @ 0x288e78a -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::remember_color_customization::remember_color_customization @ 0x28afdec -> 0xb05d10 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=34600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=41020; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=77616; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=579, inline_record=46376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze; module=584, inline_record=12764; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
// 信息 ID: eaf47ad094422c7f53fa44e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 48 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36d56e -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37fa9c -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804095 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804744 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x804e03 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x80529e -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x80503f -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x835f6d -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_columns @ 0x836960 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x8408a9 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8527d1 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x880e86 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_graph_and_requests::generate_playermade_stair_decorators @ 0x89aff0 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8be004 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x8c7007 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::should_snap_to_corner @ 0x8d4746 -> 0x8f9b30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio; module=509, inline_record=48236; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio; module=509, inline_record=52524; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=549, inline_record=26672; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=24232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=578, inline_record=53084; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
// 信息 ID: 0206f10840c39a19ecc2a7e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >; module=1803, inline_record=167248; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>; module=1803, inline_record=241100; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=4648; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=8312; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=8872; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=39036; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=39316; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=76204; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
// 信息 ID: f7975585bb9124fa3eabf6a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>; module=1803, inline_record=241152; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=4696; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=80348; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=80716; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=81392; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=125400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: f8e81ab9fb9a3c4df9d30ef4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=478460; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=482680; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=494048; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=514808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: aa67b431ca5ae2923450ca45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> > @ 0x1ba84c3 -> 0x17daf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> > @ 0x1bae0b7 -> 0x17db200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 08466cc8d582da3114a44ef8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=409736; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=478328; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=482548; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=493916; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=503244; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1803, inline_record=514676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 42520a2fe8f70fcae3326099
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1669, inline_record=35604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 96a84c90c3d04e467c6fa6e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=8844; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=14272; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=14652; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=15032; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=15412; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1669, inline_record=16940; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=329896; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=358208; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 9d345b74a69a264a82974d7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=124444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 074356cab7f5aac352d55d89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=124784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 信息 ID: 1a6c59496d5960b60853e556
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=128356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: c93e2e9981a0aae81ecb010a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=206736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: 10d1f7fb9311bae39e27362c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=207040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 信息 ID: ce098c656118e5fcfbf342a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith>; module=1803, inline_record=243396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 11e771985ac58483e505cd9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x1b9ccb6 -> 0x17d8d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impls::impl$16::deserialize<country_core::resources::walls::inner_wall_state::WallStyle,enum2$<serde_json::value::Value> > @ 0x1adab88 -> 0x17d9670 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor> @ 0x1b999f2 -> 0x17d9670 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::WallMaterial> @ 0x1b9fb0d -> 0x17d9820 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType> @ 0x1b9f53d -> 0x17d99d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::WoodBoardLayout> @ 0x1b9f73d -> 0x17d9b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::FreehandHeightType> @ 0x1b9f5bd -> 0x17d9d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize
// 信息 ID: 839b0dcf1c4d7a366f35f42b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallStyle>; module=144, inline_record=16796; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::FreehandHeightType>; module=144, inline_record=101808; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::StructureType>; module=144, inline_record=133520; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WoodBoardLayout>; module=144, inline_record=289828; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallMaterial>; module=144, inline_record=310660; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::PlasterPeelAmount>; module=225, inline_record=19184; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::FreehandHeightType>; module=225, inline_record=30836; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::FreehandHeightType>; module=225, inline_record=32888; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize::impl$0::serialize
// 信息 ID: 86ddc712775d870b11a92286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::_::impl$0::serialize::__SerializeWith>; module=1802, inline_record=88424; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::_::impl$0::serialize::__SerializeWith>; module=1802, inline_record=89416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: ef12e99b2dfe90de270dffa3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallState> @ 0x461472 -> 0x3ff6c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > > @ 0x46647c -> 0x3ff6c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount> | serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::WallHeight> @ 0x461cb2 -> 0x193ae0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount> | serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::WallHeight> @ 0x18bbd02 -> 0x193ae0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallState> @ 0x18bb072 -> 0x17d8f00 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > > @ 0x18bf33c -> 0x17d8f00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: e5f0112c42992f7ee8e7f453
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> > > @ 0x5d7f41 -> 0x3ffa40 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x5da45b -> 0x3ffa40 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x1b84e0b -> 0x3ffa40 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallStyle> > > @ 0x1b89a81 -> 0x3ffa40 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > > > @ 0x151101b -> 0x17d9270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::expecting
// 信息 ID: 9df0b6e439f17e7f0a89eccd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>; module=1669, inline_record=34728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
// 信息 ID: 651e7a17ca62481165c10aa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >; module=1669, inline_record=17316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
// 信息 ID: a2f01b6fe45a3374428acc38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >; module=1669, inline_record=17392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
// 信息 ID: 8c9261172c201b5555072901
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::deserialize_wall_style -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith> @ 0x1b9f9f0 -> 0x17d9ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$11::eq
// 信息 ID: d6dc18cb6b4c486deb8252ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3404; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9516; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$17::fmt
// 信息 ID: 7128fe6a2e6b2f92e2482784
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$19::eq
// 信息 ID: 815b977accde4eaabacfb6f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3772; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=29728; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9884; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11884; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=47740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$25::fmt
// 信息 ID: 08847be745b4895148d1d752
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$27::eq
// 信息 ID: 39581fd437d0d27ce73f3b59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3956; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio; module=509, inline_record=44792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=29756; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9976; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11976; 不是独立调用边。
// [PDB 内联宿主] system_color::style_changes::StyleScene::apply; module=2695, inline_record=5548; 不是独立调用边。
// [PDB 内联宿主] system_color::style_changes::StyleScene::changes; module=2695, inline_record=12944; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=48208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$33::fmt
// 信息 ID: df26469df2138a766508cd92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$35::eq
// 信息 ID: 8bd11d4f767aa1d49d52c98d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3496; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall::closure$0; module=535, inline_record=52140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::create_circle::create_circle; module=535, inline_record=68704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle; module=535, inline_record=82856; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors; module=535, inline_record=98160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=29116; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9700; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11700; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$41::fmt
// 信息 ID: d68ffe400c9cf446277ba53f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$43::eq
// 信息 ID: b6d4d44de56941b6965bbe5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=29644; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9608; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11608; 不是独立调用边。
// [PDB 内联宿主] system_color::style_changes::StyleScene::apply; module=2695, inline_record=6060; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=43320; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=43388; 不是独立调用边。
// [PDB 内联宿主] system_color::radial_menu::ColorRadialMenu::create; module=4029, inline_record=5816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$45::clone
// 信息 ID: 23d7bce0e52a3a9cf4b607ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c5c60f15599d089557c43d58: enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)
// 引用方: 原程序直接转移 1 条；内联宿主 33 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa3b6bb -> 0xa42ce0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall; module=520, inline_record=22364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=34684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=38216; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=44576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=46556; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=48304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=37284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=42236; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$50::fmt
// 信息 ID: e5961102d5cd59afe0b420b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$53::eq
// 信息 ID: ba9a2a241f845031e3ccf83f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3864; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=552, inline_record=29700; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9424; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11424; 不是独立调用边。
// [PDB 内联宿主] system_color::style_changes::StyleScene::apply; module=2695, inline_record=5688; 不是独立调用边。
// [PDB 内联宿主] system_color::style_changes::StyleScene::changes; module=2695, inline_record=13244; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=2709, inline_record=45904; 不是独立调用边。
// [PDB 内联宿主] system_color::radial_menu::ColorRadialMenu::create; module=4029, inline_record=5404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$60::eq
// 信息 ID: f354de0cd72d8c187c6a3705
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall::closure$0; module=535, inline_record=52112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=29088; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=588, inline_record=38196; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=9792; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=11792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$9::fmt
// 信息 ID: 7087be21bd5a15545341dcbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::is_default
// 信息 ID: 69fc440b862f4201bd7c83fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3256; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3348; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3440; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3532; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3624; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3716; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3808; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=126, inline_record=3900; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::is_default_arch_height
// 信息 ID: 6c05e2d8bf10d74c041c23b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x3ff6ec -> 0xc55790 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x3ff7ff -> 0xc55790 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1669, inline_record=10740; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1669, inline_record=12132; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
