// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/history/decorator_edit.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 417bee2a048ef85fa2c3bf8514f51f549e7e2ee650db42f5afe68e739670cdd8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 144, 153, 501, 536, 540, 564, 568, 582, 594, 607, 611, 612, 614, 1545, 1630, 1643, 1658, 1682, 1687, 1694, 1803, 1804, 1814, 4750
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use vectorize; // namespace candidate
//   package identities: [{"name":"vectorize","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 17, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":37640,"rva":10958256,"va":5379667376}
// 编译实例 RVAs: 0xa735b0, 0xa73680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36718913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667376,"raw_hex":"e841493002","size":5,"soft_fail":false,"target_va":5416386464,"va":5379667546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":540,"record_offset":37844,"parent_record_offset":37640,"ancestor_record_offset":37640,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":37864,"parent_record_offset":37844,"ancestor_record_offset":37640,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":540,"record_offset":37884,"parent_record_offset":37864,"ancestor_record_offset":37640,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":540,"record_offset":37908,"parent_record_offset":37884,"ancestor_record_offset":37640,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 17, 34, 70, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156CF
// 首条位置: {"module":1803,"inlinee":87759}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$30::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":488840,"parent_record_offset":488812,"ancestor_record_offset":488812,"inlinee":87849,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":278468,"parent_record_offset":278436,"ancestor_record_offset":278436,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":279400,"parent_record_offset":279376,"ancestor_record_offset":278436,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":279424,"parent_record_offset":279400,"ancestor_record_offset":278436,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":279448,"parent_record_offset":279424,"ancestor_record_offset":278436,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":279376,"parent_record_offset":279288,"ancestor_record_offset":278436,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":279332,"parent_record_offset":279288,"ancestor_record_offset":278436,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":279288,"parent_record_offset":279248,"ancestor_record_offset":278436,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":279248,"parent_record_offset":279208,"ancestor_record_offset":278436,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":278688,"parent_record_offset":278660,"ancestor_record_offset":278436,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":279128,"parent_record_offset":278564,"ancestor_record_offset":278436,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":278716,"parent_record_offset":278688,"ancestor_record_offset":278436,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":278748,"parent_record_offset":278716,"ancestor_record_offset":278436,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":278660,"parent_record_offset":278616,"ancestor_record_offset":278436,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1803,"record_offset":278616,"parent_record_offset":278564,"ancestor_record_offset":278436,"inlinee":33372,"source_line":473,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":278564,"parent_record_offset":278532,"ancestor_record_offset":278436,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$30::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":1803,"record_offset":488868,"parent_record_offset":488840,"ancestor_record_offset":488812,"inlinee":87850,"source_line":683,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":278500,"parent_record_offset":278468,"ancestor_record_offset":278436,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":278532,"parent_record_offset":278500,"ancestor_record_offset":278436,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":36668,"rva":10957648,"va":5379666768}
// 编译实例 RVAs: 0xa73350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36893181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666768,"raw_hex":"e8fdf13202","size":5,"soft_fail":false,"target_va":5416560512,"va":5379667326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150e8d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11121591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666768,"raw_hex":"e8b7b3a900","size":5,"soft_fail":false,"target_va":5390788816,"va":5379667220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150e990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11121878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666768,"raw_hex":"e8d6b4a900","size":5,"soft_fail":false,"target_va":5390789008,"va":5379667125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150ea80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11122222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666768,"raw_hex":"e82eb6a900","size":5,"soft_fail":false,"target_va":5390789248,"va":5379667021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415107b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11129784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666768,"raw_hex":"e8b8d3a900","size":5,"soft_fail":false,"target_va":5390796720,"va":5379666931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bace80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18062107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379666768,"raw_hex":"e81b9b1301","size":5,"soft_fail":false,"target_va":5397728896,"va":5379666784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":540,"record_offset":37072,"parent_record_offset":36668,"ancestor_record_offset":36668,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":540,"record_offset":36928,"parent_record_offset":36668,"ancestor_record_offset":36668,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":540,"record_offset":36900,"parent_record_offset":36668,"ancestor_record_offset":36668,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":540,"record_offset":36960,"parent_record_offset":36668,"ancestor_record_offset":36668,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":36668,"inlinee":33298,"source_line":28,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":540,"record_offset":37008,"parent_record_offset":36984,"ancestor_record_offset":36668,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$8::newtype_variant_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":540,"record_offset":36984,"parent_record_offset":36960,"ancestor_record_offset":36668,"inlinee":33297,"source_line":563,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":15752,"rva":10946000,"va":5379655120}
// 编译实例 RVAs: 0xa705d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36905173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655120,"raw_hex":"e8d5203302","size":5,"soft_fail":false,"target_va":5416560512,"va":5379655334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-599108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655120,"raw_hex":"e8bcdbf6ff","size":5,"soft_fail":false,"target_va":5379056256,"va":5379655359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a73350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655120,"raw_hex":"e8e72c0000","size":5,"soft_fail":false,"target_va":5379666768,"va":5379655268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bee0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18340462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655120,"raw_hex":"e86eda1701","size":5,"soft_fail":false,"target_va":5397995696,"va":5379655229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t36905869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655120,"raw_hex":"e88d233302","size":5,"soft_fail":false,"target_va":5416561184,"va":5379655310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":540,"record_offset":15944,"parent_record_offset":15752,"ancestor_record_offset":15752,"inlinee":33271,"source_line":328,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=8
// 源码行: 17, 70, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":153,"record_offset":6388,"rva":4844128,"va":5373553248}
// 编译实例 RVAs: 0x49ea60, 0x49eb50, 0x49efc0, 0x49f4d0, 0xa70070, 0xa70160, 0xa70be0, 0xa710e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1726180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553488,"raw_hex":"e81ca9e5ff","size":5,"soft_fail":false,"target_va":5371827984,"va":5373554159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045aea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-277988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553488,"raw_hex":"e81cc2fbff","size":5,"soft_fail":false,"target_va":5373275808,"va":5373553791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045bdb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-276335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373555920,"raw_hex":"e891c8fbff","size":5,"soft_fail":false,"target_va":5373279664,"va":5373555994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045e980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-263146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553488,"raw_hex":"e816fcfbff","size":5,"soft_fail":false,"target_va":5373290880,"va":5373554021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045ea80
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-263865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e847f9fbff","size":5,"soft_fail":false,"target_va":5373291136,"va":5373554996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f200
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-262591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e841fefbff","size":5,"soft_fail":false,"target_va":5373293056,"va":5373555642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140460800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-256266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e8f616fcff","size":5,"soft_fail":false,"target_va":5373298688,"va":5373554949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140460900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-255875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e87d18fcff","size":5,"soft_fail":false,"target_va":5373298944,"va":5373554814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140462f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-244624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553248,"raw_hex":"e87044fcff","size":5,"soft_fail":false,"target_va":5373308704,"va":5373553323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-238389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553488,"raw_hex":"e8cb5cfcff","size":5,"soft_fail":false,"target_va":5373315680,"va":5373554064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046a900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-215011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e81db8fcff","size":5,"soft_fail":false,"target_va":5373339904,"va":5373554910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046b660
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-210303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553488,"raw_hex":"e881cafcff","size":5,"soft_fail":false,"target_va":5373343328,"va":5373553626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4565792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e820ab4500","size":5,"soft_fail":false,"target_va":5378120480,"va":5373554683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4572465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e831c54500","size":5,"soft_fail":false,"target_va":5378127136,"va":5373554666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c7a3b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8237985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e8a1b37d00","size":5,"soft_fail":false,"target_va":5381792688,"va":5373554698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b3cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14953345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379657952,"raw_hex":"e8812be400","size":5,"soft_fail":false,"target_va":5394611376,"va":5379658026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b5ef0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14963162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e8da51e400","size":5,"soft_fail":false,"target_va":5394620144,"va":5379656977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b60f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14963366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e8a652e400","size":5,"soft_fail":false,"target_va":5394620656,"va":5379657285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b78e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14969850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e8fa6be400","size":5,"soft_fail":false,"target_va":5394626784,"va":5379656929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b7e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14971233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e86171e400","size":5,"soft_fail":false,"target_va":5394628128,"va":5379656890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b7f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14970506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e88a6ee400","size":5,"soft_fail":false,"target_va":5394628384,"va":5379657873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b8e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14977798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653984,"raw_hex":"e8068be400","size":5,"soft_fail":false,"target_va":5394632320,"va":5379654517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b95c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14976541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e81d86e400","size":5,"soft_fail":false,"target_va":5394634176,"va":5379657630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14982107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653984,"raw_hex":"e8db9be400","size":5,"soft_fail":false,"target_va":5394636672,"va":5379654560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14994944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653744,"raw_hex":"e800cee400","size":5,"soft_fail":false,"target_va":5394648768,"va":5379653819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bd1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14995244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653984,"raw_hex":"e82ccfe400","size":5,"soft_fail":false,"target_va":5394649536,"va":5379654287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418be600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14997921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379656672,"raw_hex":"e8a1d9e400","size":5,"soft_fail":false,"target_va":5394654720,"va":5379656794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=30
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42799615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553248,"raw_hex":"e8ff118d02","size":5,"soft_fail":false,"target_va":5416353040,"va":5373553420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43003923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373553488,"raw_hex":"e813309002","size":5,"soft_fail":false,"target_va":5416557744,"va":5373553816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::decorator::ClutterVersion>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e88e070000","size":5,"soft_fail":false,"target_va":5373556992,"va":5373555053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::decorator::StairMaterial>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e85c050000","size":5,"soft_fail":false,"target_va":5373556864,"va":5373555487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::decorator::StairRailingType>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e863060000","size":5,"soft_fail":false,"target_va":5373556960,"va":5373555320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::decorator::StairSupportType>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e862060000","size":5,"soft_fail":false,"target_va":5373557024,"va":5373555385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<core::option::Option<country_core::resources::color_id::ColorId> > >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e853060000","size":5,"soft_fail":false,"target_va":5373556896,"va":5373555272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,u32>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t1904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373554624,"raw_hex":"e870070000","size":5,"soft_fail":false,"target_va":5373556928,"va":5373555019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t43002855","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373554624,"raw_hex":"e9e72b9002","size":5,"soft_fail":false,"target_va":5416557744,"va":5373554884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tjmp\t43006001","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373553488,"raw_hex":"e931389002","size":5,"soft_fail":false,"target_va":5416560208,"va":5373554202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":153,"record_offset":13340,"parent_record_offset":11788,"ancestor_record_offset":11788,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":153,"record_offset":12088,"parent_record_offset":11788,"ancestor_record_offset":11788,"inlinee":14659,"source_line":145,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_lantern_brightness
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":18788,"parent_record_offset":17228,"ancestor_record_offset":17228,"inlinee":33272,"source_line":161,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_stair_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":18816,"parent_record_offset":17228,"ancestor_record_offset":17228,"inlinee":33273,"source_line":177,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":18728,"parent_record_offset":17228,"ancestor_record_offset":17228,"inlinee":28372,"source_line":220,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":8044,"parent_record_offset":7760,"ancestor_record_offset":7760,"inlinee":7399,"source_line":233,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":6680,"parent_record_offset":6388,"ancestor_record_offset":6388,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":7212,"parent_record_offset":6388,"ancestor_record_offset":6388,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=34
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":7184,"parent_record_offset":6388,"ancestor_record_offset":6388,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":7036,"parent_record_offset":7004,"ancestor_record_offset":6388,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":6388,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":6388,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":6388,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":6388,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":6388,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":6388,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":7004,"parent_record_offset":6856,"ancestor_record_offset":6388,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6388,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":7124,"parent_record_offset":6856,"ancestor_record_offset":6388,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":6892,"parent_record_offset":6856,"ancestor_record_offset":6388,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":153,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":6388,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":153,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":11788,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":153,"record_offset":12116,"parent_record_offset":12088,"ancestor_record_offset":11788,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":153,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":11788,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":153,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":7760,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":153,"record_offset":12144,"parent_record_offset":12116,"ancestor_record_offset":11788,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":153,"record_offset":10220,"parent_record_offset":9140,"ancestor_record_offset":7760,"inlinee":14657,"source_line":34,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$113::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":153,"record_offset":13472,"parent_record_offset":13436,"ancestor_record_offset":11788,"inlinee":14663,"source_line":364,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":6388,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":9740,"parent_record_offset":9140,"ancestor_record_offset":7760,"inlinee":7401,"source_line":1910,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":7272,"parent_record_offset":7244,"ancestor_record_offset":6388,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":6708,"parent_record_offset":6680,"ancestor_record_offset":6388,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":8560,"parent_record_offset":8044,"ancestor_record_offset":7760,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10252,"parent_record_offset":10220,"ancestor_record_offset":7760,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":7244,"parent_record_offset":7212,"ancestor_record_offset":6388,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10924,"parent_record_offset":10220,"ancestor_record_offset":7760,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10796,"parent_record_offset":10220,"ancestor_record_offset":7760,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":153,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6388,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":153,"record_offset":6760,"parent_record_offset":6732,"ancestor_record_offset":6388,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$8::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97E1
// 首条位置: {"module":611,"inlinee":38881}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13986
// 首条位置: {"module":1545,"inlinee":80262}
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
// 源码行: 25, 28, 31, 34, 70, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8212
// 首条位置: {"module":540,"inlinee":33298}
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
// 源码行: 25, 28, 31, 34, 70, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8222
// 首条位置: {"module":540,"inlinee":33314}
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
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 25, 28, 31, 34, 70, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15756
// 首条位置: {"module":1804,"inlinee":87894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":53692,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":54280,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":53436,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":54252,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":54768,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":55624,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":131632,"parent_record_offset":130924,"ancestor_record_offset":130924,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":53380,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":53472,"parent_record_offset":53348,"ancestor_record_offset":53348,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":54644,"parent_record_offset":54616,"ancestor_record_offset":53348,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":54672,"parent_record_offset":54644,"ancestor_record_offset":53348,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":54704,"parent_record_offset":54672,"ancestor_record_offset":53348,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":54616,"parent_record_offset":54428,"ancestor_record_offset":53348,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":53588,"parent_record_offset":53564,"ancestor_record_offset":53348,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":54520,"parent_record_offset":54492,"ancestor_record_offset":53348,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":54548,"parent_record_offset":54520,"ancestor_record_offset":53348,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":53564,"parent_record_offset":53528,"ancestor_record_offset":53348,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":54308,"parent_record_offset":54280,"ancestor_record_offset":53348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1804,"record_offset":54584,"parent_record_offset":54460,"ancestor_record_offset":53348,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":295048,"parent_record_offset":294996,"ancestor_record_offset":290872,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":120728,"parent_record_offset":120676,"ancestor_record_offset":119292,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1804,"record_offset":292580,"parent_record_offset":292556,"ancestor_record_offset":290872,"inlinee":35724,"source_line":28,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1804,"record_offset":292628,"parent_record_offset":292604,"ancestor_record_offset":290872,"inlinee":35659,"source_line":28,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1804,"record_offset":131444,"parent_record_offset":131420,"ancestor_record_offset":130924,"inlinee":33379,"source_line":473,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1804,"record_offset":120628,"parent_record_offset":120604,"ancestor_record_offset":119292,"inlinee":35346,"source_line":242,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1804,"record_offset":120676,"parent_record_offset":120652,"ancestor_record_offset":119292,"inlinee":35338,"source_line":155,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1804,"record_offset":54460,"parent_record_offset":54428,"ancestor_record_offset":53348,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1804,"record_offset":54392,"parent_record_offset":54364,"ancestor_record_offset":53348,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1804,"record_offset":54428,"parent_record_offset":54392,"ancestor_record_offset":53348,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1804,"record_offset":55256,"parent_record_offset":55228,"ancestor_record_offset":53348,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1804,"record_offset":54492,"parent_record_offset":54460,"ancestor_record_offset":53348,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1804,"record_offset":54364,"parent_record_offset":54336,"ancestor_record_offset":53348,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":54840,"parent_record_offset":54816,"ancestor_record_offset":53348,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":297052,"parent_record_offset":297028,"ancestor_record_offset":290872,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":121032,"parent_record_offset":121008,"ancestor_record_offset":119292,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":54816,"parent_record_offset":54792,"ancestor_record_offset":53348,"inlinee":34987,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":120892,"parent_record_offset":120868,"ancestor_record_offset":119292,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":297332,"parent_record_offset":297308,"ancestor_record_offset":290872,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":297472,"parent_record_offset":297448,"ancestor_record_offset":290872,"inlinee":35640,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":54864,"parent_record_offset":54840,"ancestor_record_offset":53348,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":53404,"parent_record_offset":53380,"ancestor_record_offset":53348,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":53500,"parent_record_offset":53472,"ancestor_record_offset":53348,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":53648,"parent_record_offset":53624,"ancestor_record_offset":53348,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=33
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":53624,"parent_record_offset":53528,"ancestor_record_offset":53348,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":295376,"parent_record_offset":295352,"ancestor_record_offset":290872,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":295328,"parent_record_offset":295304,"ancestor_record_offset":290872,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":53896,"parent_record_offset":53872,"ancestor_record_offset":53348,"inlinee":83339,"source_line":1515,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":293480,"parent_record_offset":293456,"ancestor_record_offset":290872,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":295020,"parent_record_offset":294996,"ancestor_record_offset":290872,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":120700,"parent_record_offset":120676,"ancestor_record_offset":119292,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":120092,"parent_record_offset":120068,"ancestor_record_offset":119292,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=34
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":53528,"parent_record_offset":53500,"ancestor_record_offset":53348,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":120652,"parent_record_offset":120628,"ancestor_record_offset":119292,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":295352,"parent_record_offset":295328,"ancestor_record_offset":290872,"inlinee":85976,"source_line":317,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$11::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x821E
// 首条位置: {"module":540,"inlinee":33310}
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
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8214
// 首条位置: {"module":540,"inlinee":33300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$15::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x821A
// 首条位置: {"module":540,"inlinee":33306}
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
// 源码行: 34, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x825C
// 首条位置: {"module":1803,"inlinee":33372}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":278660,"parent_record_offset":278616,"ancestor_record_offset":278616,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":278688,"parent_record_offset":278660,"ancestor_record_offset":278616,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":278716,"parent_record_offset":278688,"ancestor_record_offset":278616,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":278748,"parent_record_offset":278716,"ancestor_record_offset":278616,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 34, 70, 473, 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156A3
// 首条位置: {"module":1803,"inlinee":87715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":57792,"parent_record_offset":57060,"ancestor_record_offset":57060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":80872,"parent_record_offset":78124,"ancestor_record_offset":78124,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":58284,"parent_record_offset":57060,"ancestor_record_offset":57060,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":57096,"parent_record_offset":57060,"ancestor_record_offset":57060,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":58160,"parent_record_offset":58132,"ancestor_record_offset":57060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":58188,"parent_record_offset":58160,"ancestor_record_offset":57060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":58220,"parent_record_offset":58188,"ancestor_record_offset":57060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":58132,"parent_record_offset":57944,"ancestor_record_offset":57060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1803,"record_offset":187860,"parent_record_offset":187776,"ancestor_record_offset":187468,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":57192,"parent_record_offset":57160,"ancestor_record_offset":57060,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1803,"record_offset":58036,"parent_record_offset":58008,"ancestor_record_offset":57060,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1803,"record_offset":58064,"parent_record_offset":58036,"ancestor_record_offset":57060,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":57816,"parent_record_offset":57792,"ancestor_record_offset":57060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1803,"record_offset":187896,"parent_record_offset":187860,"ancestor_record_offset":187468,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1803,"record_offset":58100,"parent_record_offset":57976,"ancestor_record_offset":57060,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":57248,"parent_record_offset":57192,"ancestor_record_offset":57060,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":57276,"parent_record_offset":57192,"ancestor_record_offset":57060,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":57220,"parent_record_offset":57192,"ancestor_record_offset":57060,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":57300,"parent_record_offset":57276,"ancestor_record_offset":57060,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":57392,"parent_record_offset":57160,"ancestor_record_offset":57060,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":79472,"parent_record_offset":79420,"ancestor_record_offset":78124,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1803,"record_offset":79372,"parent_record_offset":79348,"ancestor_record_offset":78124,"inlinee":35724,"source_line":28,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1803,"record_offset":79420,"parent_record_offset":79396,"ancestor_record_offset":78124,"inlinee":35659,"source_line":28,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":1803,"record_offset":187752,"parent_record_offset":187728,"ancestor_record_offset":187468,"inlinee":87728,"source_line":473,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1803,"record_offset":79048,"parent_record_offset":79024,"ancestor_record_offset":78124,"inlinee":35346,"source_line":658,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1803,"record_offset":201080,"parent_record_offset":201056,"ancestor_record_offset":200448,"inlinee":35338,"source_line":155,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1803,"record_offset":57976,"parent_record_offset":57944,"ancestor_record_offset":57060,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1803,"record_offset":57888,"parent_record_offset":57864,"ancestor_record_offset":57060,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1803,"record_offset":57944,"parent_record_offset":57888,"ancestor_record_offset":57060,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1803,"record_offset":57916,"parent_record_offset":57888,"ancestor_record_offset":57060,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1803,"record_offset":58008,"parent_record_offset":57976,"ancestor_record_offset":57060,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1803,"record_offset":57864,"parent_record_offset":57840,"ancestor_record_offset":57060,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":57128,"parent_record_offset":57096,"ancestor_record_offset":57060,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":57336,"parent_record_offset":57160,"ancestor_record_offset":57060,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":187804,"parent_record_offset":187776,"ancestor_record_offset":187468,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":57360,"parent_record_offset":57336,"ancestor_record_offset":57060,"inlinee":83339,"source_line":1515,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":80796,"parent_record_offset":80772,"ancestor_record_offset":78124,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":80144,"parent_record_offset":80120,"ancestor_record_offset":78124,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":79444,"parent_record_offset":79420,"ancestor_record_offset":78124,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":80472,"parent_record_offset":80448,"ancestor_record_offset":78124,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=15
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":57160,"parent_record_offset":57128,"ancestor_record_offset":57060,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":79396,"parent_record_offset":79372,"ancestor_record_offset":78124,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1803,"record_offset":187776,"parent_record_offset":187752,"ancestor_record_offset":187468,"inlinee":32851,"source_line":17,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3941
// 首条位置: {"module":153,"inlinee":14657}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10252,"parent_record_offset":10220,"ancestor_record_offset":10220,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10924,"parent_record_offset":10220,"ancestor_record_offset":10220,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10796,"parent_record_offset":10220,"ancestor_record_offset":10220,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":10648,"parent_record_offset":10616,"ancestor_record_offset":10220,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":10516,"parent_record_offset":10488,"ancestor_record_offset":10220,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":10576,"parent_record_offset":10544,"ancestor_record_offset":10220,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":10220,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":10220,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":10544,"parent_record_offset":10516,"ancestor_record_offset":10220,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":10448,"parent_record_offset":10420,"ancestor_record_offset":10220,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":10616,"parent_record_offset":10448,"ancestor_record_offset":10220,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":10392,"parent_record_offset":10364,"ancestor_record_offset":10220,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":10736,"parent_record_offset":10448,"ancestor_record_offset":10220,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":10488,"parent_record_offset":10448,"ancestor_record_offset":10220,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":153,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":10220,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10308,"parent_record_offset":10280,"ancestor_record_offset":10220,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":10220,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":10220,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":10952,"parent_record_offset":10924,"ancestor_record_offset":10220,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":153,"record_offset":10364,"parent_record_offset":10336,"ancestor_record_offset":10220,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":153,"record_offset":10336,"parent_record_offset":10308,"ancestor_record_offset":10220,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$19::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8218
// 首条位置: {"module":540,"inlinee":33304}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":16012,"rva":10946368,"va":5379655488}
// 编译实例 RVAs: 0xa70740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3571333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8857e3600","size":5,"soft_fail":false,"target_va":5383227792,"va":5379656454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a744d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8973c0000","size":5,"soft_fail":false,"target_va":5379671248,"va":5379655732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b81430
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17894329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8b90b1101","size":5,"soft_fail":false,"target_va":5397550128,"va":5379655794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b81e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17896263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e847131101","size":5,"soft_fail":false,"target_va":5397552672,"va":5379656404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b83160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17901956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e884291101","size":5,"soft_fail":false,"target_va":5397557600,"va":5379655639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b833a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17901954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e882291101","size":5,"soft_fail":false,"target_va":5397558176,"va":5379656217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84570
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17906241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8413a1101","size":5,"soft_fail":false,"target_va":5397562736,"va":5379656490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17910943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e89f4c1101","size":5,"soft_fail":false,"target_va":5397567120,"va":5379656172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17911941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e885501101","size":5,"soft_fail":false,"target_va":5397567776,"va":5379655830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b85bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17912555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8eb521101","size":5,"soft_fail":false,"target_va":5397568432,"va":5379655872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b86da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17917345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8a1651101","size":5,"soft_fail":false,"target_va":5397573024,"va":5379655674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b892b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17926571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e8ab891101","size":5,"soft_fail":false,"target_va":5397582512,"va":5379655936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17937741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e84db51101","size":5,"soft_fail":false,"target_va":5397593456,"va":5379655710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b8c490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17938966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379655488,"raw_hex":"e816ba1101","size":5,"soft_fail":false,"target_va":5397595280,"va":5379656309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":540,"record_offset":17164,"parent_record_offset":16012,"ancestor_record_offset":16012,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":540,"record_offset":17052,"parent_record_offset":16012,"ancestor_record_offset":16012,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":16224,"parent_record_offset":16012,"ancestor_record_offset":16012,"inlinee":14659,"source_line":145,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_lantern_brightness
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":17108,"parent_record_offset":16012,"ancestor_record_offset":16012,"inlinee":33272,"source_line":161,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::is_default_stair_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":17136,"parent_record_offset":16012,"ancestor_record_offset":16012,"inlinee":33273,"source_line":177,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":17080,"parent_record_offset":16012,"ancestor_record_offset":16012,"inlinee":28372,"source_line":220,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":16012,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":16372,"parent_record_offset":16344,"ancestor_record_offset":16012,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":16252,"parent_record_offset":16224,"ancestor_record_offset":16012,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":540,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":16012,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":540,"record_offset":16280,"parent_record_offset":16252,"ancestor_record_offset":16012,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$105::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":16832,"parent_record_offset":16804,"ancestor_record_offset":16012,"inlinee":33275,"source_line":353,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$113::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":17016,"parent_record_offset":16988,"ancestor_record_offset":16012,"inlinee":14663,"source_line":364,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":16648,"parent_record_offset":16620,"ancestor_record_offset":16012,"inlinee":27213,"source_line":323,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$20::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FD3
// 首条位置: {"module":536,"inlinee":32723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$130::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":536,"record_offset":96696,"parent_record_offset":96640,"ancestor_record_offset":96640,"inlinee":26913,"source_line":570,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$21::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8216
// 首条位置: {"module":540,"inlinee":33302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$23::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x140EA
// 首条位置: {"module":1658,"inlinee":82154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1658,"record_offset":11064,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1658,"record_offset":10952,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$105::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":11120,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":33275,"source_line":353,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$113::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":11176,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":14663,"source_line":364,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$134::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":10428,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":26967,"source_line":570,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":10780,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":26794,"source_line":21,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":10980,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":67220,"source_line":180,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$61::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":10924,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":38687,"source_line":242,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$89::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":11092,"parent_record_offset":10328,"ancestor_record_offset":10328,"inlinee":27213,"source_line":323,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1658,"record_offset":10464,"parent_record_offset":10428,"ancestor_record_offset":10328,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1658,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":10328,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1658,"record_offset":10832,"parent_record_offset":10808,"ancestor_record_offset":10328,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1658,"record_offset":10712,"parent_record_offset":10680,"ancestor_record_offset":10328,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$140::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":10328,"inlinee":26951,"source_line":636,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$158::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1658,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":10328,"inlinee":26949,"source_line":666,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1658,"record_offset":10616,"parent_record_offset":10544,"ancestor_record_offset":10328,"inlinee":26957,"source_line":44,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":1658,"record_offset":10544,"parent_record_offset":10488,"ancestor_record_offset":10328,"inlinee":26955,"source_line":6,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1658,"record_offset":10740,"parent_record_offset":10680,"ancestor_record_offset":10328,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1658,"record_offset":10580,"parent_record_offset":10544,"ancestor_record_offset":10328,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1658,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":10328,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1658,"record_offset":10516,"parent_record_offset":10488,"ancestor_record_offset":10328,"inlinee":26953,"source_line":106,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=33
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3943
// 首条位置: {"module":153,"inlinee":14659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":153,"record_offset":12116,"parent_record_offset":12088,"ancestor_record_offset":12088,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":16684,"inlinee":13199,"source_line":1878,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":153,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":12180,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":153,"record_offset":12144,"parent_record_offset":12116,"ancestor_record_offset":12088,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$105::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":16832,"parent_record_offset":16804,"ancestor_record_offset":16776,"inlinee":33275,"source_line":353,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$113::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":153,"record_offset":13472,"parent_record_offset":13436,"ancestor_record_offset":13400,"inlinee":14663,"source_line":364,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":540,"record_offset":16648,"parent_record_offset":16620,"ancestor_record_offset":16592,"inlinee":27213,"source_line":323,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default_chimney_smoke
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":607,"record_offset":126540,"rva":9411360,"va":5378120480}
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
// 函数符号: country_core::resources::history::decorator_edit::is_default_lantern_brightness
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81F8
// 首条位置: {"module":540,"inlinee":33272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::is_default_stair_width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81F9
// 首条位置: {"module":540,"inlinee":33273}
// 编译实例 RVAs: 0xc7a3b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":607,"record_offset":127488,"rva":13084432,"va":5381793552}
// 编译实例 RVAs: 0xc7a710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34828241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381793552,"raw_hex":"e8d16f1302","size":5,"soft_fail":false,"target_va":5416622240,"va":5381793994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":607,"record_offset":128068,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":607,"record_offset":127844,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":607,"record_offset":127760,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":607,"record_offset":127696,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_chimney
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127664,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":28390,"source_line":420,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_flag
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":128012,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":28383,"source_line":420,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_lantern
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127928,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":28391,"source_line":420,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127984,"parent_record_offset":127488,"ancestor_record_offset":127488,"inlinee":28387,"source_line":420,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FD1
// 首条位置: {"module":536,"inlinee":32721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":536,"record_offset":88700,"parent_record_offset":88676,"ancestor_record_offset":88676,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$3::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":40968,"rva":9419680,"va":5378128800}
// 编译实例 RVAs: 0x8fbba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":501,"record_offset":41116,"parent_record_offset":40968,"ancestor_record_offset":40968,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$128::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":501,"record_offset":41140,"parent_record_offset":41116,"ancestor_record_offset":40968,"inlinee":28137,"source_line":455,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$4::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 303
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6EFA
// 首条位置: {"module":540,"inlinee":28410}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":540,"record_offset":22380,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$24::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FE8
// 首条位置: {"module":536,"inlinee":32744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":536,"record_offset":154028,"parent_record_offset":154000,"ancestor_record_offset":154000,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":536,"record_offset":154568,"parent_record_offset":154540,"ancestor_record_offset":154000,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":536,"record_offset":154468,"parent_record_offset":154264,"ancestor_record_offset":154000,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":154432,"parent_record_offset":154408,"ancestor_record_offset":154000,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":154596,"parent_record_offset":154540,"ancestor_record_offset":154000,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":154176,"parent_record_offset":154140,"ancestor_record_offset":154000,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":536,"record_offset":154296,"parent_record_offset":154264,"ancestor_record_offset":154000,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":154384,"parent_record_offset":154264,"ancestor_record_offset":154000,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":154408,"parent_record_offset":154384,"ancestor_record_offset":154000,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":536,"record_offset":154084,"parent_record_offset":154056,"ancestor_record_offset":154000,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":536,"record_offset":154344,"parent_record_offset":154320,"ancestor_record_offset":154000,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":536,"record_offset":154204,"parent_record_offset":154176,"ancestor_record_offset":154000,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":536,"record_offset":154540,"parent_record_offset":154140,"ancestor_record_offset":154000,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":536,"record_offset":154056,"parent_record_offset":154028,"ancestor_record_offset":154000,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":536,"record_offset":154232,"parent_record_offset":154204,"ancestor_record_offset":154000,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":536,"record_offset":154264,"parent_record_offset":154232,"ancestor_record_offset":154000,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":536,"record_offset":154140,"parent_record_offset":154112,"ancestor_record_offset":154000,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":536,"record_offset":154112,"parent_record_offset":154084,"ancestor_record_offset":154000,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$26::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":8624,"rva":10763104,"va":5379472224}
// 编译实例 RVAs: 0xa43b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field1_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5348889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379472224,"raw_hex":"e8199e5100","size":5,"soft_fail":false,"target_va":5384821184,"va":5379472290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$5::from
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FE6
// 首条位置: {"module":536,"inlinee":32742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":153900,"parent_record_offset":153484,"ancestor_record_offset":153484,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":536,"record_offset":153512,"parent_record_offset":153484,"ancestor_record_offset":153484,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":536,"record_offset":153764,"parent_record_offset":153736,"ancestor_record_offset":153484,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":536,"record_offset":153816,"parent_record_offset":153792,"ancestor_record_offset":153484,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":153708,"parent_record_offset":153672,"ancestor_record_offset":153484,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":536,"record_offset":153736,"parent_record_offset":153672,"ancestor_record_offset":153484,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":536,"record_offset":153792,"parent_record_offset":153736,"ancestor_record_offset":153484,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":536,"record_offset":153536,"parent_record_offset":153512,"ancestor_record_offset":153484,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":536,"record_offset":153672,"parent_record_offset":153584,"ancestor_record_offset":153484,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":536,"record_offset":153560,"parent_record_offset":153536,"ancestor_record_offset":153484,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":536,"record_offset":153644,"parent_record_offset":153584,"ancestor_record_offset":153484,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":536,"record_offset":153616,"parent_record_offset":153584,"ancestor_record_offset":153484,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":536,"record_offset":153584,"parent_record_offset":153560,"ancestor_record_offset":153484,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$5::from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x987C
// 首条位置: {"module":614,"inlinee":39036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":614,"record_offset":180088,"parent_record_offset":179964,"ancestor_record_offset":179964,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$65::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":614,"record_offset":179988,"parent_record_offset":179964,"ancestor_record_offset":179964,"inlinee":26911,"source_line":277,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":614,"record_offset":180136,"parent_record_offset":180112,"ancestor_record_offset":179964,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":614,"record_offset":180112,"parent_record_offset":180088,"ancestor_record_offset":179964,"inlinee":32721,"source_line":247,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$120::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":614,"record_offset":180056,"parent_record_offset":179988,"ancestor_record_offset":179964,"inlinee":26915,"source_line":403,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$130::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":614,"record_offset":180032,"parent_record_offset":179988,"ancestor_record_offset":179964,"inlinee":26913,"source_line":570,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 343
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":20944,"rva":10949120,"va":5379658240}
// 编译实例 RVAs: 0xa71200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3981519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379658240,"raw_hex":"e8313fc3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379660218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fbba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1530619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379658240,"raw_hex":"e805a5e8ff","size":5,"soft_fail":false,"target_va":5378128800,"va":5379659414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36961919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379658240,"raw_hex":"e87ffe3302","size":5,"soft_fail":false,"target_va":5416622208,"va":5379660284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2132759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379658240,"raw_hex":"e8178b2000","size":5,"soft_fail":false,"target_va":5381792720,"va":5379659956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1531012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379658240,"raw_hex":"e87ca3e8ff","size":5,"soft_fail":false,"target_va":5378128096,"va":5379659103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":540,"record_offset":22332,"parent_record_offset":20944,"ancestor_record_offset":20944,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":22472,"parent_record_offset":20944,"ancestor_record_offset":20944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::decorator_edit::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":22304,"parent_record_offset":20944,"ancestor_record_offset":20944,"inlinee":32723,"source_line":70,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_pop_bucket.rs
//     evidence={"module":540,"record_offset":22276,"parent_record_offset":20944,"ancestor_record_offset":20944,"inlinee":33277,"source_line":43,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$37::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":540,"record_offset":21208,"parent_record_offset":20944,"ancestor_record_offset":20944,"inlinee":11333,"source_line":2067,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$45::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":540,"record_offset":21796,"parent_record_offset":20944,"ancestor_record_offset":20944,"inlinee":11341,"source_line":2159,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":22596,"parent_record_offset":22572,"ancestor_record_offset":20944,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":22620,"parent_record_offset":22596,"ancestor_record_offset":20944,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":22644,"parent_record_offset":22620,"ancestor_record_offset":20944,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":22572,"parent_record_offset":22544,"ancestor_record_offset":20944,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":540,"record_offset":22380,"parent_record_offset":22356,"ancestor_record_offset":20944,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":540,"record_offset":21480,"parent_record_offset":21456,"ancestor_record_offset":20944,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":540,"record_offset":21716,"parent_record_offset":21628,"ancestor_record_offset":20944,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":540,"record_offset":21656,"parent_record_offset":21628,"ancestor_record_offset":20944,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":540,"record_offset":21684,"parent_record_offset":21656,"ancestor_record_offset":20944,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":540,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":20944,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":540,"record_offset":21512,"parent_record_offset":21428,"ancestor_record_offset":20944,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":22496,"parent_record_offset":22472,"ancestor_record_offset":20944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":540,"record_offset":21748,"parent_record_offset":21584,"ancestor_record_offset":20944,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::decorator_edit::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/decorator_edit.rs
//     evidence={"module":540,"record_offset":22356,"parent_record_offset":22332,"ancestor_record_offset":20944,"inlinee":28410,"source_line":303,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":540,"record_offset":21964,"parent_record_offset":21936,"ancestor_record_offset":20944,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":540,"record_offset":21996,"parent_record_offset":21964,"ancestor_record_offset":20944,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":540,"record_offset":21936,"parent_record_offset":21896,"ancestor_record_offset":20944,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":540,"record_offset":21456,"parent_record_offset":21428,"ancestor_record_offset":20944,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":540,"record_offset":21240,"parent_record_offset":21208,"ancestor_record_offset":20944,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":540,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":20944,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":22116,"parent_record_offset":21896,"ancestor_record_offset":20944,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21428,"parent_record_offset":21344,"ancestor_record_offset":20944,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21896,"parent_record_offset":21868,"ancestor_record_offset":20944,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21584,"parent_record_offset":21552,"ancestor_record_offset":20944,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21552,"parent_record_offset":21272,"ancestor_record_offset":20944,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21316,"parent_record_offset":21272,"ancestor_record_offset":20944,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21400,"parent_record_offset":21344,"ancestor_record_offset":20944,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21372,"parent_record_offset":21344,"ancestor_record_offset":20944,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21344,"parent_record_offset":21316,"ancestor_record_offset":20944,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21628,"parent_record_offset":21584,"ancestor_record_offset":20944,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21272,"parent_record_offset":21240,"ancestor_record_offset":20944,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":20944,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":22544,"parent_record_offset":22520,"ancestor_record_offset":20944,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":540,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":20944,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
// 记录形式: compiled_procedure; instances=1
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":607,"record_offset":126692,"rva":13083600,"va":5381792720}
// 编译实例 RVAs: 0xc7a3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":607,"record_offset":126864,"parent_record_offset":126692,"ancestor_record_offset":126692,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":607,"record_offset":126984,"parent_record_offset":126948,"ancestor_record_offset":126692,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":607,"record_offset":127188,"parent_record_offset":127156,"ancestor_record_offset":126692,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$134::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":126948,"parent_record_offset":126916,"ancestor_record_offset":126692,"inlinee":26967,"source_line":570,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$140::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127008,"parent_record_offset":126984,"ancestor_record_offset":126692,"inlinee":26951,"source_line":636,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$158::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127156,"parent_record_offset":127132,"ancestor_record_offset":126692,"inlinee":26949,"source_line":666,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127316,"parent_record_offset":127288,"ancestor_record_offset":126692,"inlinee":26794,"source_line":21,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127212,"parent_record_offset":127188,"ancestor_record_offset":126692,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$61::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":607,"record_offset":127376,"parent_record_offset":127352,"ancestor_record_offset":126692,"inlinee":38687,"source_line":242,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$30::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":607,"record_offset":126888,"parent_record_offset":126864,"ancestor_record_offset":126692,"inlinee":12141,"source_line":476,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_attachment::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_attachment.rs
//     evidence={"module":607,"record_offset":127064,"parent_record_offset":127008,"ancestor_record_offset":126692,"inlinee":26955,"source_line":6,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":607,"record_offset":127244,"parent_record_offset":127156,"ancestor_record_offset":126692,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":607,"record_offset":127092,"parent_record_offset":127064,"ancestor_record_offset":126692,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":607,"record_offset":127036,"parent_record_offset":127008,"ancestor_record_offset":126692,"inlinee":26953,"source_line":106,"archive":"_pdb/C/modules/00607.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8263
// 首条位置: {"module":1803,"inlinee":33379}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":131468,"parent_record_offset":131444,"ancestor_record_offset":131444,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156B0
// 首条位置: {"module":1803,"inlinee":87728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":187932,"parent_record_offset":187752,"ancestor_record_offset":187752,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] vectorize::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"vectorize","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vectorize-0.2.0/src/lib.rs
//     evidence={"module":1803,"record_offset":187776,"parent_record_offset":187752,"ancestor_record_offset":187752,"inlinee":32851,"source_line":17,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1803,"record_offset":187860,"parent_record_offset":187776,"ancestor_record_offset":187752,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1803,"record_offset":187896,"parent_record_offset":187860,"ancestor_record_offset":187752,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":187832,"parent_record_offset":187776,"ancestor_record_offset":187752,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":187804,"parent_record_offset":187776,"ancestor_record_offset":187752,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3749
// 首条位置: {"module":144,"inlinee":14153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$28::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97F3
// 首条位置: {"module":611,"inlinee":38899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":611,"record_offset":91100,"parent_record_offset":91072,"ancestor_record_offset":91072,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":611,"record_offset":91624,"parent_record_offset":91600,"ancestor_record_offset":91072,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":611,"record_offset":91528,"parent_record_offset":91324,"ancestor_record_offset":91072,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":611,"record_offset":91492,"parent_record_offset":91468,"ancestor_record_offset":91072,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":611,"record_offset":91652,"parent_record_offset":91600,"ancestor_record_offset":91072,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":611,"record_offset":91244,"parent_record_offset":91212,"ancestor_record_offset":91072,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":611,"record_offset":91356,"parent_record_offset":91324,"ancestor_record_offset":91072,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":611,"record_offset":91444,"parent_record_offset":91324,"ancestor_record_offset":91072,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":611,"record_offset":91468,"parent_record_offset":91444,"ancestor_record_offset":91072,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":611,"record_offset":91156,"parent_record_offset":91128,"ancestor_record_offset":91072,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":611,"record_offset":91404,"parent_record_offset":91380,"ancestor_record_offset":91072,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":611,"record_offset":91268,"parent_record_offset":91244,"ancestor_record_offset":91072,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":611,"record_offset":91600,"parent_record_offset":91212,"ancestor_record_offset":91072,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":611,"record_offset":91128,"parent_record_offset":91100,"ancestor_record_offset":91072,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":611,"record_offset":91296,"parent_record_offset":91268,"ancestor_record_offset":91072,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":611,"record_offset":91324,"parent_record_offset":91296,"ancestor_record_offset":91072,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":611,"record_offset":91212,"parent_record_offset":91184,"ancestor_record_offset":91072,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":611,"record_offset":91184,"parent_record_offset":91156,"ancestor_record_offset":91072,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$30::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97DF
// 首条位置: {"module":611,"inlinee":38879}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":611,"record_offset":27500,"parent_record_offset":27476,"ancestor_record_offset":27476,"inlinee":13788,"source_line":1397,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":611,"record_offset":27524,"parent_record_offset":27500,"ancestor_record_offset":27476,"inlinee":13277,"source_line":191,"archive":"_pdb/C/modules/00611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 480
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A8C
// 首条位置: {"module":564,"inlinee":35468}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":564,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":564,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":17164,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93D1
// 首条位置: {"module":594,"inlinee":37841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::flat_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":35308,"parent_record_offset":34856,"ancestor_record_offset":34856,"inlinee":16895,"source_line":1501,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":594,"record_offset":34880,"parent_record_offset":34856,"ancestor_record_offset":34856,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":594,"record_offset":35168,"parent_record_offset":35140,"ancestor_record_offset":34856,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":594,"record_offset":35220,"parent_record_offset":35196,"ancestor_record_offset":34856,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::FlatMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":594,"record_offset":35332,"parent_record_offset":35308,"ancestor_record_offset":34856,"inlinee":16897,"source_line":22,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":594,"record_offset":35112,"parent_record_offset":35080,"ancestor_record_offset":34856,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":594,"record_offset":35140,"parent_record_offset":35080,"ancestor_record_offset":34856,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":594,"record_offset":35196,"parent_record_offset":35140,"ancestor_record_offset":34856,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":594,"record_offset":34932,"parent_record_offset":34904,"ancestor_record_offset":34856,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":594,"record_offset":35080,"parent_record_offset":34988,"ancestor_record_offset":34856,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":594,"record_offset":34960,"parent_record_offset":34932,"ancestor_record_offset":34856,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":594,"record_offset":35052,"parent_record_offset":34988,"ancestor_record_offset":34856,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":594,"record_offset":35024,"parent_record_offset":34988,"ancestor_record_offset":34856,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":594,"record_offset":34988,"parent_record_offset":34960,"ancestor_record_offset":34856,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":594,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":34856,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 489
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D2D
// 首条位置: {"module":1643,"inlinee":81197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1643,"record_offset":125800,"parent_record_offset":125776,"ancestor_record_offset":125776,"inlinee":10241,"source_line":481,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":126076,"parent_record_offset":126048,"ancestor_record_offset":125776,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":126128,"parent_record_offset":126104,"ancestor_record_offset":125776,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1643,"record_offset":125992,"parent_record_offset":125956,"ancestor_record_offset":125776,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":126048,"parent_record_offset":125956,"ancestor_record_offset":125776,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":126104,"parent_record_offset":126048,"ancestor_record_offset":125776,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":125848,"parent_record_offset":125824,"ancestor_record_offset":125776,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":125872,"parent_record_offset":125848,"ancestor_record_offset":125776,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":125956,"parent_record_offset":125896,"ancestor_record_offset":125776,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":125928,"parent_record_offset":125896,"ancestor_record_offset":125776,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":125896,"parent_record_offset":125872,"ancestor_record_offset":125776,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1643,"record_offset":125824,"parent_record_offset":125800,"ancestor_record_offset":125776,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 495
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8227
// 首条位置: {"module":540,"inlinee":33319}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1687,"record_offset":16660,"parent_record_offset":16636,"ancestor_record_offset":16636,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1687,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":16636,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1687,"record_offset":16856,"parent_record_offset":16832,"ancestor_record_offset":16636,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1687,"record_offset":16940,"parent_record_offset":16916,"ancestor_record_offset":16636,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1687,"record_offset":17348,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1687,"record_offset":17088,"parent_record_offset":17060,"ancestor_record_offset":16636,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1687,"record_offset":17060,"parent_record_offset":17032,"ancestor_record_offset":16636,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1687,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":16636,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1687,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":16636,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1687,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":16636,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1687,"record_offset":17000,"parent_record_offset":16972,"ancestor_record_offset":16636,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1687,"record_offset":17032,"parent_record_offset":17000,"ancestor_record_offset":16636,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1687,"record_offset":16972,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1687,"record_offset":16832,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1687,"record_offset":17376,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1687,"record_offset":16916,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1687,"record_offset":17500,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1687,"record_offset":17256,"parent_record_offset":17236,"ancestor_record_offset":16636,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1687,"record_offset":16688,"parent_record_offset":16660,"ancestor_record_offset":16636,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1687,"record_offset":17236,"parent_record_offset":17132,"ancestor_record_offset":16636,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1687,"record_offset":17568,"parent_record_offset":16688,"ancestor_record_offset":16636,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":16636,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":17468,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":17152,"parent_record_offset":17132,"ancestor_record_offset":16636,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":16748,"parent_record_offset":16720,"ancestor_record_offset":16636,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":16720,"parent_record_offset":16688,"ancestor_record_offset":16636,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":16636,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":17528,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1687,"record_offset":17132,"parent_record_offset":16776,"ancestor_record_offset":16636,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::decorator_edit::impl$33::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x986F
// 首条位置: {"module":614,"inlinee":39023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":614,"record_offset":65964,"parent_record_offset":65928,"ancestor_record_offset":65928,"inlinee":13788,"source_line":1397,"archive":"_pdb/C/modules/00614.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":614,"record_offset":65988,"parent_record_offset":65964,"ancestor_record_offset":65928,"inlinee":13277,"source_line":191,"archive":"_pdb/C/modules/00614.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorBeforeAfterWithStorage", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryDiff
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$211::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$211::deserialize", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$216::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$216::deserialize", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::clone_from_ref_in<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::clone_from_ref_in", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$62::default<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>", "constructor": "alloc::sync::impl$62::default", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::new<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit> >", "constructor": "alloc::sync::Arc::new", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$60::fmt<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$60::fmt", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$7::clone_into<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::slice::impl$7::clone_into", "arguments": ["enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditChange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditChange"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorHistoryState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::rustc_entry::RustcOccupiedEntry::remove<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,alloc::alloc::Global>", "constructor": "hashbrown::rustc_entry::RustcOccupiedEntry::remove", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >", "tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "tuple$<>", "country_core::resources::history::decorator_edit::impl$5::from::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >", "constructor": "hashbrown::map::impl$44::fold::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::walls::decorator::DecoratorInfo", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>", "hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::erase<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::erase", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>", "constructor": "serde_core::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$4::deserialize_map", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "serde_core::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_tuple<serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_json::value::de::impl$4::deserialize_tuple", "arguments": ["serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "ref$", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "ref$", "arguments": ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$81::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::sync::impl$81::to_arc_slice", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >", "constructor": "core::iter::adapters::copied::impl$1::try_fold", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "country_core::resources::walls::decorator::DecoratorId", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "country_core::resources::history::decorator_edit::DecoratorHistoryState", "std::hash::random::RandomState", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$3::deserialize_any<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$3::deserialize_any", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1> >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure$0::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "constructor": "tuple$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState", "country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawIntoIter", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::raw::RawIntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>", "country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >", "constructor": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "tuple$<>", "country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIter::drop_elements<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> > >", "constructor": "hashbrown::raw::RawIter::drop_elements", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>", "country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>", "std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>", "country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0", "tuple$<>", "ref_mut$<ref_mut$<core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>", "std::hash::random::RandomState", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2>,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >", "core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2>", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "vectorize::serialize<ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global> >,enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "vectorize::serialize", "arguments": ["ref$<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global> >", "enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "ref$", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "tuple$", "arguments": ["ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >", "ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "std::collections::hash::map::Iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "std::collections::hash::map::Iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "vectorize::deserialize<std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>,enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,enum2$<serde_json::value::Value> >", "constructor": "vectorize::deserialize", "arguments": ["std::collections::hash::map::HashMap<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>", "enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "ref_mut$", "arguments": ["hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["usize", "ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$16::clone_from_impl::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "hashbrown::scopeguard::ScopeGuard", "arguments": ["tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["hashbrown::scopeguard::ScopeGuard<tuple$<usize,ref_mut$<hashbrown::raw::RawTable<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >,hashbrown::raw::impl$16::clone_from_impl::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "serde_core::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_tuple<serde_core::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_json::value::de::impl$4::deserialize_tuple", "arguments": ["serde_core::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "constructor": "std::collections::hash::map::Values", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>", "country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$81::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >", "constructor": "alloc::sync::impl$81::to_arc_slice", "arguments": ["country_core::resources::walls::decorator::DecoratorId", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>", "country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 38; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::cull_allow_list
// 信息 ID: b1b998faac816fd3b4d3f57b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=594, inline_record=34856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::DecoratorHistoryDiff::is_empty
// 信息 ID: f7f5253bb07483f48ef4aafe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=564, inline_record=17164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=564, inline_record=20252; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::is_decorator_history_diff_empty; module=568, inline_record=44684; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::add_decorator_diff; module=568, inline_record=46760; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::History::finish_input_recording; module=568, inline_record=56664; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=594, inline_record=35592; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1814, inline_record=24552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize
// 信息 ID: bd3c56fa083e62f2f44845d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=540, inline_record=37032; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=540, inline_record=37176; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=540, inline_record=37332; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=540, inline_record=37492; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$211::deserialize<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,enum2$<serde_json::value::Value> >; module=582, inline_record=10644; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde_core::de::impls::impl$188::deserialize::TupleVisitor<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >; module=1803, inline_record=18048; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde_core::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >; module=1803, inline_record=131552; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<serde_core::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState> >; module=1804, inline_record=158740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 8f3e5ab012a2ba5b166c19a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=278616; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=426412; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=443420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: c5c2096a8ebb122dfeb68eab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> > @ 0x1bacec7 -> 0xa735b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> > @ 0x1ba66b3 -> 0xa73680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 496d9a5845d54f2aacf226c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=278436; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=373100; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=426244; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=443296; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=488812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: a76a56087f6ecc84adacb077
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=46732; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=47176; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=47400; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=47624; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=47848; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=48072; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=48704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>
// 信息 ID: e1dc9fc67d7cffcbeafc0e3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_enum -> ["serde_json::value::de::EnumDeserializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> > @ 0xa70664 -> 0xa73350 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor> @ 0x1bee1ff -> 0xa73350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: b76e62c54edf1b03a53e8c6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=53348; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=82084; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=113440; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=119292; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=130924; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=290872; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=317128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 860eded3a852c02375d2d6a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=131444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 04f66869e745307c2aecd8e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=57060; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=78124; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=187468; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=200448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: cc3573cc761b82736ef2ee88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::decorator_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=187752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: d19001fe1b7c0f803b916b57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,enum2$<serde_json::value::Value> > @ 0xc8f21c -> 0xa705d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize
// 信息 ID: b79d242e0ed0b10b3c34913b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=10220; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=540, inline_record=14184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize::impl$0::serialize
// 信息 ID: e97f93e4ed6615354e140851
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::decorator_edit::_::impl$0::serialize::__SerializeWith>; module=144, inline_record=206408; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::decorator_edit::_::impl$0::serialize::__SerializeWith>; module=1694, inline_record=292392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 7233591ef478b2bf23d44c0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> > @ 0x46ad16 -> 0x49ea60 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global> > @ 0x45bee5 -> 0x49efc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global> > @ 0x45bf8d -> 0x49efc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x45a5c1 -> 0x49f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x45a6a7 -> 0x49f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::sync::Arc<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global> > @ 0x18b1196 -> 0xa70070 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global> > @ 0x18b3de5 -> 0xa70be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global> > @ 0x18b3e8d -> 0xa70be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x18b0251 -> 0xa710e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<tuple$<ref$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> >,ref$<country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff> >,alloc::alloc::Global> > > @ 0x18b0337 -> 0xa710e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 41e4656aba772243ef8b98ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> > > > @ 0x151126b -> 0xa70740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::convert_before_after_states_to_changes
// 信息 ID: 91f776a61954f7f8abf3e179
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0xa718b4 -> 0xc7a3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$11::fmt
// 信息 ID: bc85e5409d7a56bcc5c5c39c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditChange>; module=540, inline_record=46328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$13::fmt
// 信息 ID: ddc29c2409256c0b2be9e76e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditPlace>; module=540, inline_record=44936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$15::fmt
// 信息 ID: 196feeddda0f2d0047e65bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRemove>; module=540, inline_record=45904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$19::fmt
// 信息 ID: fc2a78b1f96e9fb5c280e719
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryEditRotate>; module=540, inline_record=45676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$1::from
// 信息 ID: 35190bd4d7aa61a58f17720d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0xa4919a -> 0xc7a710 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> > @ 0xca3fc1 -> 0xc7a710 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::next<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::DecoratorAddrSnapshot>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure_env$0> @ 0x15eaeac -> 0xc7a710 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::next<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::DecoratorAddrSnapshot>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,country_core::resources::walls::decorator_storage::impl$17::take_snapshot::closure_env$0> @ 0x15eb0d8 -> 0xc7a710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$20::clone
// 信息 ID: 45d69d2b0575aad4dfad92de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=536, inline_record=96640; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=540, inline_record=22304; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,alloc::alloc::Global>; module=614, inline_record=64056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$21::fmt
// 信息 ID: 4cda53282dd056ee2283d6b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::history::decorator_edit::DecoratorHistoryState>; module=540, inline_record=45144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$23::eq
// 信息 ID: a2574804c133cf6204253ae1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::impl$82::extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0> >; module=1658, inline_record=10328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$24::default
// 信息 ID: d7d2028fe7c90e8ba60f06c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl; module=536, inline_record=154000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$26::fmt
// 信息 ID: 1ce4280d4802df5f88ff4c5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$28::default
// 信息 ID: df06ef06ce06329c7e5f117d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$62::default<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>; module=611, inline_record=91072; 不是独立调用边。
// [PDB 内联宿主] alloc::sync::impl$62::default<country_core::resources::history::decorator_edit::DecoratorHistoryDiff>; module=4750, inline_record=636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$2::from
// 信息 ID: 516e981656f2e759f319d370
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=536, inline_record=88676; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=614, inline_record=180112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$30::clone
// 信息 ID: 4674b7af248c8090cd30469e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::Arc::clone_from_ref_in<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>; module=611, inline_record=27476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$33::clone
// 信息 ID: ecf67aeb2aeb011e6c2549f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>; module=614, inline_record=65928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$3::from
// 信息 ID: eb4e7b77a2c172a91dd3c72f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_snapshot @ 0xa480db -> 0x8fbba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0xa49849 -> 0x8fbba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0xa71696 -> 0x8fbba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$4::from
// 信息 ID: 9ef454be83ecd9e03ec007cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=540, inline_record=22356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$5::from
// 信息 ID: dd753512cb00f1074f5be72d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl; module=536, inline_record=153484; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >; module=612, inline_record=66724; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::walls::decorator_storage::Decorators>,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,tuple$<>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState> >,hashbrown::map::impl$82::extend::closure_env$0<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,alloc::alloc::Global>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1> >,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2> >,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0> > > > >; module=1682, inline_record=25372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$5::from::closure$0
// 信息 ID: 8c6d1e59c0c168246cfa423b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$5::from", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$5::from::closure_env$0 <- std::collections::hash::map::impl$78::from_iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$44::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,tuple$<>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::resources::history::decorator_edit::impl$5::from::closure_env$0> > > > >,tuple$<> >; module=614, inline_record=179964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0
// 信息 ID: db2fca428a6af12e27dde697
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$7::cull_allow_list", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0 <- alloc::sync::impl$81::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>; module=1643, inline_record=125776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure$0
// 信息 ID: 84fa444ad6125962df8c52be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::decorator_edit::impl$7::cull_allow_list", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- alloc::sync::impl$81::to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState> >,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0>,country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0 <- core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::walls::decorator::DecoratorId,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::walls::decorator::DecoratorId,ref_mut$<country_core::resources::history::decorator_edit::impl$7::cull_allow_list::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::walls::decorator::DecoratorId,tuple$<> > > >; module=1687, inline_record=16636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$8::fmt
// 信息 ID: c2f32954aee373a16d4fbacf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$60::fmt<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>; module=611, inline_record=84468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::impl$9::clone
// 信息 ID: f8275bd5ab268c0db94f1c33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::slice::impl$7::clone_into<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>; module=1630, inline_record=764; 不是独立调用边。
// [PDB 内联宿主] alloc::slice::impl$7::clone_into<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>; module=1630, inline_record=1468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default
// 信息 ID: 201059f923fcb805ecf049e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=12088; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=12180; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=12272; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=12364; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=12456; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=12548; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=153, inline_record=13400; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=540, inline_record=16224; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default_chimney_smoke
// 信息 ID: 62e9d8a241d6b5d6b5b44d7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 143 条；内联宿主 0 条；闭包环境类型引用 0 条。
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
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default_lantern_brightness
// 信息 ID: 40ae016f7889042c18e7ba06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=540, inline_record=17108; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=540, inline_record=18788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::is_default_stair_width
// 信息 ID: 3900e8b62de50ac13bf93ea7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f00a -> 0xc7a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f01c -> 0xc7a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f1d3 -> 0xc7a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x49f1f5 -> 0xc7a3b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=540, inline_record=17136; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=540, inline_record=18816; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>
// 信息 ID: bb6346e112aebc55d3d2599e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::decorator_edit::send_state_diff_events -> ["country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> @ 0xa49f72 -> 0xa71200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
