// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/walls/inner_wall_state.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 180e8e17a1d9095dac08b98c96de2f5eef037375b7bc81aa222c311818d0879d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: inner_wall_state.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 45, 174, 191, 199, 400, 404, 479, 480, 494, 497, 508, 519, 533, 545, 547, 553, 554, 555, 556, 558, 563, 564, 566, 1627, 1628, 1629, 1630, 1643, 1734, 3756, 4471
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use ryu; // namespace candidate
//   package identities: [{"name":"ryu","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.11"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 19, 27, 49, 84, 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6804
// 首条位置: {"module":547,"inlinee":26628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":325552,"parent_record_offset":325528,"ancestor_record_offset":325528,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":526764,"parent_record_offset":526740,"ancestor_record_offset":526740,"inlinee":6705,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":664116,"parent_record_offset":664092,"ancestor_record_offset":664092,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":269324,"parent_record_offset":269300,"ancestor_record_offset":269300,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":269400,"parent_record_offset":269348,"ancestor_record_offset":269300,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":269348,"parent_record_offset":269324,"ancestor_record_offset":269300,"inlinee":26613,"source_line":143,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":325576,"parent_record_offset":325552,"ancestor_record_offset":325528,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":269372,"parent_record_offset":269348,"ancestor_record_offset":269300,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 19, 27, 49, 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67F1
// 首条位置: {"module":480,"inlinee":26609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":480,"record_offset":36556,"parent_record_offset":36500,"ancestor_record_offset":36500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":480,"record_offset":36588,"parent_record_offset":36500,"ancestor_record_offset":36500,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":36528,"parent_record_offset":36500,"ancestor_record_offset":36500,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=4
// 源码行: 19, 49, 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":480,"record_offset":35660,"rva":8781568,"va":5377490688}
// 编译实例 RVAs: 0x85ff00, 0x8606b0, 0x861780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t33986744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8b8980602","size":5,"soft_fail":false,"target_va":5411478016,"va":5377491267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3309389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e84d7f3200","size":5,"soft_fail":false,"target_va":5380800512,"va":5377491118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=32
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2410091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e89539dbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377491590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=12
//     target_aliases=7 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8fdd5ffff","size":5,"soft_fail":false,"target_va":5377481360,"va":5377492110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t163275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8cb7d0200","size":5,"soft_fail":false,"target_va":5377654256,"va":5377490976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t167038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e87e8c0200","size":5,"soft_fail":false,"target_va":5377658224,"va":5377491181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889390
//     owner=unresolved_namespace; count=8
//     target_aliases=4 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t168116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8b4900200","size":5,"soft_fail":false,"target_va":5377659792,"va":5377491671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t232611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8a38c0300","size":5,"soft_fail":false,"target_va":5377724224,"va":5377491608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c450
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12436659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8b3c4bd00","size":5,"soft_fail":false,"target_va":5389927504,"va":5377490840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d1e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14095978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377492656,"raw_hex":"e86a16d700","size":5,"soft_fail":false,"target_va":5391589008,"va":5377493025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d1fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14098266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e85a1fd700","size":5,"soft_fail":false,"target_va":5391589328,"va":5377491057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d2cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14095354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377496960,"raw_hex":"e8fa13d700","size":5,"soft_fail":false,"target_va":5391592688,"va":5377497329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34183986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8329b0902","size":5,"soft_fail":false,"target_va":5411674944,"va":5377490953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33987295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377490688,"raw_hex":"e8df9a0602","size":5,"soft_fail":false,"target_va":5411478544,"va":5377491244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":480,"record_offset":35852,"parent_record_offset":35660,"ancestor_record_offset":35660,"inlinee":26604,"source_line":255,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":36864,"parent_record_offset":36840,"ancestor_record_offset":35660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":36840,"parent_record_offset":36752,"ancestor_record_offset":35660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":480,"record_offset":36024,"parent_record_offset":36000,"ancestor_record_offset":35660,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":480,"record_offset":36080,"parent_record_offset":36056,"ancestor_record_offset":35660,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":480,"record_offset":36132,"parent_record_offset":36104,"ancestor_record_offset":35660,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":480,"record_offset":36156,"parent_record_offset":36132,"ancestor_record_offset":35660,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":37232,"parent_record_offset":37208,"ancestor_record_offset":35660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":36780,"parent_record_offset":36752,"ancestor_record_offset":35660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":36752,"parent_record_offset":36728,"ancestor_record_offset":35660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":37280,"parent_record_offset":37256,"ancestor_record_offset":35660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":37256,"parent_record_offset":37232,"ancestor_record_offset":35660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":36728,"parent_record_offset":36704,"ancestor_record_offset":35660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":480,"record_offset":37208,"parent_record_offset":37184,"ancestor_record_offset":35660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":480,"record_offset":36676,"parent_record_offset":36648,"ancestor_record_offset":35660,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":480,"record_offset":36808,"parent_record_offset":36780,"ancestor_record_offset":35660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":480,"record_offset":36104,"parent_record_offset":36080,"ancestor_record_offset":35660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":480,"record_offset":36272,"parent_record_offset":36184,"ancestor_record_offset":35660,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=124
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":480,"record_offset":36620,"parent_record_offset":35852,"ancestor_record_offset":35660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":480,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":35660,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":480,"record_offset":36556,"parent_record_offset":36500,"ancestor_record_offset":35660,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":480,"record_offset":36588,"parent_record_offset":36500,"ancestor_record_offset":35660,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":480,"record_offset":36184,"parent_record_offset":36156,"ancestor_record_offset":35660,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":480,"record_offset":36240,"parent_record_offset":36216,"ancestor_record_offset":35660,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":480,"record_offset":36216,"parent_record_offset":36184,"ancestor_record_offset":35660,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":36500,"parent_record_offset":35852,"ancestor_record_offset":35660,"inlinee":26609,"source_line":147,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":36528,"parent_record_offset":36500,"ancestor_record_offset":35660,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":480,"record_offset":36000,"parent_record_offset":35852,"ancestor_record_offset":35660,"inlinee":26606,"source_line":877,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":480,"record_offset":36056,"parent_record_offset":35852,"ancestor_record_offset":35660,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 19, 27, 49, 143, 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x22CE
// 首条位置: {"module":45,"inlinee":8910}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1627,"record_offset":109904,"parent_record_offset":109880,"ancestor_record_offset":109880,"inlinee":5436,"source_line":260,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":191,"record_offset":86232,"parent_record_offset":86212,"ancestor_record_offset":86212,"inlinee":5432,"source_line":249,"archive":"_pdb/B/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1628,"record_offset":128864,"parent_record_offset":128840,"ancestor_record_offset":128840,"inlinee":8420,"source_line":180,"archive":"_pdb/B/modules/01628.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":174,"record_offset":50264,"parent_record_offset":50236,"ancestor_record_offset":50236,"inlinee":8911,"source_line":170,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":174,"record_offset":50636,"parent_record_offset":50612,"ancestor_record_offset":50236,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":174,"record_offset":50660,"parent_record_offset":50636,"ancestor_record_offset":50236,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":174,"record_offset":50612,"parent_record_offset":50584,"ancestor_record_offset":50236,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110424,"parent_record_offset":110392,"ancestor_record_offset":109880,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110240,"parent_record_offset":110212,"ancestor_record_offset":109880,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":174,"record_offset":50524,"parent_record_offset":50496,"ancestor_record_offset":50236,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110336,"parent_record_offset":110308,"ancestor_record_offset":109880,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110308,"parent_record_offset":110272,"ancestor_record_offset":109880,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110472,"parent_record_offset":110448,"ancestor_record_offset":109880,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110448,"parent_record_offset":110424,"ancestor_record_offset":109880,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1627,"record_offset":110272,"parent_record_offset":110240,"ancestor_record_offset":109880,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":174,"record_offset":50584,"parent_record_offset":50552,"ancestor_record_offset":50236,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":174,"record_offset":50552,"parent_record_offset":50524,"ancestor_record_offset":50236,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":174,"record_offset":50460,"parent_record_offset":50432,"ancestor_record_offset":50236,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":174,"record_offset":50432,"parent_record_offset":50404,"ancestor_record_offset":50236,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":174,"record_offset":50376,"parent_record_offset":50348,"ancestor_record_offset":50236,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":174,"record_offset":50404,"parent_record_offset":50376,"ancestor_record_offset":50236,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":174,"record_offset":50348,"parent_record_offset":50320,"ancestor_record_offset":50236,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":174,"record_offset":50320,"parent_record_offset":50292,"ancestor_record_offset":50236,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1627,"record_offset":110172,"parent_record_offset":110144,"ancestor_record_offset":109880,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1627,"record_offset":110392,"parent_record_offset":110172,"ancestor_record_offset":109880,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1627,"record_offset":110116,"parent_record_offset":110088,"ancestor_record_offset":109880,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1627,"record_offset":110512,"parent_record_offset":110172,"ancestor_record_offset":109880,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1627,"record_offset":110212,"parent_record_offset":110172,"ancestor_record_offset":109880,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":174,"record_offset":50496,"parent_record_offset":50460,"ancestor_record_offset":50236,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1627,"record_offset":110144,"parent_record_offset":110116,"ancestor_record_offset":109880,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1627,"record_offset":110004,"parent_record_offset":109960,"ancestor_record_offset":109880,"inlinee":9364,"source_line":646,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1628,"record_offset":128984,"parent_record_offset":128960,"ancestor_record_offset":128840,"inlinee":6588,"source_line":571,"archive":"_pdb/B/modules/01628.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":174,"record_offset":50736,"parent_record_offset":50712,"ancestor_record_offset":50236,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1628,"record_offset":129016,"parent_record_offset":128936,"ancestor_record_offset":128840,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01628.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":174,"record_offset":50712,"parent_record_offset":50460,"ancestor_record_offset":50236,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1627,"record_offset":110988,"parent_record_offset":110764,"ancestor_record_offset":109880,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] ryu::buffer::Buffer::format_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ryu-1.0.11/src/buffer/mod.rs
//     evidence={"module":1627,"record_offset":110596,"parent_record_offset":110564,"ancestor_record_offset":109880,"inlinee":9368,"source_line":72,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] ryu::buffer::impl$6::write_to_ryu_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ryu-1.0.11/src/buffer/mod.rs
//     evidence={"module":1627,"record_offset":110620,"parent_record_offset":110596,"ancestor_record_offset":109880,"inlinee":9370,"source_line":138,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$50::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":1627,"record_offset":109932,"parent_record_offset":109904,"ancestor_record_offset":109880,"inlinee":5589,"source_line":11,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::from_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1628,"record_offset":128960,"parent_record_offset":128936,"ancestor_record_offset":128840,"inlinee":6587,"source_line":264,"archive":"_pdb/B/modules/01628.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1627,"record_offset":110564,"parent_record_offset":109960,"ancestor_record_offset":109880,"inlinee":9366,"source_line":1710,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1627,"record_offset":110032,"parent_record_offset":109960,"ancestor_record_offset":109880,"inlinee":9365,"source_line":1599,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1627,"record_offset":109960,"parent_record_offset":109932,"ancestor_record_offset":109880,"inlinee":9363,"source_line":174,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":191,"record_offset":86256,"parent_record_offset":86232,"ancestor_record_offset":86212,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1628,"record_offset":128912,"parent_record_offset":128888,"ancestor_record_offset":128840,"inlinee":8893,"source_line":133,"archive":"_pdb/B/modules/01628.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1628,"record_offset":128936,"parent_record_offset":128912,"ancestor_record_offset":128840,"inlinee":8894,"source_line":138,"archive":"_pdb/B/modules/01628.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":174,"record_offset":50292,"parent_record_offset":50264,"ancestor_record_offset":50236,"inlinee":8912,"source_line":150,"archive":"_pdb/B/modules/00174.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":1627,"record_offset":110088,"parent_record_offset":110060,"ancestor_record_offset":109880,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":1627,"record_offset":110060,"parent_record_offset":110032,"ancestor_record_offset":109880,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/01627.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$7::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":3648,"rva":7761472,"va":5376470592}
// 编译实例 RVAs: 0x766e40, 0x771a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6421798","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376470592,"raw_hex":"e926fd6100","size":5,"soft_fail":false,"target_va":5382892432,"va":5376470629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$9::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x393A
// 首条位置: {"module":199,"inlinee":14650}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27, 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6805
// 首条位置: {"module":1630,"inlinee":26629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":477736,"parent_record_offset":477704,"ancestor_record_offset":477704,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":477764,"parent_record_offset":477736,"ancestor_record_offset":477704,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":477792,"parent_record_offset":477764,"ancestor_record_offset":477704,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":477820,"parent_record_offset":477792,"ancestor_record_offset":477704,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 27, 49, 84, 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1223C
// 首条位置: {"module":1630,"inlinee":74300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":476604,"parent_record_offset":476580,"ancestor_record_offset":476580,"inlinee":74331,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":264608,"parent_record_offset":264580,"ancestor_record_offset":264580,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":264916,"parent_record_offset":264892,"ancestor_record_offset":264580,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":264892,"parent_record_offset":264836,"ancestor_record_offset":264580,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":264864,"parent_record_offset":264836,"ancestor_record_offset":264580,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":264836,"parent_record_offset":264812,"ancestor_record_offset":264580,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":264812,"parent_record_offset":264788,"ancestor_record_offset":264580,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":477764,"parent_record_offset":477736,"ancestor_record_offset":477576,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":264740,"parent_record_offset":264704,"ancestor_record_offset":264580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":477792,"parent_record_offset":477764,"ancestor_record_offset":477576,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":477820,"parent_record_offset":477792,"ancestor_record_offset":477576,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":477736,"parent_record_offset":477704,"ancestor_record_offset":477576,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":477704,"parent_record_offset":477660,"ancestor_record_offset":477576,"inlinee":26629,"source_line":147,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":264704,"parent_record_offset":264672,"ancestor_record_offset":264580,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1630,"record_offset":476632,"parent_record_offset":476604,"ancestor_record_offset":476580,"inlinee":74332,"source_line":615,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":264640,"parent_record_offset":264608,"ancestor_record_offset":264580,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":264672,"parent_record_offset":264640,"ancestor_record_offset":264580,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$14::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":49808,"rva":11041424,"va":5379750544}
// 编译实例 RVAs: 0xa87a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3141846","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379750544,"raw_hex":"e9d6f02f00","size":5,"soft_fail":false,"target_va":5382892432,"va":5379750581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$16::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3940
// 首条位置: {"module":199,"inlinee":14656}
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
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":494,"record_offset":152036,"rva":9197152,"va":5377906272}
// 编译实例 RVAs: 0x8c5660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 49, 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":480,"record_offset":49668,"rva":8785232,"va":5377494352}
// 编译实例 RVAs: 0x860d50, 0x861520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34181123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494352,"raw_hex":"e803900902","size":5,"soft_fail":false,"target_va":5411675680,"va":5377494552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34350540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377496352,"raw_hex":"e8cc250c02","size":5,"soft_fail":false,"target_va":5411847152,"va":5377496607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":480,"record_offset":49872,"parent_record_offset":49668,"ancestor_record_offset":49668,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":49892,"parent_record_offset":49872,"ancestor_record_offset":49668,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":480,"record_offset":49912,"parent_record_offset":49892,"ancestor_record_offset":49668,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":480,"record_offset":49932,"parent_record_offset":49912,"ancestor_record_offset":49668,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=3
// 源码行: 49, 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":6176,"rva":5193888,"va":5373903008}
// 编译实例 RVAs: 0x4f40a0, 0x8605a0, 0x860e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1178432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903008,"raw_hex":"e840fb1100","size":5,"soft_fail":false,"target_va":5375081488,"va":5373903051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2413334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e8ea2cdbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377494833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085db60
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e850c9ffff","size":5,"soft_fail":false,"target_va":5377481568,"va":5377495563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t229415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e827800300","size":5,"soft_fail":false,"target_va":5377724224,"va":5377494804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b864c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3298296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e8f8533200","size":5,"soft_fail":false,"target_va":5380793536,"va":5377495235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141593a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13838926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e84e2ad300","size":5,"soft_fail":false,"target_va":5391333920,"va":5377494989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141593f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13840428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e82c30d300","size":5,"soft_fail":false,"target_va":5391335216,"va":5377494783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415943f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13841394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e8f233d300","size":5,"soft_fail":false,"target_va":5391336432,"va":5377495033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13844977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e8f141d300","size":5,"soft_fail":false,"target_va":5391340336,"va":5377495354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141595a20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13847359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e83f4bd300","size":5,"soft_fail":false,"target_va":5391342112,"va":5377494748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141596240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13849474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e88253d300","size":5,"soft_fail":false,"target_va":5391344192,"va":5377494713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141596ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13852746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e84a60d300","size":5,"soft_fail":false,"target_va":5391347696,"va":5377494945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141597e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13856309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e8356ed300","size":5,"soft_fail":false,"target_va":5391351392,"va":5377495078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141599080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13860841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377494576,"raw_hex":"e8e97fd300","size":5,"soft_fail":false,"target_va":5391356032,"va":5377495186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t37637740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903008,"raw_hex":"e86c4e3e02","size":5,"soft_fail":false,"target_va":5411540963,"va":5373903218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":480,"record_offset":51332,"parent_record_offset":50452,"ancestor_record_offset":50452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":480,"record_offset":51304,"parent_record_offset":50452,"ancestor_record_offset":50452,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":480,"record_offset":51880,"parent_record_offset":50452,"ancestor_record_offset":50452,"inlinee":26024,"source_line":220,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":50660,"parent_record_offset":50452,"ancestor_record_offset":50452,"inlinee":14648,"source_line":139,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default_arch_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":51852,"parent_record_offset":50452,"ancestor_record_offset":50452,"inlinee":26610,"source_line":169,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":199,"record_offset":6368,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":8911,"source_line":170,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":6176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":6176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":51564,"parent_record_offset":51540,"ancestor_record_offset":50452,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":6176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":51540,"parent_record_offset":51484,"ancestor_record_offset":50452,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":6624,"parent_record_offset":6596,"ancestor_record_offset":6176,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":51512,"parent_record_offset":51484,"ancestor_record_offset":50452,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":51484,"parent_record_offset":51460,"ancestor_record_offset":50452,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":6176,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":6176,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":51460,"parent_record_offset":51436,"ancestor_record_offset":50452,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":6176,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":6536,"parent_record_offset":6508,"ancestor_record_offset":6176,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":6480,"parent_record_offset":6452,"ancestor_record_offset":6176,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":6508,"parent_record_offset":6480,"ancestor_record_offset":6176,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":6452,"parent_record_offset":6424,"ancestor_record_offset":6176,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":6176,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":6596,"parent_record_offset":6564,"ancestor_record_offset":6176,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":51268,"parent_record_offset":51240,"ancestor_record_offset":50452,"inlinee":8501,"source_line":1740,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":50716,"parent_record_offset":50688,"ancestor_record_offset":50452,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":50688,"parent_record_offset":50660,"ancestor_record_offset":50452,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":199,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":6176,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":6812,"parent_record_offset":6564,"ancestor_record_offset":6176,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":480,"record_offset":51364,"parent_record_offset":51332,"ancestor_record_offset":50452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":51176,"parent_record_offset":51148,"ancestor_record_offset":50452,"inlinee":14656,"source_line":27,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":50992,"parent_record_offset":50964,"ancestor_record_offset":50452,"inlinee":14652,"source_line":49,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":50808,"parent_record_offset":50780,"ancestor_record_offset":50452,"inlinee":14658,"source_line":143,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$38::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":51084,"parent_record_offset":51056,"ancestor_record_offset":50452,"inlinee":14654,"source_line":147,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":50900,"parent_record_offset":50872,"ancestor_record_offset":50452,"inlinee":14650,"source_line":19,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":199,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":6176,"inlinee":8912,"source_line":150,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$21::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":49948,"rva":11041472,"va":5379750592}
// 编译实例 RVAs: 0xa87ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3141801","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379750592,"raw_hex":"e9a9f02f00","size":5,"soft_fail":false,"target_va":5382892432,"va":5379750626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$23::eq
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x393C
// 首条位置: {"module":199,"inlinee":14652}
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
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B3B
// 首条位置: {"module":508,"inlinee":27451}
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 69, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B3C
// 首条位置: {"module":494,"record_offset":152188,"rva":9197168,"va":5377906288}
// 编译实例 RVAs: 0x8c5670
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
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67DF
// 首条位置: {"module":480,"inlinee":26591}
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
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1223A
// 首条位置: {"module":1630,"inlinee":74298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":265492,"parent_record_offset":262996,"ancestor_record_offset":262996,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":269972,"parent_record_offset":262996,"ancestor_record_offset":262996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":266140,"parent_record_offset":262996,"ancestor_record_offset":262996,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":269796,"parent_record_offset":262996,"ancestor_record_offset":262996,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":263060,"parent_record_offset":262996,"ancestor_record_offset":262996,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":265520,"parent_record_offset":262996,"ancestor_record_offset":262996,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":263880,"parent_record_offset":263856,"ancestor_record_offset":262996,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":263856,"parent_record_offset":263800,"ancestor_record_offset":262996,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":263224,"parent_record_offset":263200,"ancestor_record_offset":262996,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":263276,"parent_record_offset":263248,"ancestor_record_offset":262996,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":263300,"parent_record_offset":263276,"ancestor_record_offset":262996,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":264240,"parent_record_offset":264216,"ancestor_record_offset":262996,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":263828,"parent_record_offset":263800,"ancestor_record_offset":262996,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":263800,"parent_record_offset":263776,"ancestor_record_offset":262996,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":264288,"parent_record_offset":264264,"ancestor_record_offset":262996,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":264264,"parent_record_offset":264240,"ancestor_record_offset":262996,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":263776,"parent_record_offset":263752,"ancestor_record_offset":262996,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":264216,"parent_record_offset":264192,"ancestor_record_offset":262996,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":264192,"parent_record_offset":263932,"ancestor_record_offset":262996,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":265644,"parent_record_offset":265620,"ancestor_record_offset":262996,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":264148,"parent_record_offset":264120,"ancestor_record_offset":262996,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":263248,"parent_record_offset":263224,"ancestor_record_offset":262996,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":265620,"parent_record_offset":265584,"ancestor_record_offset":262996,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":263416,"parent_record_offset":263328,"ancestor_record_offset":262996,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":263644,"parent_record_offset":263140,"ancestor_record_offset":262996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":263440,"parent_record_offset":263416,"ancestor_record_offset":262996,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":265700,"parent_record_offset":265644,"ancestor_record_offset":262996,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":265220,"parent_record_offset":263140,"ancestor_record_offset":262996,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":265784,"parent_record_offset":265760,"ancestor_record_offset":262996,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":263328,"parent_record_offset":263300,"ancestor_record_offset":262996,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":263384,"parent_record_offset":263360,"ancestor_record_offset":262996,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":263360,"parent_record_offset":263328,"ancestor_record_offset":262996,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1630,"record_offset":269692,"parent_record_offset":269668,"ancestor_record_offset":262996,"inlinee":31381,"source_line":591,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":269300,"parent_record_offset":269276,"ancestor_record_offset":262996,"inlinee":26628,"source_line":84,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":269348,"parent_record_offset":269324,"ancestor_record_offset":262996,"inlinee":26613,"source_line":143,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":264580,"parent_record_offset":264548,"ancestor_record_offset":262996,"inlinee":74300,"source_line":147,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":265760,"parent_record_offset":265736,"ancestor_record_offset":262996,"inlinee":26631,"source_line":84,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":268960,"parent_record_offset":268936,"ancestor_record_offset":262996,"inlinee":26633,"source_line":84,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":269932,"parent_record_offset":269908,"ancestor_record_offset":262996,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":269908,"parent_record_offset":269884,"ancestor_record_offset":262996,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":263100,"parent_record_offset":263060,"ancestor_record_offset":262996,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":265552,"parent_record_offset":265520,"ancestor_record_offset":262996,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":264704,"parent_record_offset":264672,"ancestor_record_offset":262996,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":266744,"parent_record_offset":266720,"ancestor_record_offset":262996,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":264548,"parent_record_offset":263140,"ancestor_record_offset":262996,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":266096,"parent_record_offset":266072,"ancestor_record_offset":262996,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":269716,"parent_record_offset":269692,"ancestor_record_offset":262996,"inlinee":6547,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":266436,"parent_record_offset":266412,"ancestor_record_offset":262996,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":263200,"parent_record_offset":263140,"ancestor_record_offset":262996,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":264640,"parent_record_offset":264608,"ancestor_record_offset":262996,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":264608,"parent_record_offset":264580,"ancestor_record_offset":262996,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":264672,"parent_record_offset":264640,"ancestor_record_offset":262996,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":269324,"parent_record_offset":269300,"ancestor_record_offset":262996,"inlinee":26611,"source_line":298,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":263140,"parent_record_offset":263100,"ancestor_record_offset":262996,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":265584,"parent_record_offset":265552,"ancestor_record_offset":262996,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6807
// 首条位置: {"module":1630,"inlinee":26631}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":265784,"parent_record_offset":265760,"ancestor_record_offset":265760,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6809
// 首条位置: {"module":1630,"inlinee":26633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":268984,"parent_record_offset":268960,"ancestor_record_offset":268960,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1221E
// 首条位置: {"module":1630,"inlinee":74270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":167552,"parent_record_offset":166840,"ancestor_record_offset":166840,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":168828,"parent_record_offset":166840,"ancestor_record_offset":166840,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":166888,"parent_record_offset":166840,"ancestor_record_offset":166840,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":167760,"parent_record_offset":167736,"ancestor_record_offset":166840,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":167736,"parent_record_offset":167648,"ancestor_record_offset":166840,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":167676,"parent_record_offset":167648,"ancestor_record_offset":166840,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":167648,"parent_record_offset":167624,"ancestor_record_offset":166840,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":167624,"parent_record_offset":167600,"ancestor_record_offset":166840,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1630,"record_offset":166976,"parent_record_offset":166944,"ancestor_record_offset":166840,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":167704,"parent_record_offset":167676,"ancestor_record_offset":166840,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":167576,"parent_record_offset":167552,"ancestor_record_offset":166840,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":167032,"parent_record_offset":166976,"ancestor_record_offset":166840,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":167060,"parent_record_offset":166976,"ancestor_record_offset":166840,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":167004,"parent_record_offset":166976,"ancestor_record_offset":166840,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":167084,"parent_record_offset":167060,"ancestor_record_offset":166840,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":167512,"parent_record_offset":167276,"ancestor_record_offset":166840,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":167168,"parent_record_offset":167144,"ancestor_record_offset":166840,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1630,"record_offset":167144,"parent_record_offset":167120,"ancestor_record_offset":166840,"inlinee":74272,"source_line":84,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":166916,"parent_record_offset":166888,"ancestor_record_offset":166840,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":167120,"parent_record_offset":166944,"ancestor_record_offset":166840,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":167480,"parent_record_offset":167456,"ancestor_record_offset":166840,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":166944,"parent_record_offset":166916,"ancestor_record_offset":166840,"inlinee":29591,"source_line":575,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12220
// 首条位置: {"module":1630,"inlinee":74272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":167168,"parent_record_offset":167144,"ancestor_record_offset":167144,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12259
// 首条位置: {"module":1630,"inlinee":74329}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":462388,"parent_record_offset":462364,"ancestor_record_offset":462364,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::serialize::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3957
// 首条位置: {"module":1628,"inlinee":14679}
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
// 记录形式: compiled_procedure; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":4204,"rva":5193120,"va":5373902240}
// 编译实例 RVAs: 0x4f3da0, 0x861220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2353072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e85018dcff","size":5,"soft_fail":false,"target_va":5371549808,"va":5373902875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b7cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-246138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8863efcff","size":5,"soft_fail":false,"target_va":5373656272,"va":5373902405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b8720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-243812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e89c47fcff","size":5,"soft_fail":false,"target_va":5373658912,"va":5373902719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b9c20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-238311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8195dfcff","size":5,"soft_fail":false,"target_va":5373664288,"va":5373902594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bb400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-232236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8d474fcff","size":5,"soft_fail":false,"target_va":5373670400,"va":5373902631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-228447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8a183fcff","size":5,"soft_fail":false,"target_va":5373674384,"va":5373902826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bcb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-226154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8968cfcff","size":5,"soft_fail":false,"target_va":5373676320,"va":5373902469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bd250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-224795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8e591fcff","size":5,"soft_fail":false,"target_va":5373678160,"va":5373902950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-221674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8169efcff","size":5,"soft_fail":false,"target_va":5373680768,"va":5373902437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bf200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-216405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8abb2fcff","size":5,"soft_fail":false,"target_va":5373686272,"va":5373902672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833230
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3404869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e845f43300","size":5,"soft_fail":false,"target_va":5377307184,"va":5373902310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11088059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e8bb30a900","size":5,"soft_fail":false,"target_va":5388584224,"va":5377496160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13770664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e8a81fd200","size":5,"soft_fail":false,"target_va":5391266448,"va":5377495779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141583920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13772197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e8a525d200","size":5,"soft_fail":false,"target_va":5391268128,"va":5377495926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141584b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13776954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e83a38d200","size":5,"soft_fail":false,"target_va":5391272848,"va":5377495889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141585160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13778059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e88b3cd200","size":5,"soft_fail":false,"target_va":5391274336,"va":5377496272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141585300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13778780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e85c3fd200","size":5,"soft_fail":false,"target_va":5391274752,"va":5377495967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13782472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e8c84dd200","size":5,"soft_fail":false,"target_va":5391278224,"va":5377495747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415879f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13789000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e84867d200","size":5,"soft_fail":false,"target_va":5391284720,"va":5377495715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13790492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e81c6dd200","size":5,"soft_fail":false,"target_va":5391286608,"va":5377496111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158b9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13805010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377495584,"raw_hex":"e8d2a5d200","size":5,"soft_fail":false,"target_va":5391301024,"va":5377496009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37415242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e84ae93a02","size":5,"soft_fail":false,"target_va":5411318240,"va":5373902993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::is_default_arch_height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4004031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902240,"raw_hex":"e8bf183d00","size":5,"soft_fail":false,"target_va":5377906320,"va":5373902284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::is_initial
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":480,"record_offset":53600,"parent_record_offset":51912,"ancestor_record_offset":51912,"inlinee":26024,"source_line":220,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4500,"parent_record_offset":4204,"ancestor_record_offset":4204,"inlinee":14648,"source_line":139,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::is_default_arch_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":53488,"parent_record_offset":51912,"ancestor_record_offset":51912,"inlinee":26610,"source_line":169,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":5144,"parent_record_offset":4204,"ancestor_record_offset":4204,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":5968,"parent_record_offset":4204,"ancestor_record_offset":4204,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":5688,"parent_record_offset":4204,"ancestor_record_offset":4204,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":5540,"parent_record_offset":5508,"ancestor_record_offset":4204,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":4204,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":5468,"parent_record_offset":5436,"ancestor_record_offset":4204,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":4204,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":4204,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":199,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":4204,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":4204,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":5508,"parent_record_offset":5340,"ancestor_record_offset":4204,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":4204,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":5628,"parent_record_offset":5340,"ancestor_record_offset":4204,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":5380,"parent_record_offset":5340,"ancestor_record_offset":4204,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":199,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4204,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":4204,"inlinee":8501,"source_line":1740,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":4204,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4204,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4924,"parent_record_offset":4896,"ancestor_record_offset":4204,"inlinee":14656,"source_line":27,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$23::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4832,"parent_record_offset":4804,"ancestor_record_offset":4204,"inlinee":14652,"source_line":49,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":4204,"inlinee":14658,"source_line":143,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$38::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":4204,"inlinee":14654,"source_line":147,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4648,"parent_record_offset":4620,"ancestor_record_offset":4204,"inlinee":14650,"source_line":19,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":4204,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":4204,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":4204,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":199,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":4204,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":4204,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":199,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":4204,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$25::clone
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=41
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D0F
// 首条位置: {"module":497,"inlinee":27919}
// 编译实例 RVAs: 0xa82390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4646905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379728272,"raw_hex":"e80718b9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379728388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4647335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379728272,"raw_hex":"e85916b9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379728834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32118723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379728272,"raw_hex":"e8c317ea01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379728440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31812213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379728272,"raw_hex":"e8756ae501","size":5,"soft_fail":false,"target_va":5411540963,"va":5379728745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=26
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":497,"record_offset":36404,"parent_record_offset":34852,"ancestor_record_offset":34852,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=29
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":34920,"parent_record_offset":34852,"ancestor_record_offset":34852,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":35428,"parent_record_offset":35404,"ancestor_record_offset":34852,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":35452,"parent_record_offset":35428,"ancestor_record_offset":34852,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":36352,"parent_record_offset":36328,"ancestor_record_offset":34852,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":35404,"parent_record_offset":35228,"ancestor_record_offset":34852,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":36328,"parent_record_offset":36240,"ancestor_record_offset":34852,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":35168,"parent_record_offset":35140,"ancestor_record_offset":34852,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":36268,"parent_record_offset":36240,"ancestor_record_offset":34852,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":36240,"parent_record_offset":36216,"ancestor_record_offset":34852,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":35228,"parent_record_offset":35196,"ancestor_record_offset":34852,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":35196,"parent_record_offset":35168,"ancestor_record_offset":34852,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":36216,"parent_record_offset":36192,"ancestor_record_offset":34852,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":35264,"parent_record_offset":35228,"ancestor_record_offset":34852,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":35104,"parent_record_offset":35072,"ancestor_record_offset":34852,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":35072,"parent_record_offset":35004,"ancestor_record_offset":34852,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":508,"record_offset":107828,"parent_record_offset":107800,"ancestor_record_offset":107700,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":35044,"parent_record_offset":35004,"ancestor_record_offset":34852,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":35140,"parent_record_offset":35104,"ancestor_record_offset":34852,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":34972,"parent_record_offset":34920,"ancestor_record_offset":34852,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":35004,"parent_record_offset":34972,"ancestor_record_offset":34852,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":497,"record_offset":35288,"parent_record_offset":35264,"ancestor_record_offset":34852,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":497,"record_offset":35312,"parent_record_offset":35288,"ancestor_record_offset":34852,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":497,"record_offset":35528,"parent_record_offset":35504,"ancestor_record_offset":34852,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":35336,"parent_record_offset":35312,"ancestor_record_offset":34852,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":35360,"parent_record_offset":35336,"ancestor_record_offset":34852,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":36296,"parent_record_offset":36268,"ancestor_record_offset":34852,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":35504,"parent_record_offset":35104,"ancestor_record_offset":34852,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":36168,"parent_record_offset":34920,"ancestor_record_offset":34852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3938
// 首条位置: {"module":199,"inlinee":14648}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4500,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$19::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":5052,"inlinee":8501,"source_line":1740,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":199,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":4500,"inlinee":8507,"source_line":1740,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4924,"parent_record_offset":4896,"ancestor_record_offset":4868,"inlinee":14656,"source_line":27,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$23::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4832,"parent_record_offset":4804,"ancestor_record_offset":4776,"inlinee":14652,"source_line":49,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$32::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":4960,"inlinee":14658,"source_line":143,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$38::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":4684,"inlinee":14654,"source_line":147,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":199,"record_offset":4648,"parent_record_offset":4620,"ancestor_record_offset":4592,"inlinee":14650,"source_line":19,"archive":"_pdb/B/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67F5
// 首条位置: {"module":480,"inlinee":26613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":269400,"parent_record_offset":269348,"ancestor_record_offset":269348,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":269372,"parent_record_offset":269348,"ancestor_record_offset":269348,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$29::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":50088,"rva":11041520,"va":5379750640}
// 编译实例 RVAs: 0xa87af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3143598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379750640,"raw_hex":"e8aef72f00","size":5,"soft_fail":false,"target_va":5382894288,"va":5379750685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$32::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3942
// 首条位置: {"module":199,"inlinee":14658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::impl$38::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x393E
// 首条位置: {"module":199,"inlinee":14654}
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 156, 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B3E
// 首条位置: {"module":494,"record_offset":152340,"rva":9197184,"va":5377906304}
// 编译实例 RVAs: 0x8c5680
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
// 记录形式: compiled_procedure; instances=1
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":494,"record_offset":152492,"rva":8598064,"va":5377307184}
// 编译实例 RVAs: 0x833230
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
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67F2
// 首条位置: {"module":480,"inlinee":26610}
// 编译实例 RVAs: 0x8c5690
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
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x675B
// 首条位置: {"module":479,"inlinee":26459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":479,"record_offset":26708,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":25006,"source_line":205,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::WallState::height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B42
// 首条位置: {"module":566,"inlinee":27458}
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
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B44
// 首条位置: {"module":553,"inlinee":27460}
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
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":494,"record_offset":152788,"rva":9197232,"va":5377906352}
// 编译实例 RVAs: 0x8c56b0
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
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B30
// 首条位置: {"module":494,"inlinee":27440}
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
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B48
// 首条位置: {"module":564,"inlinee":27464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121944,"parent_record_offset":121916,"ancestor_record_offset":121916,"inlinee":27441,"source_line":176,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121968,"parent_record_offset":121944,"ancestor_record_offset":121916,"inlinee":27443,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":480,"record_offset":25452,"rva":8776640,"va":5377485760}
// 编译实例 RVAs: 0x85ebc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33992064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e880ad0602","size":5,"soft_fail":false,"target_va":5411478016,"va":5377485947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2404610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e8fe4edbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377486109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t168288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e860910200","size":5,"soft_fail":false,"target_va":5377654256,"va":5377485963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088a500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t177845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e8b5b60200","size":5,"soft_fail":false,"target_va":5377664256,"va":5377486406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088e430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t194455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e897f70200","size":5,"soft_fail":false,"target_va":5377680432,"va":5377485972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415abad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13946393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e819ced400","size":5,"soft_fail":false,"target_va":5391432400,"va":5377486002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inner_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34188670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e87ead0902","size":5,"soft_fail":false,"target_va":5411674944,"va":5377486269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::MapDeserializer::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3314792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377485760,"raw_hex":"e868943200","size":5,"soft_fail":false,"target_va":5380800720,"va":5377485923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":480,"record_offset":25644,"parent_record_offset":25452,"ancestor_record_offset":25452,"inlinee":19010,"source_line":212,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":26400,"parent_record_offset":26376,"ancestor_record_offset":25452,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":480,"record_offset":26376,"parent_record_offset":26312,"ancestor_record_offset":25452,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":26344,"parent_record_offset":26312,"ancestor_record_offset":25452,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":26312,"parent_record_offset":26284,"ancestor_record_offset":25452,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":480,"record_offset":26284,"parent_record_offset":26256,"ancestor_record_offset":25452,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":25856,"parent_record_offset":25824,"ancestor_record_offset":25452,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":480,"record_offset":26200,"parent_record_offset":25732,"ancestor_record_offset":25452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":480,"record_offset":25888,"parent_record_offset":25856,"ancestor_record_offset":25452,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":480,"record_offset":25920,"parent_record_offset":25888,"ancestor_record_offset":25452,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":480,"record_offset":25824,"parent_record_offset":25776,"ancestor_record_offset":25452,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":25704,"parent_record_offset":25644,"ancestor_record_offset":25452,"inlinee":26600,"source_line":270,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":480,"record_offset":25776,"parent_record_offset":25732,"ancestor_record_offset":25452,"inlinee":26601,"source_line":280,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":26916,"parent_record_offset":26824,"ancestor_record_offset":25452,"inlinee":6474,"source_line":1455,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":26860,"parent_record_offset":26824,"ancestor_record_offset":25452,"inlinee":6475,"source_line":1355,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":26732,"parent_record_offset":26684,"ancestor_record_offset":25452,"inlinee":10893,"source_line":1654,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":25732,"parent_record_offset":25644,"ancestor_record_offset":25452,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":26888,"parent_record_offset":26824,"ancestor_record_offset":25452,"inlinee":6473,"source_line":1417,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":480,"record_offset":26796,"parent_record_offset":25644,"ancestor_record_offset":25452,"inlinee":10892,"source_line":1619,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":480,"record_offset":26824,"parent_record_offset":25644,"ancestor_record_offset":25452,"inlinee":19013,"source_line":470,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::visit_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":480,"record_offset":26684,"parent_record_offset":25644,"ancestor_record_offset":25452,"inlinee":26603,"source_line":190,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67DB
// 首条位置: {"module":480,"inlinee":26587}
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
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67E8
// 首条位置: {"module":480,"inlinee":26600}
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
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67E9
// 首条位置: {"module":480,"inlinee":26601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":480,"record_offset":25824,"parent_record_offset":25776,"ancestor_record_offset":25776,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":480,"record_offset":25856,"parent_record_offset":25824,"ancestor_record_offset":25776,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":480,"record_offset":25888,"parent_record_offset":25856,"ancestor_record_offset":25776,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":480,"record_offset":25920,"parent_record_offset":25888,"ancestor_record_offset":25776,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00480.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType>", "constructor": "serde::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::ArchType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::FreehandHeightType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::FreehandHeightType>", "constructor": "serde::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::FreehandHeightType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::PlasterPeelAmount
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::PlasterPeelAmount,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>", "constructor": "serde::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::PlasterPeelAmount"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::StructureType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType>", "constructor": "serde::de::impl$6::next_element", "arguments": ["serde_json::value::de::SeqDeserializer", "country_core::resources::walls::inner_wall_state::StructureType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 37
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "core::option::Option", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Drain", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "country_core::resources::walls::WallId", "country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::walls::inner_wall_state::WallState>", "constructor": "ref$", "arguments": ["country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "ref$<country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "tuple$", "arguments": ["tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >", "country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIter::drop_elements<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "hashbrown::raw::RawIter::drop_elements", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "country_core::resources::walls::inner_walls::impl$1::all::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>", "country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::WallId", "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$87::from_iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::impl$87::from_iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$5::fold<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > > >", "constructor": "alloc::vec::into_iter::impl$5::fold", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "constructor": "serde::de::impls::impl$188::deserialize::TupleVisitor", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::inner_wall_state::WallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_tuple<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_json::value::de::impl$2::deserialize_tuple", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "ref$", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "ref$", "arguments": ["ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > > >", "constructor": "ref$", "arguments": ["ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > > > >", "constructor": "serde_json::value::to_value", "arguments": ["ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::WallId,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::WallStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallStyle>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::walls::inner_wall_state::WallStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde::de::impl$1::fmt", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>", "constructor": "serde_json::value::de::visit_array", "arguments": ["country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: inner_wall_state.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-state/B-focused/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::inner_wall_state::WallStyle", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: inner_wall_state.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 26; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "country_core::systems::ui::resources::slider3d::Slider3d", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle
// 信息 ID: 10343802238baa7d87bc4a1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::color_menu::color_menu @ 0x22933bc -> 0x8c5660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::StructureType::is_fence
// 信息 ID: 9d8f756a23941a5a65f18e3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 31 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::rollercoaster::rollercoaster_update @ 0x3b865d -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c66 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c85 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4dde -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4e67 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x688817 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75c9d8 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x76c2f2 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6b80 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a68e6 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a696b -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a68f4 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a55fc -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac0e8 -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac12f -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac1aa -> 0x8c5680 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::ui_continue_wall::ui_continue_wall; module=563, inline_record=38744; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls; module=566, inline_record=47900; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=78064; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::StructureType::is_wall
// 信息 ID: 9f7543ae9cfc016787812ff9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::write_settings_file @ 0x354dbb -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f3de6 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f3f61 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4c55 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::decorator_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f4d9d -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7581ec -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7585f8 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758ffa -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758317 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x76178b -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121d388 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e208a2 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::color_menu::color_menu @ 0x2293830 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::color_menu::color_menu @ 0x2293b91 -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231dbfa -> 0x833230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x231dce2 -> 0x833230 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
// 信息 ID: 1790cb0bcb370d1661ac8934
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=121916; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=122100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
// 信息 ID: 745cc3fa99d8d15a63129e2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x819df7 -> 0x8c56b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::height
// 信息 ID: 807b8f454667eec9c3980868
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::is_enclosed
// 信息 ID: f096afe56651437bf3d30eeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
// 信息 ID: b922f26bd698d7e1ba9a81ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=553, inline_record=70640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=554, inline_record=78028; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl; module=564, inline_record=128576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=66720; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=79720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallState::new
// 信息 ID: 48a6cc380627a412c59ada71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve; module=479, inline_record=26676; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve; module=479, inline_record=26768; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_walls::InnerWalls::new_wall_w_params; module=479, inline_record=27656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
// 信息 ID: f70779d75608606db4c34461
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=96388; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=102860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=553, inline_record=71164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze; module=564, inline_record=132232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=81612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
// 信息 ID: 9d0c733809f5fa147de48741
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 40 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe487 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d8c7 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758760 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7587eb -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758d74 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x75832b -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x78883e -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_columns @ 0x78924b -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78bfad -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78c182 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78c276 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x7c5f93 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x7e900c -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f1c7b -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::generate_playermade_stair_decorators @ 0x805216 -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::should_snap_to_corner @ 0x81194d -> 0x8c5670 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=519, inline_record=48072; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=556, inline_record=63612; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=141676; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
// 信息 ID: 934b892ef134e67c1df2feca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<serde::de::impls::impl$188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> >; module=1630, inline_record=113312; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=269300; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=325528; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=325976; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>; module=1630, inline_record=464420; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=525660; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=526052; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=526740; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
// 信息 ID: dd6cc4bd25f00bd007ff0f29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=269348; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>; module=1630, inline_record=464472; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=525708; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=526100; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=527084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: fae31144f56edea22cc30809
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1630, inline_record=477704; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1630, inline_record=487688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 2d4e6187d62c2d1051723008
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> > @ 0x15d1ed6 -> 0x860d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x15bb37d -> 0x861520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: a616108b9b1c64cd86b51e27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=264580; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1630, inline_record=476580; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1630, inline_record=477576; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Field> >; module=1630, inline_record=487560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 8ba024f34d7c19e89d455c41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=480, inline_record=1540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: d271bedf6787dccb93db08fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=480, inline_record=36500; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=480, inline_record=40556; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=480, inline_record=46452; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=480, inline_record=56600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 6abd3062835dc75fc23a0566
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=262996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 753c662954792173ee7f1b73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=265760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 信息 ID: 14fb27b55dbeb69628dca64b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=268960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 46c07f67c125508e268b2d97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=166840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: 6248920b749cb42c8914b196
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=167144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 信息 ID: d57101e1eccf6c39b46b6f7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith>; module=1630, inline_record=462364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 04bb821de992df9555dc1bc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor> @ 0x15aa129 -> 0x85ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType> | serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::FreehandHeightType> @ 0x15d0acd -> 0x85ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e1800 -> 0x85ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e186b -> 0x85ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e4650 -> 0x85ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e46bb -> 0x85ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor> @ 0x15aa0c3 -> 0x8606b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15dd9f0 -> 0x8606b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15dda5b -> 0x8606b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType> @ 0x15d0ced -> 0x861780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize
// 信息 ID: b7cd43ee668b9ff494a7d905
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::FreehandHeightType>; module=174, inline_record=50236; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::FreehandHeightType>; module=191, inline_record=86212; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallStyle>; module=191, inline_record=122876; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::StructureType>; module=191, inline_record=223660; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallStyle>; module=1627, inline_record=71508; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::ArchType>; module=1627, inline_record=100476; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>; module=1627, inline_record=109880; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::StructureType>; module=1627, inline_record=112564; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize::impl$0::serialize
// 信息 ID: 1bad2cce889322cda201e657
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::_::impl$0::serialize::__SerializeWith>; module=1628, inline_record=147980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: e509a05f164c143b2eeb6629
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallState> @ 0x4b8a68 -> 0x4f3da0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > > @ 0x4bf852 -> 0x4f3da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > > @ 0x15836b2 -> 0x861220 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::WallState> @ 0x15841a8 -> 0x861220 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 8bc30ca138d007d90cea89fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::walls::inner_wall_state::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x3e3dae -> 0x4f40a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<country_core::resources::walls::inner_wall_state::WallStyle> @ 0x1593aee -> 0x8605a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::to_value<ref$<ref$<ref$<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState> > > > > @ 0x15a4481 -> 0x860e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::expecting
// 信息 ID: 4554e02fb6aa867e0ed3c34b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>; module=480, inline_record=884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_bool
// 信息 ID: bfff4f3a414b3e0b26c4dbf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >; module=480, inline_record=25704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style::impl$0::visit_str
// 信息 ID: bcc7d830f27ed990483ef74e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >; module=480, inline_record=25776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
// 信息 ID: 14fc9e77dd22a1093b93619a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::walls::inner_wall_state::deserialize_wall_style -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith> @ 0x15d0f90 -> 0x85ebc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$14::fmt
// 信息 ID: 45dceb7548c8519b74121b8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$16::eq
// 信息 ID: f5ee4002f185c17c5c26bff7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4924; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=51176; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=480, inline_record=52732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=545, inline_record=22172; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=37480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$21::fmt
// 信息 ID: 2d599251b4eeb1b4dfb43914
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$23::eq
// 信息 ID: 3a58d07068c725b5549c0f44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4832; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=50992; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=480, inline_record=52548; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game; module=545, inline_record=22144; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=35596; 不是独立调用边。
// [PDB 内联宿主] system_color::radial_menu::ColorRadialMenu::create; module=4471, inline_record=4744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$25::clone
// 信息 ID: a904d55a233dfe36eaa13ab3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 30 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa80e07 -> 0xa82390 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::deferred_wall_history::history_record_merge_walls; module=497, inline_record=34852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall; module=508, inline_record=79052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=96472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=99344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=102888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=107700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=109128; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls; module=519, inline_record=40084; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$29::fmt
// 信息 ID: f8f70f1c06a9b0381ab6db33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$32::eq
// 信息 ID: 140d4286c151cfe204c53998
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=5016; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=50808; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=480, inline_record=52364; 不是独立调用边。
// [PDB 内联宿主] system_color::color_menu::color_menu; module=3756, inline_record=37712; 不是独立调用边。
// [PDB 内联宿主] system_color::radial_menu::ColorRadialMenu::create; module=4471, inline_record=4828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$38::eq
// 信息 ID: fd768d23d165e888a6fd7025
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4740; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=51084; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=480, inline_record=52640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=57664; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=60668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall::closure$0; module=554, inline_record=85464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$7::fmt
// 信息 ID: a30c234e742a9113e25fb71a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::impl$9::eq
// 信息 ID: 98625000a322ae96a25689cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4648; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=50900; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=480, inline_record=52456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::is_default
// 信息 ID: d528b4802a8cfd53e89dc525
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4500; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4592; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4684; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4776; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4868; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=4960; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=199, inline_record=5052; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=50660; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::inner_wall_state::is_default_arch_height
// 信息 ID: cf9c68c49f171c395956076f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f3dcc -> 0x8c5690 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x4f3ebf -> 0x8c5690 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=480, inline_record=51852; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::inner_wall_state::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=480, inline_record=53488; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
