// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/history/terrain_edit.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5f2c10c9421948a98aceb4142a0c319402fc8aca45dac0dd6af493fd42dca974
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 34, 507, 536, 594, 1643, 1682, 1686, 1803, 1804, 1833
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 20, 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156DA
// 首条位置: {"module":1803,"inlinee":87770}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":293496,"parent_record_offset":293464,"ancestor_record_offset":293464,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":293520,"parent_record_offset":293496,"ancestor_record_offset":293464,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":293544,"parent_record_offset":293520,"ancestor_record_offset":293464,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":293572,"parent_record_offset":293544,"ancestor_record_offset":293464,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 20, 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156D9
// 首条位置: {"module":1803,"inlinee":87769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$30::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":507348,"parent_record_offset":507320,"ancestor_record_offset":507320,"inlinee":87849,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":293324,"parent_record_offset":293292,"ancestor_record_offset":293292,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":294088,"parent_record_offset":294064,"ancestor_record_offset":293292,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":294112,"parent_record_offset":294088,"ancestor_record_offset":293292,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":294136,"parent_record_offset":294112,"ancestor_record_offset":293292,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":294064,"parent_record_offset":293984,"ancestor_record_offset":293292,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":294024,"parent_record_offset":293984,"ancestor_record_offset":293292,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":293984,"parent_record_offset":293948,"ancestor_record_offset":293292,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":293948,"parent_record_offset":293912,"ancestor_record_offset":293292,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":293520,"parent_record_offset":293496,"ancestor_record_offset":293292,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":293840,"parent_record_offset":293420,"ancestor_record_offset":293292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":293544,"parent_record_offset":293520,"ancestor_record_offset":293292,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":293572,"parent_record_offset":293544,"ancestor_record_offset":293292,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":293496,"parent_record_offset":293464,"ancestor_record_offset":293292,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1803,"record_offset":293464,"parent_record_offset":293420,"ancestor_record_offset":293292,"inlinee":87770,"source_line":20,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":293420,"parent_record_offset":293388,"ancestor_record_offset":293292,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$30::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":1803,"record_offset":507376,"parent_record_offset":507348,"ancestor_record_offset":507320,"inlinee":87850,"source_line":683,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":293356,"parent_record_offset":293324,"ancestor_record_offset":293292,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":293388,"parent_record_offset":293356,"ancestor_record_offset":293292,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":89396,"rva":29813440,"va":5398522560}
// 编译实例 RVAs: 0x1c6eac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t18037365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522560,"raw_hex":"e8753a1301","size":5,"soft_fail":false,"target_va":5416560512,"va":5398523142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150e230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7735643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522560,"raw_hex":"e8a5f689ff","size":5,"soft_fail":false,"target_va":5390787120,"va":5398522758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150eb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7733551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522560,"raw_hex":"e8d1fe89ff","size":5,"soft_fail":false,"target_va":5390789440,"va":5398522986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150f2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7731519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522560,"raw_hex":"e8c1068aff","size":5,"soft_fail":false,"target_va":5390791360,"va":5398522874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bae5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-787733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522560,"raw_hex":"e8ebfaf3ff","size":5,"soft_fail":false,"target_va":5397734848,"va":5398522576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1833,"record_offset":89812,"parent_record_offset":89396,"ancestor_record_offset":89396,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1833,"record_offset":89652,"parent_record_offset":89396,"ancestor_record_offset":89396,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1833,"record_offset":89624,"parent_record_offset":89396,"ancestor_record_offset":89396,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1833,"record_offset":89684,"parent_record_offset":89396,"ancestor_record_offset":89396,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1833,"record_offset":89772,"parent_record_offset":89748,"ancestor_record_offset":89396,"inlinee":87873,"source_line":28,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1833,"record_offset":89748,"parent_record_offset":89712,"ancestor_record_offset":89396,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$8::newtype_variant_seed
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1833,"record_offset":89712,"parent_record_offset":89684,"ancestor_record_offset":89396,"inlinee":33297,"source_line":563,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":37840,"rva":29799152,"va":5398508272}
// 编译实例 RVAs: 0x1c6b2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t18052021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398508272,"raw_hex":"e8b5731301","size":5,"soft_fail":false,"target_va":5416560512,"va":5398508486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19452266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398508272,"raw_hex":"e8962ed7fe","size":5,"soft_fail":false,"target_va":5379056256,"va":5398508517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141be8580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-536034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398508272,"raw_hex":"e81ed2f7ff","size":5,"soft_fail":false,"target_va":5397972352,"va":5398508381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c6eac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398508272,"raw_hex":"e837370000","size":5,"soft_fail":false,"target_va":5398522560,"va":5398508420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t18052717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398508272,"raw_hex":"e86d761301","size":5,"soft_fail":false,"target_va":5416561184,"va":5398508462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1833,"record_offset":38032,"parent_record_offset":37840,"ancestor_record_offset":37840,"inlinee":33271,"source_line":328,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":34,"record_offset":59576,"rva":1591648,"va":5370300768}
// 编译实例 RVAs: 0x184960, 0x1c6ad70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1526233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e8d9491700","size":5,"soft_fail":false,"target_va":5371827984,"va":5370301746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045ea80
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2990140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e83ca02d00","size":5,"soft_fail":false,"target_va":5373291136,"va":5370300991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404618f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3001631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e81fcd2d00","size":5,"soft_fail":false,"target_va":5373303024,"va":5370301388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3014637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e8edff2d00","size":5,"soft_fail":false,"target_va":5373315680,"va":5370301038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140466530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3020838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e826182e00","size":5,"soft_fail":false,"target_va":5373322544,"va":5370301701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046b660
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3042442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e88a6c2e00","size":5,"soft_fail":false,"target_va":5373343328,"va":5370300881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b5ef0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3886948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398506864,"raw_hex":"e89cb0c4ff","size":5,"soft_fail":false,"target_va":5394620144,"va":5398507087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3872938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398506864,"raw_hex":"e856e7c4ff","size":5,"soft_fail":false,"target_va":5394634864,"va":5398507797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3870467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398506864,"raw_hex":"e8fdf0c4ff","size":5,"soft_fail":false,"target_va":5394636672,"va":5398507134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3859265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398506864,"raw_hex":"e8bf1cc5ff","size":5,"soft_fail":false,"target_va":5394648224,"va":5398507484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=18
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46051209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e889afbe02","size":5,"soft_fail":false,"target_va":5416353040,"va":5370301826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46256117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370300768,"raw_hex":"e8f5cfc102","size":5,"soft_fail":false,"target_va":5416557744,"va":5370301622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tjmp\t46258412","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370300768,"raw_hex":"e9ecd8c102","size":5,"soft_fail":false,"target_va":5416560208,"va":5370301791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":59856,"parent_record_offset":59576,"ancestor_record_offset":59576,"inlinee":7399,"source_line":233,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":60252,"parent_record_offset":60220,"ancestor_record_offset":59576,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":60120,"parent_record_offset":60092,"ancestor_record_offset":59576,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":60180,"parent_record_offset":60148,"ancestor_record_offset":59576,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":60300,"parent_record_offset":60276,"ancestor_record_offset":59576,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":60276,"parent_record_offset":60252,"ancestor_record_offset":59576,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":60148,"parent_record_offset":60120,"ancestor_record_offset":59576,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":60052,"parent_record_offset":60024,"ancestor_record_offset":59576,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":60220,"parent_record_offset":60052,"ancestor_record_offset":59576,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":59996,"parent_record_offset":59968,"ancestor_record_offset":59576,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":60340,"parent_record_offset":60052,"ancestor_record_offset":59576,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":60092,"parent_record_offset":60052,"ancestor_record_offset":59576,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":34,"record_offset":60024,"parent_record_offset":59996,"ancestor_record_offset":59576,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":34,"record_offset":60416,"parent_record_offset":60392,"ancestor_record_offset":59576,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":34,"record_offset":60952,"parent_record_offset":59856,"ancestor_record_offset":59576,"inlinee":8025,"source_line":28,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":59912,"parent_record_offset":59856,"ancestor_record_offset":59576,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":60448,"parent_record_offset":59856,"ancestor_record_offset":59576,"inlinee":7401,"source_line":1910,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":61008,"parent_record_offset":60980,"ancestor_record_offset":59576,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":60392,"parent_record_offset":59856,"ancestor_record_offset":59576,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":60980,"parent_record_offset":60952,"ancestor_record_offset":59576,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":61524,"parent_record_offset":60952,"ancestor_record_offset":59576,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":34,"record_offset":59968,"parent_record_offset":59940,"ancestor_record_offset":59576,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":34,"record_offset":59940,"parent_record_offset":59912,"ancestor_record_offset":59576,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15741
// 首条位置: {"module":1803,"inlinee":87873}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x718B
// 首条位置: {"module":1833,"inlinee":29067}
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
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1577A
// 首条位置: {"module":1804,"inlinee":87930}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":194752,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":195964,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":194496,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":195768,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":195628,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":196300,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":194440,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":194532,"parent_record_offset":194384,"ancestor_record_offset":194384,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":196568,"parent_record_offset":196544,"ancestor_record_offset":194384,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":196592,"parent_record_offset":196568,"ancestor_record_offset":194384,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":196616,"parent_record_offset":196592,"ancestor_record_offset":194384,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":196544,"parent_record_offset":196456,"ancestor_record_offset":194384,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":196060,"parent_record_offset":196036,"ancestor_record_offset":194384,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":196036,"parent_record_offset":196012,"ancestor_record_offset":194384,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":196012,"parent_record_offset":195988,"ancestor_record_offset":194384,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":194648,"parent_record_offset":194624,"ancestor_record_offset":194384,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":196084,"parent_record_offset":196060,"ancestor_record_offset":194384,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":194624,"parent_record_offset":194588,"ancestor_record_offset":194384,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":195988,"parent_record_offset":195964,"ancestor_record_offset":194384,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":195012,"parent_record_offset":194960,"ancestor_record_offset":194384,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1804,"record_offset":194960,"parent_record_offset":194936,"ancestor_record_offset":194384,"inlinee":35709,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1804,"record_offset":250448,"parent_record_offset":250424,"ancestor_record_offset":249560,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":195700,"parent_record_offset":195676,"ancestor_record_offset":194384,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":196180,"parent_record_offset":196156,"ancestor_record_offset":194384,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":195676,"parent_record_offset":195652,"ancestor_record_offset":194384,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":251356,"parent_record_offset":251332,"ancestor_record_offset":249560,"inlinee":34837,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":195868,"parent_record_offset":195844,"ancestor_record_offset":194384,"inlinee":35640,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":195724,"parent_record_offset":195700,"ancestor_record_offset":194384,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":194464,"parent_record_offset":194440,"ancestor_record_offset":194384,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":194560,"parent_record_offset":194532,"ancestor_record_offset":194384,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":194708,"parent_record_offset":194684,"ancestor_record_offset":194384,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":194684,"parent_record_offset":194588,"ancestor_record_offset":194384,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":195552,"parent_record_offset":195528,"ancestor_record_offset":194384,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":194984,"parent_record_offset":194960,"ancestor_record_offset":194384,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":195272,"parent_record_offset":195248,"ancestor_record_offset":194384,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=15
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":194588,"parent_record_offset":194560,"ancestor_record_offset":194384,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156A8
// 首条位置: {"module":1803,"inlinee":87720}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":115920,"parent_record_offset":114812,"ancestor_record_offset":114812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":223164,"parent_record_offset":221884,"ancestor_record_offset":221884,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":115200,"parent_record_offset":114812,"ancestor_record_offset":114812,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":114844,"parent_record_offset":114812,"ancestor_record_offset":114812,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":116244,"parent_record_offset":116220,"ancestor_record_offset":114812,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":116268,"parent_record_offset":116244,"ancestor_record_offset":114812,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":116292,"parent_record_offset":116268,"ancestor_record_offset":114812,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":116220,"parent_record_offset":116132,"ancestor_record_offset":114812,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":116016,"parent_record_offset":115992,"ancestor_record_offset":114812,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":115992,"parent_record_offset":115968,"ancestor_record_offset":114812,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":115968,"parent_record_offset":115944,"ancestor_record_offset":114812,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":114928,"parent_record_offset":114900,"ancestor_record_offset":114812,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1803,"record_offset":116188,"parent_record_offset":116160,"ancestor_record_offset":114812,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":115944,"parent_record_offset":115920,"ancestor_record_offset":114812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":114984,"parent_record_offset":114928,"ancestor_record_offset":114812,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":115012,"parent_record_offset":114928,"ancestor_record_offset":114812,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":114956,"parent_record_offset":114928,"ancestor_record_offset":114812,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":115036,"parent_record_offset":115012,"ancestor_record_offset":114812,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":115148,"parent_record_offset":115096,"ancestor_record_offset":114812,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1803,"record_offset":115096,"parent_record_offset":115072,"ancestor_record_offset":114812,"inlinee":35709,"source_line":38,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1803,"record_offset":222916,"parent_record_offset":222892,"ancestor_record_offset":221884,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":114872,"parent_record_offset":114844,"ancestor_record_offset":114812,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":115072,"parent_record_offset":114900,"ancestor_record_offset":114812,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":115500,"parent_record_offset":115476,"ancestor_record_offset":114812,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":115120,"parent_record_offset":115096,"ancestor_record_offset":114812,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":115844,"parent_record_offset":115820,"ancestor_record_offset":114812,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":114900,"parent_record_offset":114872,"ancestor_record_offset":114812,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 28, 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F59
// 首条位置: {"module":34,"inlinee":8025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":60980,"parent_record_offset":60952,"ancestor_record_offset":60952,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":61524,"parent_record_offset":60952,"ancestor_record_offset":60952,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":61376,"parent_record_offset":61344,"ancestor_record_offset":60952,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":61244,"parent_record_offset":61216,"ancestor_record_offset":60952,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":61304,"parent_record_offset":61272,"ancestor_record_offset":60952,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":61424,"parent_record_offset":61400,"ancestor_record_offset":60952,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":61400,"parent_record_offset":61376,"ancestor_record_offset":60952,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":34,"record_offset":61272,"parent_record_offset":61244,"ancestor_record_offset":60952,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":61176,"parent_record_offset":61148,"ancestor_record_offset":60952,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":61344,"parent_record_offset":61176,"ancestor_record_offset":60952,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":61120,"parent_record_offset":61092,"ancestor_record_offset":60952,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":61464,"parent_record_offset":61176,"ancestor_record_offset":60952,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":34,"record_offset":61216,"parent_record_offset":61176,"ancestor_record_offset":60952,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":34,"record_offset":61148,"parent_record_offset":61120,"ancestor_record_offset":60952,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":61036,"parent_record_offset":61008,"ancestor_record_offset":60952,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":34,"record_offset":61008,"parent_record_offset":60980,"ancestor_record_offset":60952,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":34,"record_offset":61092,"parent_record_offset":61064,"ancestor_record_offset":60952,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":34,"record_offset":61064,"parent_record_offset":61036,"ancestor_record_offset":60952,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00034.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 36, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":90200,"rva":29814128,"va":5398523248}
// 编译实例 RVAs: 0x1c6ed70, 0x1c6ee40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1833,"record_offset":90400,"parent_record_offset":90200,"ancestor_record_offset":90200,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1833,"record_offset":90420,"parent_record_offset":90400,"ancestor_record_offset":90200,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":90440,"parent_record_offset":90420,"ancestor_record_offset":90200,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":90464,"parent_record_offset":90440,"ancestor_record_offset":90200,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":87220,"rva":9582576,"va":5378291696}
// 编译实例 RVAs: 0x9237f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6523309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378291696,"raw_hex":"e8ad896300","size":5,"soft_fail":false,"target_va":5384815120,"va":5378291806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::fmt::builders::DebugStruct>::field
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6517305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378291696,"raw_hex":"e839726300","size":5,"soft_fail":false,"target_va":5384809152,"va":5378291842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::fmt::builders::DebugStruct>::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6517697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378291696,"raw_hex":"e8c1736300","size":5,"soft_fail":false,"target_va":5384809552,"va":5378291850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::duration
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7185
// 首条位置: {"module":594,"inlinee":29061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":29044,"parent_record_offset":28772,"ancestor_record_offset":28772,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":29016,"parent_record_offset":28772,"ancestor_record_offset":28772,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":28904,"parent_record_offset":28772,"ancestor_record_offset":28772,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":28960,"parent_record_offset":28772,"ancestor_record_offset":28772,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":28820,"parent_record_offset":28772,"ancestor_record_offset":28772,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":594,"record_offset":28928,"parent_record_offset":28904,"ancestor_record_offset":28772,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":594,"record_offset":28984,"parent_record_offset":28960,"ancestor_record_offset":28772,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":594,"record_offset":28868,"parent_record_offset":28844,"ancestor_record_offset":28772,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":594,"record_offset":28844,"parent_record_offset":28820,"ancestor_record_offset":28772,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::duration::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FF3
// 首条位置: {"module":536,"inlinee":32755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":60644,"parent_record_offset":60564,"ancestor_record_offset":60564,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1682,"record_offset":60588,"parent_record_offset":60564,"ancestor_record_offset":60564,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":60856,"parent_record_offset":60832,"ancestor_record_offset":60564,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":60716,"parent_record_offset":60688,"ancestor_record_offset":60564,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1682,"record_offset":60612,"parent_record_offset":60588,"ancestor_record_offset":60564,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":1682,"record_offset":60668,"parent_record_offset":60644,"ancestor_record_offset":60564,"inlinee":32757,"source_line":88,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":60796,"parent_record_offset":60768,"ancestor_record_offset":60564,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":60768,"parent_record_offset":60668,"ancestor_record_offset":60564,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":60688,"parent_record_offset":60668,"ancestor_record_offset":60564,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1682,"record_offset":60832,"parent_record_offset":60796,"ancestor_record_offset":60564,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FF5
// 首条位置: {"module":536,"inlinee":32757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":60768,"parent_record_offset":60668,"ancestor_record_offset":60668,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":60688,"parent_record_offset":60668,"ancestor_record_offset":60668,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":60856,"parent_record_offset":60832,"ancestor_record_offset":60668,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":60716,"parent_record_offset":60688,"ancestor_record_offset":60668,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":60796,"parent_record_offset":60768,"ancestor_record_offset":60668,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1682,"record_offset":60832,"parent_record_offset":60796,"ancestor_record_offset":60668,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::revert
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":75496,"rva":9578416,"va":5378287536}
// 编译实例 RVAs: 0x9227b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2610332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e8642bd8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378289031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4807984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e8d0a2b6ff","size":5,"soft_fail":false,"target_va":5373479776,"va":5378287755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5037013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e8d5db4c00","size":5,"soft_fail":false,"target_va":5383325056,"va":5378288038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160ba10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13537303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e81790ce00","size":5,"soft_fail":false,"target_va":5391825424,"va":5378288116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38061889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e841c74402","size":5,"soft_fail":false,"target_va":5416350496,"va":5378288602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38062383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e82fc94402","size":5,"soft_fail":false,"target_va":5416350608,"va":5378288220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38062217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e889c84402","size":5,"soft_fail":false,"target_va":5416351056,"va":5378288834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38332809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e889e94802","size":5,"soft_fail":false,"target_va":5416621762,"va":5378288948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38333302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e876eb4802","size":5,"soft_fail":false,"target_va":5416622240,"va":5378288933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e883f6ffff","size":5,"soft_fail":false,"target_va":5378285536,"va":5378287960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5009133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378287536,"raw_hex":"e8ed6e4c00","size":5,"soft_fail":false,"target_va":5383297200,"va":5378288062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":77428,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":507,"record_offset":77548,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":75800,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":507,"record_offset":77344,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":507,"record_offset":75692,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":77192,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":80332,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":80388,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":77260,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":77164,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::stroke_audio_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":75916,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":29029,"source_line":260,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":78280,"parent_record_offset":75496,"ancestor_record_offset":75496,"inlinee":29022,"source_line":75,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":78148,"parent_record_offset":78124,"ancestor_record_offset":75496,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":78172,"parent_record_offset":78148,"ancestor_record_offset":75496,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":78196,"parent_record_offset":78172,"ancestor_record_offset":75496,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":78124,"parent_record_offset":78036,"ancestor_record_offset":75496,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":77800,"parent_record_offset":77768,"ancestor_record_offset":75496,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":78064,"parent_record_offset":78036,"ancestor_record_offset":75496,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":78036,"parent_record_offset":78012,"ancestor_record_offset":75496,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":77848,"parent_record_offset":77824,"ancestor_record_offset":75496,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":77824,"parent_record_offset":77800,"ancestor_record_offset":75496,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":78012,"parent_record_offset":77988,"ancestor_record_offset":75496,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":77768,"parent_record_offset":77716,"ancestor_record_offset":75496,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":77680,"parent_record_offset":77644,"ancestor_record_offset":75496,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":77716,"parent_record_offset":77680,"ancestor_record_offset":75496,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":77368,"parent_record_offset":77344,"ancestor_record_offset":75496,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":77616,"parent_record_offset":77576,"ancestor_record_offset":75496,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":77576,"parent_record_offset":77548,"ancestor_record_offset":75496,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":507,"record_offset":77644,"parent_record_offset":77576,"ancestor_record_offset":75496,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":75828,"parent_record_offset":75800,"ancestor_record_offset":75496,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":76676,"parent_record_offset":76652,"ancestor_record_offset":75496,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":76244,"parent_record_offset":76220,"ancestor_record_offset":75496,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":76328,"parent_record_offset":76304,"ancestor_record_offset":75496,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":76744,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":507,"record_offset":77392,"parent_record_offset":77368,"ancestor_record_offset":75496,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":78092,"parent_record_offset":78064,"ancestor_record_offset":75496,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":76476,"parent_record_offset":76448,"ancestor_record_offset":75496,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":76448,"parent_record_offset":76420,"ancestor_record_offset":75496,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":75716,"parent_record_offset":75692,"ancestor_record_offset":75496,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":76920,"parent_record_offset":75916,"ancestor_record_offset":75496,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":77044,"parent_record_offset":75916,"ancestor_record_offset":75496,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":76268,"parent_record_offset":76244,"ancestor_record_offset":75496,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":77916,"parent_record_offset":77716,"ancestor_record_offset":75496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":77308,"parent_record_offset":77284,"ancestor_record_offset":75496,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":76592,"parent_record_offset":76568,"ancestor_record_offset":75496,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":77888,"parent_record_offset":77716,"ancestor_record_offset":75496,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":76972,"parent_record_offset":76944,"ancestor_record_offset":75496,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":77464,"parent_record_offset":77428,"ancestor_record_offset":75496,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":77000,"parent_record_offset":76972,"ancestor_record_offset":75496,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":77500,"parent_record_offset":77464,"ancestor_record_offset":75496,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":507,"record_offset":77284,"parent_record_offset":77260,"ancestor_record_offset":75496,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::revert::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":77228,"parent_record_offset":77192,"ancestor_record_offset":75496,"inlinee":29032,"source_line":125,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::revert::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":80356,"parent_record_offset":80332,"ancestor_record_offset":75496,"inlinee":29036,"source_line":131,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":76944,"parent_record_offset":76920,"ancestor_record_offset":75496,"inlinee":29034,"source_line":267,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":77072,"parent_record_offset":77044,"ancestor_record_offset":75496,"inlinee":29035,"source_line":268,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::TerrainEditingData::get_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":75952,"parent_record_offset":75916,"ancestor_record_offset":75496,"inlinee":29030,"source_line":79,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":76700,"parent_record_offset":76676,"ancestor_record_offset":75496,"inlinee":29024,"source_line":38,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":76388,"parent_record_offset":76360,"ancestor_record_offset":75496,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":76420,"parent_record_offset":76388,"ancestor_record_offset":75496,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":76360,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":76220,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":76772,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":76304,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":507,"record_offset":76164,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":507,"record_offset":76652,"parent_record_offset":76628,"ancestor_record_offset":75496,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":76000,"parent_record_offset":75976,"ancestor_record_offset":75496,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":78328,"parent_record_offset":78304,"ancestor_record_offset":75496,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":76628,"parent_record_offset":76520,"ancestor_record_offset":75496,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":76032,"parent_record_offset":76000,"ancestor_record_offset":75496,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76568,"parent_record_offset":76544,"ancestor_record_offset":75496,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76864,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76544,"parent_record_offset":76520,"ancestor_record_offset":75496,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76084,"parent_record_offset":76060,"ancestor_record_offset":75496,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76060,"parent_record_offset":76000,"ancestor_record_offset":75496,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":78388,"parent_record_offset":78328,"ancestor_record_offset":75496,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76108,"parent_record_offset":76084,"ancestor_record_offset":75496,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76192,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76520,"parent_record_offset":76108,"ancestor_record_offset":75496,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":75976,"parent_record_offset":75952,"ancestor_record_offset":75496,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":78304,"parent_record_offset":78280,"ancestor_record_offset":75496,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7168
// 首条位置: {"module":507,"inlinee":29032}
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
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x716C
// 首条位置: {"module":507,"inlinee":29036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::revert::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DBC
// 首条位置: {"module":1643,"inlinee":81340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":572536,"parent_record_offset":572420,"ancestor_record_offset":572420,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":572448,"parent_record_offset":572420,"ancestor_record_offset":572420,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":572476,"parent_record_offset":572448,"ancestor_record_offset":572420,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":572592,"parent_record_offset":572560,"ancestor_record_offset":572420,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":572560,"parent_record_offset":572536,"ancestor_record_offset":572420,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":572708,"parent_record_offset":572536,"ancestor_record_offset":572420,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::reapply
// 记录形式: compiled_procedure; instances=1
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":80684,"rva":9580064,"va":5378289184}
// 编译实例 RVAs: 0x922e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2611724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e8f425d8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378290423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4809632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e8609cb6ff","size":5,"soft_fail":false,"target_va":5373479776,"va":5378289403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5035391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e87fd54c00","size":5,"soft_fail":false,"target_va":5383325056,"va":5378289660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160e770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13547297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e821b7ce00","size":5,"soft_fail":false,"target_va":5391837040,"va":5378289738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38060530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e8f2c14402","size":5,"soft_fail":false,"target_va":5416350496,"va":5378289961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38060761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e8d9c24402","size":5,"soft_fail":false,"target_va":5416350608,"va":5378289842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38060986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e8bac34402","size":5,"soft_fail":false,"target_va":5416351056,"va":5378290065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38331428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e824e44802","size":5,"soft_fail":false,"target_va":5416621762,"va":5378290329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e82df0ffff","size":5,"soft_fail":false,"target_va":5378285536,"va":5378289582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5007511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378289184,"raw_hex":"e897684c00","size":5,"soft_fail":false,"target_va":5383297200,"va":5378289684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":82640,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":507,"record_offset":82760,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":81012,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":507,"record_offset":82556,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":507,"record_offset":82320,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":82404,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":84572,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":82472,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":82376,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::stroke_audio_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":81128,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":29029,"source_line":260,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":507,"record_offset":80876,"parent_record_offset":80684,"ancestor_record_offset":80684,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":83360,"parent_record_offset":83336,"ancestor_record_offset":80684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":83384,"parent_record_offset":83360,"ancestor_record_offset":80684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":83408,"parent_record_offset":83384,"ancestor_record_offset":80684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":83336,"parent_record_offset":83248,"ancestor_record_offset":80684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":83012,"parent_record_offset":82980,"ancestor_record_offset":80684,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":83276,"parent_record_offset":83248,"ancestor_record_offset":80684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":83248,"parent_record_offset":83224,"ancestor_record_offset":80684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":83060,"parent_record_offset":83036,"ancestor_record_offset":80684,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":83036,"parent_record_offset":83012,"ancestor_record_offset":80684,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":83224,"parent_record_offset":83200,"ancestor_record_offset":80684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":82980,"parent_record_offset":82928,"ancestor_record_offset":80684,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":82892,"parent_record_offset":82856,"ancestor_record_offset":80684,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":82928,"parent_record_offset":82892,"ancestor_record_offset":80684,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":82580,"parent_record_offset":82556,"ancestor_record_offset":80684,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":82828,"parent_record_offset":82788,"ancestor_record_offset":80684,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":82788,"parent_record_offset":82760,"ancestor_record_offset":80684,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":507,"record_offset":82856,"parent_record_offset":82788,"ancestor_record_offset":80684,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":81040,"parent_record_offset":81012,"ancestor_record_offset":80684,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":81888,"parent_record_offset":81864,"ancestor_record_offset":80684,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":81456,"parent_record_offset":81432,"ancestor_record_offset":80684,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":81540,"parent_record_offset":81516,"ancestor_record_offset":80684,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":81956,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":507,"record_offset":82604,"parent_record_offset":82580,"ancestor_record_offset":80684,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":83304,"parent_record_offset":83276,"ancestor_record_offset":80684,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":81688,"parent_record_offset":81660,"ancestor_record_offset":80684,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":81660,"parent_record_offset":81632,"ancestor_record_offset":80684,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":80980,"parent_record_offset":80876,"ancestor_record_offset":80684,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":80924,"parent_record_offset":80876,"ancestor_record_offset":80684,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":80900,"parent_record_offset":80876,"ancestor_record_offset":80684,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":82132,"parent_record_offset":81128,"ancestor_record_offset":80684,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":82256,"parent_record_offset":81128,"ancestor_record_offset":80684,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":81480,"parent_record_offset":81456,"ancestor_record_offset":80684,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":83128,"parent_record_offset":82928,"ancestor_record_offset":80684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":82520,"parent_record_offset":82496,"ancestor_record_offset":80684,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":81804,"parent_record_offset":81780,"ancestor_record_offset":80684,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":83100,"parent_record_offset":82928,"ancestor_record_offset":80684,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":82184,"parent_record_offset":82156,"ancestor_record_offset":80684,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":82676,"parent_record_offset":82640,"ancestor_record_offset":80684,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":82212,"parent_record_offset":82184,"ancestor_record_offset":80684,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":82712,"parent_record_offset":82676,"ancestor_record_offset":80684,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":507,"record_offset":82496,"parent_record_offset":82472,"ancestor_record_offset":80684,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":82440,"parent_record_offset":82404,"ancestor_record_offset":80684,"inlinee":29038,"source_line":207,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":82156,"parent_record_offset":82132,"ancestor_record_offset":80684,"inlinee":29034,"source_line":267,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":82284,"parent_record_offset":82256,"ancestor_record_offset":80684,"inlinee":29035,"source_line":268,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::TerrainEditingData::get_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":81164,"parent_record_offset":81128,"ancestor_record_offset":80684,"inlinee":29030,"source_line":79,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":81912,"parent_record_offset":81888,"ancestor_record_offset":80684,"inlinee":29024,"source_line":38,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":81600,"parent_record_offset":81572,"ancestor_record_offset":80684,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":81632,"parent_record_offset":81600,"ancestor_record_offset":80684,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":81572,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":81432,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":81984,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":81516,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":507,"record_offset":81376,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":507,"record_offset":81864,"parent_record_offset":81840,"ancestor_record_offset":80684,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":81212,"parent_record_offset":81188,"ancestor_record_offset":80684,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":81840,"parent_record_offset":81732,"ancestor_record_offset":80684,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":81244,"parent_record_offset":81212,"ancestor_record_offset":80684,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81780,"parent_record_offset":81756,"ancestor_record_offset":80684,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":82076,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81756,"parent_record_offset":81732,"ancestor_record_offset":80684,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81296,"parent_record_offset":81272,"ancestor_record_offset":80684,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81272,"parent_record_offset":81212,"ancestor_record_offset":80684,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81320,"parent_record_offset":81296,"ancestor_record_offset":80684,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81404,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":81732,"parent_record_offset":81320,"ancestor_record_offset":80684,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":81188,"parent_record_offset":81164,"ancestor_record_offset":80684,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x716E
// 首条位置: {"module":507,"inlinee":29038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::reapply::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DCB
// 首条位置: {"module":1643,"inlinee":81355}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":624088,"parent_record_offset":623972,"ancestor_record_offset":623972,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":624000,"parent_record_offset":623972,"ancestor_record_offset":623972,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":624028,"parent_record_offset":624000,"ancestor_record_offset":623972,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":624144,"parent_record_offset":624112,"ancestor_record_offset":623972,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":624112,"parent_record_offset":624088,"ancestor_record_offset":623972,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":624260,"parent_record_offset":624088,"ancestor_record_offset":623972,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::TerrainEdit::stroke_audio_position
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7165
// 首条位置: {"module":507,"inlinee":29029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":76920,"parent_record_offset":75916,"ancestor_record_offset":75916,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":77044,"parent_record_offset":75916,"ancestor_record_offset":75916,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_stroke
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":75952,"parent_record_offset":75916,"ancestor_record_offset":75916,"inlinee":29030,"source_line":79,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":76676,"parent_record_offset":76652,"ancestor_record_offset":75916,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":76244,"parent_record_offset":76220,"ancestor_record_offset":75916,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":76328,"parent_record_offset":76304,"ancestor_record_offset":75916,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":76744,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":76476,"parent_record_offset":76448,"ancestor_record_offset":75916,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":76448,"parent_record_offset":76420,"ancestor_record_offset":75916,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":76268,"parent_record_offset":76244,"ancestor_record_offset":75916,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":76592,"parent_record_offset":76568,"ancestor_record_offset":75916,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":76972,"parent_record_offset":76944,"ancestor_record_offset":75916,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":77000,"parent_record_offset":76972,"ancestor_record_offset":75916,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":76944,"parent_record_offset":76920,"ancestor_record_offset":75916,"inlinee":29034,"source_line":267,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":77072,"parent_record_offset":77044,"ancestor_record_offset":75916,"inlinee":29035,"source_line":268,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":76700,"parent_record_offset":76676,"ancestor_record_offset":75916,"inlinee":29024,"source_line":38,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":76388,"parent_record_offset":76360,"ancestor_record_offset":75916,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":76420,"parent_record_offset":76388,"ancestor_record_offset":75916,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":76360,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":76220,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":76772,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":76304,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":507,"record_offset":76164,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":507,"record_offset":76652,"parent_record_offset":76628,"ancestor_record_offset":75916,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":76000,"parent_record_offset":75976,"ancestor_record_offset":75916,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":76628,"parent_record_offset":76520,"ancestor_record_offset":75916,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":76032,"parent_record_offset":76000,"ancestor_record_offset":75916,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76568,"parent_record_offset":76544,"ancestor_record_offset":75916,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76864,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76544,"parent_record_offset":76520,"ancestor_record_offset":75916,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76084,"parent_record_offset":76060,"ancestor_record_offset":75916,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76060,"parent_record_offset":76000,"ancestor_record_offset":75916,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76108,"parent_record_offset":76084,"ancestor_record_offset":75916,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76192,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":76520,"parent_record_offset":76108,"ancestor_record_offset":75916,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":75976,"parent_record_offset":75952,"ancestor_record_offset":75916,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x716A
// 首条位置: {"module":507,"inlinee":29034}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":76972,"parent_record_offset":76944,"ancestor_record_offset":76944,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":77000,"parent_record_offset":76972,"ancestor_record_offset":76944,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x716B
// 首条位置: {"module":507,"inlinee":29035}
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
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":69588,"rva":9576416,"va":5378285536}
// 编译实例 RVAs: 0x921fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2607148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e8d437d8ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378285831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-2607816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e83835d8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378286515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9563631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e811126eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378285818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9046669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e873f575ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5378287272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4805861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e81babb6ff","size":5,"soft_fail":false,"target_va":5373479776,"va":5378285632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f2c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13437463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e8170acd00","size":5,"soft_fail":false,"target_va":5391723552,"va":5378286084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17083634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e8f2ac0401","size":5,"soft_fail":false,"target_va":5395370064,"va":5378286425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17098624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e880e70401","size":5,"soft_fail":false,"target_va":5395384752,"va":5378286123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: terrain_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38771189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e8f5994f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378285862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38334609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e891f04802","size":5,"soft_fail":false,"target_va":5416620794,"va":5378286180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38335371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e88bf34802","size":5,"soft_fail":false,"target_va":5416622208,"va":5378286832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38336319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e83ff74802","size":5,"soft_fail":false,"target_va":5416622240,"va":5378285916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38334644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e8b4f04802","size":5,"soft_fail":false,"target_va":5416621632,"va":5378286983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5011206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378285536,"raw_hex":"e806774c00","size":5,"soft_fail":false,"target_va":5383297200,"va":5378285989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":70764,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":71328,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":507,"record_offset":73588,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":71272,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":71880,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_mut_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":69800,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":29022,"source_line":75,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":507,"record_offset":73328,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":507,"record_offset":73620,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":26303,"source_line":851,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_new_from_points_remove_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":507,"record_offset":72224,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":29027,"source_line":249,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":507,"record_offset":73440,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":507,"record_offset":73268,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":507,"record_offset":73172,"parent_record_offset":69588,"ancestor_record_offset":69588,"inlinee":26087,"source_line":191,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":71068,"parent_record_offset":71044,"ancestor_record_offset":69588,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":71092,"parent_record_offset":71068,"ancestor_record_offset":69588,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":72120,"parent_record_offset":72096,"ancestor_record_offset":69588,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":72144,"parent_record_offset":72120,"ancestor_record_offset":69588,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":71116,"parent_record_offset":71092,"ancestor_record_offset":69588,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":72168,"parent_record_offset":72144,"ancestor_record_offset":69588,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":71044,"parent_record_offset":71012,"ancestor_record_offset":69588,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":72096,"parent_record_offset":71980,"ancestor_record_offset":69588,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":71440,"parent_record_offset":71416,"ancestor_record_offset":69588,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":70952,"parent_record_offset":70916,"ancestor_record_offset":69588,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":72020,"parent_record_offset":71980,"ancestor_record_offset":69588,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":71980,"parent_record_offset":71944,"ancestor_record_offset":69588,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":71488,"parent_record_offset":71464,"ancestor_record_offset":69588,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":71464,"parent_record_offset":71440,"ancestor_record_offset":69588,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":71012,"parent_record_offset":70980,"ancestor_record_offset":69588,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":70980,"parent_record_offset":70952,"ancestor_record_offset":69588,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":71944,"parent_record_offset":71912,"ancestor_record_offset":69588,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":72832,"parent_record_offset":72792,"ancestor_record_offset":69588,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":70800,"parent_record_offset":70764,"ancestor_record_offset":69588,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":70836,"parent_record_offset":70800,"ancestor_record_offset":69588,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":70872,"parent_record_offset":70836,"ancestor_record_offset":69588,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":71416,"parent_record_offset":71392,"ancestor_record_offset":69588,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":71392,"parent_record_offset":71356,"ancestor_record_offset":69588,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":73664,"parent_record_offset":73620,"ancestor_record_offset":69588,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":71228,"parent_record_offset":70872,"ancestor_record_offset":69588,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":70916,"parent_record_offset":70872,"ancestor_record_offset":69588,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":71356,"parent_record_offset":71328,"ancestor_record_offset":69588,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":507,"record_offset":72856,"parent_record_offset":72832,"ancestor_record_offset":69588,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":73520,"parent_record_offset":73496,"ancestor_record_offset":69588,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":73544,"parent_record_offset":73520,"ancestor_record_offset":69588,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":70520,"parent_record_offset":70496,"ancestor_record_offset":69588,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":70088,"parent_record_offset":70064,"ancestor_record_offset":69588,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":507,"record_offset":70172,"parent_record_offset":70148,"ancestor_record_offset":69588,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":507,"record_offset":73292,"parent_record_offset":73268,"ancestor_record_offset":69588,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":507,"record_offset":73236,"parent_record_offset":73172,"ancestor_record_offset":69588,"inlinee":26088,"source_line":626,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":507,"record_offset":73208,"parent_record_offset":73172,"ancestor_record_offset":69588,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":70588,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":72936,"parent_record_offset":72880,"ancestor_record_offset":69588,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":72880,"parent_record_offset":72856,"ancestor_record_offset":69588,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":72908,"parent_record_offset":72880,"ancestor_record_offset":69588,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":72064,"parent_record_offset":72020,"ancestor_record_offset":69588,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":70320,"parent_record_offset":70292,"ancestor_record_offset":69588,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":507,"record_offset":70292,"parent_record_offset":70264,"ancestor_record_offset":69588,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":73408,"parent_record_offset":73328,"ancestor_record_offset":69588,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":73380,"parent_record_offset":73328,"ancestor_record_offset":69588,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":73352,"parent_record_offset":73328,"ancestor_record_offset":69588,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":507,"record_offset":73496,"parent_record_offset":73472,"ancestor_record_offset":69588,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":507,"record_offset":73472,"parent_record_offset":73440,"ancestor_record_offset":69588,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":507,"record_offset":71172,"parent_record_offset":70872,"ancestor_record_offset":69588,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":70112,"parent_record_offset":70088,"ancestor_record_offset":69588,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":71912,"parent_record_offset":71880,"ancestor_record_offset":69588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":70436,"parent_record_offset":70412,"ancestor_record_offset":69588,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":507,"record_offset":73720,"parent_record_offset":73692,"ancestor_record_offset":69588,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":507,"record_offset":71296,"parent_record_offset":71272,"ancestor_record_offset":69588,"inlinee":29026,"source_line":280,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":507,"record_offset":70544,"parent_record_offset":70520,"ancestor_record_offset":69588,"inlinee":29024,"source_line":38,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":70232,"parent_record_offset":70204,"ancestor_record_offset":69588,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":70264,"parent_record_offset":70232,"ancestor_record_offset":69588,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":507,"record_offset":70204,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":70064,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":70616,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":507,"record_offset":70148,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":507,"record_offset":70008,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":507,"record_offset":70496,"parent_record_offset":70472,"ancestor_record_offset":69588,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":69848,"parent_record_offset":69824,"ancestor_record_offset":69588,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":70472,"parent_record_offset":70364,"ancestor_record_offset":69588,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":507,"record_offset":69880,"parent_record_offset":69848,"ancestor_record_offset":69588,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":70412,"parent_record_offset":70388,"ancestor_record_offset":69588,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":70708,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":70388,"parent_record_offset":70364,"ancestor_record_offset":69588,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":69932,"parent_record_offset":69908,"ancestor_record_offset":69588,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":69908,"parent_record_offset":69848,"ancestor_record_offset":69588,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":69956,"parent_record_offset":69932,"ancestor_record_offset":69588,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":70036,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":507,"record_offset":70364,"parent_record_offset":69956,"ancestor_record_offset":69588,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":69824,"parent_record_offset":69800,"ancestor_record_offset":69588,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":507,"record_offset":73692,"parent_record_offset":73620,"ancestor_record_offset":69588,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7162
// 首条位置: {"module":507,"inlinee":29026}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::terrain_edit::terrain_edit_animation_progress
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7187
// 首条位置: {"module":594,"inlinee":29063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":25388,"parent_record_offset":25348,"ancestor_record_offset":25348,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":594,"record_offset":25420,"parent_record_offset":25348,"ancestor_record_offset":25348,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":25444,"parent_record_offset":25420,"ancestor_record_offset":25348,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00594.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::terrain_edit::TerrainEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "alloc::sync::ArcInner", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::new<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "alloc::sync::Arc::new", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$60::fmt<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::impl$60::fmt", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >", "constructor": "core::option::Option", "arguments": ["enum2$<country_core::resources::history::terrain_edit::TerrainEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 29; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::LanternLightCache>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::LightLanternCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::LanternLightCache>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::LightLanternCmd>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::LanternLightCache>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::LightLanternCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::duration
// 信息 ID: 58f9a0720da0e56b46685174
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::forward; module=594, inline_record=28772; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::backward; module=594, inline_record=29432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::animate_terrain_delete::animate_terrain_delete; module=1686, inline_record=10508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::reapply
// 信息 ID: fad3176da4e306bc908f8d61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_terrain::replay_terrain @ 0xc216f1 -> 0x922e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::animate_terrain_delete::animate_terrain_delete @ 0x1821187 -> 0x922e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::revert
// 信息 ID: e09ae33eb8967a072fdc51a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_terrain::replay_terrain @ 0xc216ba -> 0x9227b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::stroke_audio_position
// 信息 ID: 3205d2e37ff70e77b4a543a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=507, inline_record=75916; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::reapply; module=507, inline_record=81128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve
// 信息 ID: 14b4d13480bfea595bb371f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::revert @ 0x922958 -> 0x921fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::reapply @ 0x922fae -> 0x921fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize
// 信息 ID: 1f45ddc1aa4aad7e1157e592
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=1833, inline_record=89772; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=1833, inline_record=89944; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>; module=1833, inline_record=90108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 7f5f3a664a18b5b834edacb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=293464; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=507452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 63fdd6fd4ea8a27d5d381947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> > @ 0x1ba5f33 -> 0x1c6ed70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> > @ 0x1baac13 -> 0x1c6ee40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 08d895eafed286896f6fdda1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=293292; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=364020; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=456580; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=507320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 303449c707923b76926f2346
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1833, inline_record=99568; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1833, inline_record=99792; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1833, inline_record=100016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>
// 信息 ID: 0c3317e5778d058a55328caf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_enum -> ["serde_json::value::de::EnumDeserializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor> @ 0x1be86cf -> 0x1c6eac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> > @ 0x1c6b384 -> 0x1c6eac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 43af874152f1e78eff57bf6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=194384; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=203828; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=249560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: e7acf78ab29ecf4397d000c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=114812; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=127708; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::terrain_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=221884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 2782b02178483b2e392dd341
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,enum2$<serde_json::value::Value> > @ 0xc8f632 -> 0x1c6b2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::serialize
// 信息 ID: 53b2ea88d663fd7947b47313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=34, inline_record=60952; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=34, inline_record=62764; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=34, inline_record=64516; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1833, inline_record=33592; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1833, inline_record=35404; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1833, inline_record=37156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 7a5c3f509e97d98c45edfd85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::terrain_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x24743d -> 0x184960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x1c1709d -> 0x1c6ad70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$0::fmt
// 信息 ID: 5e7165b604b8545480ad9a34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::sync::impl$60::fmt<enum2$<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global> @ 0xc92c77 -> 0x9237f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::duration::closure$0
// 信息 ID: f88d0fd284225883915d31ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::duration", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; module=1682, inline_record=60564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::duration::closure$0::closure$0
// 信息 ID: 962af6494c112b4eb4aed93b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::duration", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,country_core::resources::history::terrain_edit::impl$1::duration::closure_env$0> > >; module=1682, inline_record=60668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::reapply::closure$0
// 信息 ID: 48a9814e1ad5bf23e5c6c7fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::reapply", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::reapply; module=507, inline_record=82440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::reapply::closure$1
// 信息 ID: 4f7f2577a201bca412825537
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::reapply", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::reapply::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::reapply::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::reapply::closure_env$1> >; module=1643, inline_record=623972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$0
// 信息 ID: 481a863458bc4cb45e27a9b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=507, inline_record=77228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$1
// 信息 ID: 95ff88e0fc604ab7b74b8193
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=507, inline_record=80356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::revert::closure$2
// 信息 ID: 0984d5b2b2e0f65ea460b1b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::revert", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::terrain_edit::impl$1::revert::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::revert::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl$1::revert::closure_env$2> >; module=1643, inline_record=572420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$0
// 信息 ID: 62e1dabbaad46f3f37309903
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::stroke_audio_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=507, inline_record=76944; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::reapply; module=507, inline_record=82156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::stroke_audio_position::closure$1
// 信息 ID: 8a6fe5dad141786fdb35c7ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::stroke_audio_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::revert; module=507, inline_record=77072; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::reapply; module=507, inline_record=82284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::impl$1::update_terrain_curve::closure$0
// 信息 ID: 7ac60fbb564cc71366179809
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::terrain_edit::impl$1::update_terrain_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve; module=507, inline_record=71296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::terrain_edit::terrain_edit_animation_progress
// 信息 ID: f9bb26b93833249c32a75758
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::replay_terrain; module=594, inline_record=25348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::animate_terrain_delete::animate_terrain_delete; module=1686, inline_record=10816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
