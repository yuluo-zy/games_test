// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/valid_enclosure/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e6c66638cf88a4e97a2aadda181d5540520e6f2e73316941af308024055b0f36
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 86, 435, 497, 498, 508, 515, 519, 524, 533, 554, 555, 556, 558, 563, 564, 566, 567, 572, 1412, 1532, 1584, 1628, 1629, 1640, 1673, 2599, 3098, 3465, 3467, 4342
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 11, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":57296,"rva":24034032,"va":5392743152}
// 编译实例 RVAs: 0x16ebaf0, 0x16ec620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=4
//     target_aliases=92 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17842849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e85fbdeffe","size":5,"soft_fail":false,"target_va":5374900736,"va":5392743580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17662005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e8cb7ff2fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392743504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4624121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e80771b9ff","size":5,"soft_fail":false,"target_va":5388119328,"va":5392743444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4626680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392746016,"raw_hex":"e80867b9ff","size":5,"soft_fail":false,"target_va":5388119648,"va":5392746323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4623844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e81c72b9ff","size":5,"soft_fail":false,"target_va":5388119808,"va":5392743647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141283000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4626471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392746016,"raw_hex":"e8d967b9ff","size":5,"soft_fail":false,"target_va":5388120064,"va":5392746530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415b5140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1272325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e8fb95ecff","size":5,"soft_fail":false,"target_va":5391470912,"va":5392743232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ba5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1250826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e8f6e9ecff","size":5,"soft_fail":false,"target_va":5391492528,"va":5392743349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428fa000
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18932329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e869e22001","size":5,"soft_fail":false,"target_va":5411676160,"va":5392743826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18795092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392746016,"raw_hex":"e854ca1e01","size":5,"soft_fail":false,"target_va":5411541904,"va":5392746807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-18863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392743152,"raw_hex":"e851b6ffff","size":5,"soft_fail":false,"target_va":5392724672,"va":5392743530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10834783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392746016,"raw_hex":"e8a1ac5aff","size":5,"soft_fail":false,"target_va":5381911872,"va":5392746650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":58816,"parent_record_offset":57296,"ancestor_record_offset":57296,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":57488,"parent_record_offset":57296,"ancestor_record_offset":57296,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":58056,"parent_record_offset":58032,"ancestor_record_offset":57296,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":58032,"parent_record_offset":58000,"ancestor_record_offset":57296,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1673,"record_offset":58000,"parent_record_offset":57972,"ancestor_record_offset":57296,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1673,"record_offset":67556,"parent_record_offset":67500,"ancestor_record_offset":65900,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1673,"record_offset":58852,"parent_record_offset":58816,"ancestor_record_offset":57296,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":57820,"parent_record_offset":57488,"ancestor_record_offset":57296,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":57784,"parent_record_offset":57488,"ancestor_record_offset":57296,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":57720,"parent_record_offset":57488,"ancestor_record_offset":57296,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":57756,"parent_record_offset":57488,"ancestor_record_offset":57296,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":58880,"parent_record_offset":58852,"ancestor_record_offset":57296,"inlinee":70694,"source_line":46,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":67500,"parent_record_offset":67464,"ancestor_record_offset":65900,"inlinee":70707,"source_line":290,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":57524,"parent_record_offset":57488,"ancestor_record_offset":57296,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":57680,"parent_record_offset":57580,"ancestor_record_offset":57296,"inlinee":19164,"source_line":335,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":57652,"parent_record_offset":57580,"ancestor_record_offset":57296,"inlinee":19163,"source_line":433,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1673,"record_offset":57552,"parent_record_offset":57524,"ancestor_record_offset":57296,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1673,"record_offset":57624,"parent_record_offset":57580,"ancestor_record_offset":57296,"inlinee":19013,"source_line":470,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1673,"record_offset":57580,"parent_record_offset":57552,"ancestor_record_offset":57296,"inlinee":19010,"source_line":212,"archive":"_pdb/B/modules/01673.jsonl.gz"}
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
// 首条位置: {"module":1673,"record_offset":58964,"rva":24034928,"va":5392744048}
// 编译实例 RVAs: 0x16ebe70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18931005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744048,"raw_hex":"e83ddd2001","size":5,"soft_fail":false,"target_va":5411675072,"va":5392744062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":59320,"parent_record_offset":58964,"ancestor_record_offset":58964,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":59212,"parent_record_offset":58964,"ancestor_record_offset":58964,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":59232,"parent_record_offset":59212,"ancestor_record_offset":58964,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":59280,"parent_record_offset":59256,"ancestor_record_offset":58964,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1673,"record_offset":59256,"parent_record_offset":59232,"ancestor_record_offset":58964,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=4
// 源码行: 11, 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":86,"record_offset":240,"rva":2462464,"va":5371171584}
// 编译实例 RVAs: 0x259300, 0x259550, 0x16eb8a0, 0x16ec2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400efde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1480691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371172176,"raw_hex":"e80d68e9ff","size":5,"soft_fail":false,"target_va":5369691616,"va":5371172302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d8a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1569835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371172176,"raw_hex":"e82bf41700","size":5,"soft_fail":false,"target_va":5372742256,"va":5371172416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2477506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371171584,"raw_hex":"e8c2cd2500","size":5,"soft_fail":false,"target_va":5373649152,"va":5371171641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b90f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2489550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371171584,"raw_hex":"e8cefc2500","size":5,"soft_fail":false,"target_va":5373661424,"va":5371171869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157c4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1506988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392745184,"raw_hex":"e85401e9ff","size":5,"soft_fail":false,"target_va":5391238336,"va":5392745319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141580d20
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1485758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392742560,"raw_hex":"e84254e9ff","size":5,"soft_fail":false,"target_va":5391256864,"va":5392742617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141585400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1470698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392745184,"raw_hex":"e8168fe9ff","size":5,"soft_fail":false,"target_va":5391275008,"va":5392745701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1467348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392745184,"raw_hex":"e82c9ce9ff","size":5,"soft_fail":false,"target_va":5391278224,"va":5392745567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1462882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392742560,"raw_hex":"e89eade9ff","size":5,"soft_fail":false,"target_va":5391279968,"va":5392742845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158ce20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1439268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392745184,"raw_hex":"e8dc09eaff","size":5,"soft_fail":false,"target_va":5391306272,"va":5392745535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=22
//     evidence={"assembly":"\tcallq\t40146248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371171584,"raw_hex":"e848956402","size":5,"soft_fail":false,"target_va":5411318240,"va":5371171987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40305658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371171584,"raw_hex":"e8fa036702","size":5,"soft_fail":false,"target_va":5411477664,"va":5371172001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t40305956","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371171584,"raw_hex":"e924056702","size":5,"soft_fail":false,"target_va":5411477664,"va":5371171703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":2492,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5726,"source_line":808,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":2436,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5725,"source_line":794,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":3780,"parent_record_offset":3500,"ancestor_record_offset":3500,"inlinee":5418,"source_line":268,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":808,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5433,"source_line":420,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":532,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5432,"source_line":249,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1204,"parent_record_offset":1172,"ancestor_record_offset":240,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":240,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1132,"parent_record_offset":1100,"ancestor_record_offset":240,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":240,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":240,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":86,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":240,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":1004,"parent_record_offset":976,"ancestor_record_offset":240,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":1172,"parent_record_offset":1004,"ancestor_record_offset":240,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":948,"parent_record_offset":920,"ancestor_record_offset":240,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":1292,"parent_record_offset":1004,"ancestor_record_offset":240,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":86,"record_offset":1044,"parent_record_offset":1004,"ancestor_record_offset":240,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":86,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":240,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1673,"record_offset":64644,"parent_record_offset":64616,"ancestor_record_offset":62108,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":86,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":240,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":86,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":240,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":864,"parent_record_offset":808,"ancestor_record_offset":240,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":1440,"parent_record_offset":808,"ancestor_record_offset":240,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":2532,"parent_record_offset":2492,"ancestor_record_offset":240,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":1920,"parent_record_offset":808,"ancestor_record_offset":240,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":240,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":64708,"parent_record_offset":64576,"ancestor_record_offset":62108,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":65340,"parent_record_offset":65308,"ancestor_record_offset":62108,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1673,"record_offset":65308,"parent_record_offset":64576,"ancestor_record_offset":62108,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":86,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":240,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":86,"record_offset":920,"parent_record_offset":892,"ancestor_record_offset":240,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":86,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":240,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1673,"record_offset":64672,"parent_record_offset":64644,"ancestor_record_offset":62108,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1673,"record_offset":64576,"parent_record_offset":63472,"ancestor_record_offset":62108,"inlinee":13939,"source_line":16,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::is_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1673,"record_offset":64616,"parent_record_offset":64576,"ancestor_record_offset":62108,"inlinee":13932,"source_line":27,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":52240,"rva":24032832,"va":5392741952}
// 编译实例 RVAs: 0x16eb640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeStructVariant$u20$as$u20$serde..ser..SerializeStructVariant$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11948496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e830ae49ff","size":5,"soft_fail":false,"target_va":5380793920,"va":5392742411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11948811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e8f5ac49ff","size":5,"soft_fail":false,"target_va":5380793392,"va":5392742198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17660544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e88085f2fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392742027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17660816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e87084f2fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392742315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087a9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15142507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e895f118ff","size":5,"soft_fail":false,"target_va":5377599936,"va":5392742438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088c930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15069034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e896101aff","size":5,"soft_fail":false,"target_va":5377673520,"va":5392742549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158e9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1428892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e86432eaff","size":5,"soft_fail":false,"target_va":5391313392,"va":5392742279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18798484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392741952,"raw_hex":"e894d71e01","size":5,"soft_fail":false,"target_va":5411540963,"va":5392742474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":53600,"parent_record_offset":52240,"ancestor_record_offset":52240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":53512,"parent_record_offset":52240,"ancestor_record_offset":52240,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":53544,"parent_record_offset":52240,"ancestor_record_offset":52240,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1673,"record_offset":52440,"parent_record_offset":52240,"ancestor_record_offset":52240,"inlinee":8911,"source_line":170,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":52812,"parent_record_offset":52788,"ancestor_record_offset":52240,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":52836,"parent_record_offset":52812,"ancestor_record_offset":52240,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":53804,"parent_record_offset":53780,"ancestor_record_offset":52240,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":52788,"parent_record_offset":52760,"ancestor_record_offset":52240,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":53780,"parent_record_offset":53724,"ancestor_record_offset":52240,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1673,"record_offset":53912,"parent_record_offset":53884,"ancestor_record_offset":52240,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1673,"record_offset":53984,"parent_record_offset":53912,"ancestor_record_offset":52240,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":52700,"parent_record_offset":52672,"ancestor_record_offset":52240,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":53752,"parent_record_offset":53724,"ancestor_record_offset":52240,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":53724,"parent_record_offset":53700,"ancestor_record_offset":52240,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":52760,"parent_record_offset":52728,"ancestor_record_offset":52240,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":52728,"parent_record_offset":52700,"ancestor_record_offset":52240,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":53700,"parent_record_offset":53676,"ancestor_record_offset":52240,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1673,"record_offset":52636,"parent_record_offset":52608,"ancestor_record_offset":52240,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1673,"record_offset":52608,"parent_record_offset":52580,"ancestor_record_offset":52240,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1673,"record_offset":52552,"parent_record_offset":52524,"ancestor_record_offset":52240,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1673,"record_offset":52580,"parent_record_offset":52552,"ancestor_record_offset":52240,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1673,"record_offset":52524,"parent_record_offset":52496,"ancestor_record_offset":52240,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1673,"record_offset":52496,"parent_record_offset":52468,"ancestor_record_offset":52240,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":52672,"parent_record_offset":52636,"ancestor_record_offset":52240,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1673,"record_offset":52912,"parent_record_offset":52888,"ancestor_record_offset":52240,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1673,"record_offset":52888,"parent_record_offset":52636,"ancestor_record_offset":52240,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":53628,"parent_record_offset":53600,"ancestor_record_offset":52240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":53940,"parent_record_offset":53912,"ancestor_record_offset":52240,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1673,"record_offset":52468,"parent_record_offset":52440,"ancestor_record_offset":52240,"inlinee":8912,"source_line":150,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=44
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D11
// 首条位置: {"module":497,"inlinee":27921}
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
// 首条位置: {"module":555,"record_offset":50348,"rva":11041648,"va":5379750768}
// 编译实例 RVAs: 0xa87b70, 0x21aae00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3141610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379750768,"raw_hex":"e8eaef2f00","size":5,"soft_fail":false,"target_va":5382892528,"va":5379750913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3141603","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379750768,"raw_hex":"e9e3ef2f00","size":5,"soft_fail":false,"target_va":5382892432,"va":5379750824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 22, 32, 38, 263, 269, 275, 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1182D
// 首条位置: {"module":1532,"inlinee":71725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":57820,"parent_record_offset":57488,"ancestor_record_offset":57488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":57784,"parent_record_offset":57488,"ancestor_record_offset":57488,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":57720,"parent_record_offset":57488,"ancestor_record_offset":57488,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":57756,"parent_record_offset":57488,"ancestor_record_offset":57488,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":57524,"parent_record_offset":57488,"ancestor_record_offset":57488,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":58056,"parent_record_offset":58032,"ancestor_record_offset":57488,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":58032,"parent_record_offset":58000,"ancestor_record_offset":57488,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1673,"record_offset":58000,"parent_record_offset":57972,"ancestor_record_offset":57488,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":57852,"parent_record_offset":57820,"ancestor_record_offset":57488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":67932,"parent_record_offset":67908,"ancestor_record_offset":67868,"inlinee":62297,"source_line":2061,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":57680,"parent_record_offset":57580,"ancestor_record_offset":57488,"inlinee":19164,"source_line":335,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":57652,"parent_record_offset":57580,"ancestor_record_offset":57488,"inlinee":19163,"source_line":433,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":59280,"parent_record_offset":59256,"ancestor_record_offset":59212,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1673,"record_offset":57552,"parent_record_offset":57524,"ancestor_record_offset":57488,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$8::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1673,"record_offset":57624,"parent_record_offset":57580,"ancestor_record_offset":57488,"inlinee":19013,"source_line":470,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$2::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1673,"record_offset":57580,"parent_record_offset":57552,"ancestor_record_offset":57488,"inlinee":19010,"source_line":212,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=3
// 源码行: 22, 32, 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":60132,"rva":24035184,"va":5392744304}
// 编译实例 RVAs: 0x16ebf70, 0x16ec1f0, 0x16ecef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17663190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744304,"raw_hex":"e82a7bf2fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392744689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18931025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744304,"raw_hex":"e851dd2001","size":5,"soft_fail":false,"target_va":5411675680,"va":5392744650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::__private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10091201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744304,"raw_hex":"e83f0566ff","size":5,"soft_fail":false,"target_va":5382653424,"va":5392744620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":60332,"parent_record_offset":60132,"ancestor_record_offset":60132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":60500,"parent_record_offset":60476,"ancestor_record_offset":60132,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":60476,"parent_record_offset":60452,"ancestor_record_offset":60132,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":60452,"parent_record_offset":60428,"ancestor_record_offset":60132,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":60428,"parent_record_offset":60404,"ancestor_record_offset":60132,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":60356,"parent_record_offset":60332,"ancestor_record_offset":60132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":59352,"rva":24034976,"va":5392744096}
// 编译实例 RVAs: 0x16ebea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18931401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744096,"raw_hex":"e8c9de2001","size":5,"soft_fail":false,"target_va":5411675680,"va":5392744274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1673,"record_offset":59552,"parent_record_offset":59352,"ancestor_record_offset":59352,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1673,"record_offset":59572,"parent_record_offset":59552,"ancestor_record_offset":59352,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1673,"record_offset":59592,"parent_record_offset":59572,"ancestor_record_offset":59352,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1673,"record_offset":59612,"parent_record_offset":59592,"ancestor_record_offset":59352,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 22, 32, 263, 269, 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x117FB
// 首条位置: {"module":1532,"inlinee":71675}
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
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 22, 32, 263, 269, 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x117F9
// 首条位置: {"module":1532,"inlinee":71673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":588,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":560,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":560,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":560,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":560,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":696,"parent_record_offset":588,"ancestor_record_offset":560,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":560,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":560,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":916,"parent_record_offset":588,"ancestor_record_offset":560,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":16500,"parent_record_offset":16472,"ancestor_record_offset":16192,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":16528,"parent_record_offset":16500,"ancestor_record_offset":16192,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":16556,"parent_record_offset":16528,"ancestor_record_offset":16192,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":16472,"parent_record_offset":16444,"ancestor_record_offset":16192,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":16332,"parent_record_offset":16220,"ancestor_record_offset":16192,"inlinee":71697,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":16444,"parent_record_offset":16416,"ancestor_record_offset":16192,"inlinee":71698,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":560,"inlinee":71675,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16360,"parent_record_offset":16220,"ancestor_record_offset":16192,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16416,"parent_record_offset":16220,"ancestor_record_offset":16192,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":644,"parent_record_offset":588,"ancestor_record_offset":560,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11824
// 首条位置: {"module":1532,"inlinee":71716}
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
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11825
// 首条位置: {"module":1532,"inlinee":71717}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":29372,"parent_record_offset":29344,"ancestor_record_offset":29344,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":29400,"parent_record_offset":29372,"ancestor_record_offset":29344,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":29428,"parent_record_offset":29400,"ancestor_record_offset":29344,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":29456,"parent_record_offset":29428,"ancestor_record_offset":29344,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11823
// 首条位置: {"module":1532,"inlinee":71715}
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
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11821
// 首条位置: {"module":1532,"inlinee":71713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":29096,"parent_record_offset":29056,"ancestor_record_offset":29056,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":29748,"parent_record_offset":29724,"ancestor_record_offset":29056,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":29796,"parent_record_offset":29772,"ancestor_record_offset":29056,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":29772,"parent_record_offset":29748,"ancestor_record_offset":29056,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":29676,"parent_record_offset":29652,"ancestor_record_offset":29056,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":29652,"parent_record_offset":29096,"ancestor_record_offset":29056,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":29724,"parent_record_offset":29700,"ancestor_record_offset":29056,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":29700,"parent_record_offset":29676,"ancestor_record_offset":29056,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":29400,"parent_record_offset":29372,"ancestor_record_offset":29056,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":29428,"parent_record_offset":29400,"ancestor_record_offset":29056,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":29456,"parent_record_offset":29428,"ancestor_record_offset":29056,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":29372,"parent_record_offset":29344,"ancestor_record_offset":29056,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29232,"parent_record_offset":29096,"ancestor_record_offset":29056,"inlinee":71716,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29344,"parent_record_offset":29316,"ancestor_record_offset":29056,"inlinee":71717,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29176,"parent_record_offset":29152,"ancestor_record_offset":29056,"inlinee":71715,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29260,"parent_record_offset":29096,"ancestor_record_offset":29056,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29316,"parent_record_offset":29096,"ancestor_record_offset":29056,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29152,"parent_record_offset":29096,"ancestor_record_offset":29056,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11439
// 首条位置: {"module":1673,"inlinee":70713}
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
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1181D
// 首条位置: {"module":1532,"inlinee":71709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":29868,"parent_record_offset":28812,"ancestor_record_offset":28812,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":29980,"parent_record_offset":28812,"ancestor_record_offset":28812,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":30012,"parent_record_offset":28812,"ancestor_record_offset":28812,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":28852,"parent_record_offset":28812,"ancestor_record_offset":28812,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29896,"parent_record_offset":28812,"ancestor_record_offset":28812,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":29748,"parent_record_offset":29724,"ancestor_record_offset":28812,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":29796,"parent_record_offset":29772,"ancestor_record_offset":28812,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1532,"record_offset":29772,"parent_record_offset":29748,"ancestor_record_offset":28812,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":29676,"parent_record_offset":29652,"ancestor_record_offset":28812,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1532,"record_offset":29652,"parent_record_offset":29096,"ancestor_record_offset":28812,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":29724,"parent_record_offset":29700,"ancestor_record_offset":28812,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1532,"record_offset":29700,"parent_record_offset":29676,"ancestor_record_offset":28812,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":29400,"parent_record_offset":29372,"ancestor_record_offset":28812,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1532,"record_offset":28980,"parent_record_offset":28940,"ancestor_record_offset":28812,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":29428,"parent_record_offset":29400,"ancestor_record_offset":28812,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":29456,"parent_record_offset":29428,"ancestor_record_offset":28812,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1532,"record_offset":28940,"parent_record_offset":28892,"ancestor_record_offset":28812,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":29372,"parent_record_offset":29344,"ancestor_record_offset":28812,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29232,"parent_record_offset":29096,"ancestor_record_offset":28812,"inlinee":71716,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29344,"parent_record_offset":29316,"ancestor_record_offset":28812,"inlinee":71717,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29176,"parent_record_offset":29152,"ancestor_record_offset":28812,"inlinee":71715,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":29056,"parent_record_offset":29016,"ancestor_record_offset":28812,"inlinee":71713,"source_line":32,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":29096,"parent_record_offset":29056,"ancestor_record_offset":28812,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$31::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":28892,"parent_record_offset":28852,"ancestor_record_offset":28812,"inlinee":71711,"source_line":2304,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$31::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":29920,"parent_record_offset":29896,"ancestor_record_offset":28812,"inlinee":71718,"source_line":2317,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29260,"parent_record_offset":29096,"ancestor_record_offset":28812,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29316,"parent_record_offset":29096,"ancestor_record_offset":28812,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29152,"parent_record_offset":29096,"ancestor_record_offset":28812,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":29016,"parent_record_offset":28892,"ancestor_record_offset":28812,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22, 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11829
// 首条位置: {"module":1532,"inlinee":71721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":30220,"parent_record_offset":30192,"ancestor_record_offset":30192,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":30324,"parent_record_offset":30268,"ancestor_record_offset":30192,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$29::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":30268,"parent_record_offset":30244,"ancestor_record_offset":30192,"inlinee":71723,"source_line":2259,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":30244,"parent_record_offset":30220,"ancestor_record_offset":30192,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":30296,"parent_record_offset":30268,"ancestor_record_offset":30192,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 22, 263, 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":60812,"rva":24035648,"va":5392744768}
// 编译实例 RVAs: 0x16ec140, 0x16ecda0, 0x16ed020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t18729801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747936,"raw_hex":"e849cb1d01","size":5,"soft_fail":false,"target_va":5411478016,"va":5392748210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4631474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392748576,"raw_hex":"e84e54b9ff","size":5,"soft_fail":false,"target_va":5388117120,"va":5392748589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4627202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744768,"raw_hex":"e8fe64b9ff","size":5,"soft_fail":false,"target_va":5388117584,"va":5392744781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4629090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747936,"raw_hex":"e89e5db9ff","size":5,"soft_fail":false,"target_va":5388118864,"va":5392747949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412830a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4627883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747936,"raw_hex":"e85562b9ff","size":5,"soft_fail":false,"target_va":5388120224,"va":5392748102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141283ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4625863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392748576,"raw_hex":"e8396ab9ff","size":5,"soft_fail":false,"target_va":5388122832,"va":5392748690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141284050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4623800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747936,"raw_hex":"e84872b9ff","size":5,"soft_fail":false,"target_va":5388124240,"va":5392748035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141284bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4617671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744768,"raw_hex":"e8398ab9ff","size":5,"soft_fail":false,"target_va":5388127184,"va":5392744850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f7ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18923028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392744768,"raw_hex":"e814be2001","size":5,"soft_fail":false,"target_va":5411667952,"va":5392744919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":69708,"parent_record_offset":69232,"ancestor_record_offset":69232,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":61104,"parent_record_offset":60812,"ancestor_record_offset":60812,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":61216,"parent_record_offset":60812,"ancestor_record_offset":60812,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":61132,"parent_record_offset":60812,"ancestor_record_offset":60812,"inlinee":71680,"source_line":2135,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1673,"record_offset":61076,"parent_record_offset":60812,"ancestor_record_offset":60812,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1673,"record_offset":69584,"parent_record_offset":69232,"ancestor_record_offset":69232,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":71128,"parent_record_offset":71104,"ancestor_record_offset":70556,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":61180,"parent_record_offset":61156,"ancestor_record_offset":60812,"inlinee":71681,"source_line":1905,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$26::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":69612,"parent_record_offset":69584,"ancestor_record_offset":69232,"inlinee":62303,"source_line":2142,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1673,"record_offset":69648,"parent_record_offset":69612,"ancestor_record_offset":69232,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1673,"record_offset":61156,"parent_record_offset":61132,"ancestor_record_offset":60812,"inlinee":6595,"source_line":33,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1673,"record_offset":69828,"parent_record_offset":69804,"ancestor_record_offset":69232,"inlinee":71724,"source_line":16,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1673,"record_offset":69672,"parent_record_offset":69648,"ancestor_record_offset":69232,"inlinee":71726,"source_line":114,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32, 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11435
// 首条位置: {"module":1673,"inlinee":70709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x117FF
// 首条位置: {"module":1532,"inlinee":71679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":6668,"parent_record_offset":6612,"ancestor_record_offset":6612,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":6696,"parent_record_offset":6612,"ancestor_record_offset":6612,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":6724,"parent_record_offset":6612,"ancestor_record_offset":6612,"inlinee":71680,"source_line":2135,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":6612,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":6780,"parent_record_offset":6752,"ancestor_record_offset":6612,"inlinee":71681,"source_line":1905,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":6752,"parent_record_offset":6724,"ancestor_record_offset":6612,"inlinee":6595,"source_line":33,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11426
// 首条位置: {"module":1673,"inlinee":70694}
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
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":67628,"rva":24037936,"va":5392747056}
// 编译实例 RVAs: 0x16eca30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17846622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e8a2aeeffe","size":5,"soft_fail":false,"target_va":5374900736,"va":5392747353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17665767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e81971f2fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392747266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141282e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4627563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e89563b9ff","size":5,"soft_fail":false,"target_va":5388119648,"va":5392747206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141283000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4627354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e86664b9ff","size":5,"soft_fail":false,"target_va":5388120064,"va":5392747413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141285b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4616003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e8bd90b9ff","size":5,"soft_fail":false,"target_va":5388131104,"va":5392747102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428fa000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18928537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e899d32001","size":5,"soft_fail":false,"target_va":5411676160,"va":5392747618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18794214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e8e6c61e01","size":5,"soft_fail":false,"target_va":5411541904,"va":5392747685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-22627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e89da7ffff","size":5,"soft_fail":false,"target_va":5392724672,"va":5392747294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10835670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392747056,"raw_hex":"e82aa95aff","size":5,"soft_fail":false,"target_va":5381911872,"va":5392747537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":69052,"parent_record_offset":67628,"ancestor_record_offset":67628,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":67868,"parent_record_offset":67628,"ancestor_record_offset":67628,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":68292,"parent_record_offset":68268,"ancestor_record_offset":67628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":68268,"parent_record_offset":68236,"ancestor_record_offset":67628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1673,"record_offset":68236,"parent_record_offset":68208,"ancestor_record_offset":67628,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1673,"record_offset":69188,"parent_record_offset":69132,"ancestor_record_offset":67628,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1673,"record_offset":69096,"parent_record_offset":69052,"ancestor_record_offset":67628,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":68056,"parent_record_offset":67868,"ancestor_record_offset":67628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":68020,"parent_record_offset":67868,"ancestor_record_offset":67628,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":67964,"parent_record_offset":67868,"ancestor_record_offset":67628,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":67996,"parent_record_offset":67868,"ancestor_record_offset":67628,"inlinee":71725,"source_line":275,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1673,"record_offset":69132,"parent_record_offset":69096,"ancestor_record_offset":67628,"inlinee":70707,"source_line":290,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":67908,"parent_record_offset":67868,"ancestor_record_offset":67628,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1673,"record_offset":67932,"parent_record_offset":67908,"ancestor_record_offset":67628,"inlinee":62297,"source_line":2061,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B2A
// 首条位置: {"module":1628,"inlinee":11050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x627B
// 首条位置: {"module":435,"inlinee":25211}
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
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C7A
// 首条位置: {"module":555,"record_offset":50484,"rva":11041808,"va":5379750928}
// 编译实例 RVAs: 0xa87c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3143264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379750928,"raw_hex":"e860f62f00","size":5,"soft_fail":false,"target_va":5382894288,"va":5379751019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$9::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17E66
// 首条位置: {"module":2599,"inlinee":97894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2599,"record_offset":22472,"parent_record_offset":22440,"ancestor_record_offset":22440,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2599,"record_offset":22528,"parent_record_offset":22496,"ancestor_record_offset":22440,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$6::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":2599,"record_offset":22588,"parent_record_offset":22564,"ancestor_record_offset":22440,"inlinee":97898,"source_line":16,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2599,"record_offset":22496,"parent_record_offset":22472,"ancestor_record_offset":22440,"inlinee":97896,"source_line":114,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 82, 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D54
// 首条位置: {"module":498,"inlinee":27988}
// 编译实例 RVAs: 0x11ff2a0
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
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7364
// 首条位置: {"module":515,"inlinee":29540}
// 编译实例 RVAs: 0x11ff2b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5666286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580080,"raw_hex":"e8128aa9ff","size":5,"soft_fail":false,"target_va":5381913840,"va":5387580121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5662172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580080,"raw_hex":"e8249aa9ff","size":5,"soft_fail":false,"target_va":5381917936,"va":5387580103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1412,"record_offset":7432,"rva":18871024,"va":5387580144}
// 编译实例 RVAs: 0x11ff2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5666068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580144,"raw_hex":"e8ec8aa9ff","size":5,"soft_fail":false,"target_va":5381914128,"va":5387580191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5660962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580144,"raw_hex":"e8de9ea9ff","size":5,"soft_fail":false,"target_va":5381919216,"va":5387580173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=16
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7417
// 首条位置: {"module":519,"inlinee":29719}
// 编译实例 RVAs: 0xad2950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t1857419","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380057424,"raw_hex":"e98b571c00","size":5,"soft_fail":false,"target_va":5381914864,"va":5380057440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1412,"record_offset":7736,"rva":18871088,"va":5387580208}
// 编译实例 RVAs: 0x11ff330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5665548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580208,"raw_hex":"e8f48ca9ff","size":5,"soft_fail":false,"target_va":5381914864,"va":5387580407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5668566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580208,"raw_hex":"e82a81a9ff","size":5,"soft_fail":false,"target_va":5381911968,"va":5387580529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5655869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580208,"raw_hex":"e8c3b2a9ff","size":5,"soft_fail":false,"target_va":5381924400,"va":5387580264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5659999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387580208,"raw_hex":"e8a1a2a9ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5387580250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-5665659","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387580208,"raw_hex":"e9858ca9ff","size":5,"soft_fail":false,"target_va":5381914864,"va":5387580518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-5655980","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387580208,"raw_hex":"e954b2a9ff","size":5,"soft_fail":false,"target_va":5381924400,"va":5387580375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1412,"record_offset":7892,"rva":18871808,"va":5387580928}
// 编译实例 RVAs: 0x11ff600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-5668149","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387580928,"raw_hex":"e9cb82a9ff","size":5,"soft_fail":false,"target_va":5381912800,"va":5387580944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1412,"record_offset":8048,"rva":18871840,"va":5387580960}
// 编译实例 RVAs: 0x11ff620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-5669013","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387580960,"raw_hex":"e96b7fa9ff","size":5,"soft_fail":false,"target_va":5381911968,"va":5387580976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7365
// 首条位置: {"module":515,"inlinee":29541}
// 编译实例 RVAs: 0x11ff640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-5668069","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387580992,"raw_hex":"e91b83a9ff","size":5,"soft_fail":false,"target_va":5381912944,"va":5387581008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1412,"record_offset":8348,"rva":18871904,"va":5387581024}
// 编译实例 RVAs: 0x11ff660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::circle::Circle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-5668741","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387581024,"raw_hex":"e97b80a9ff","size":5,"soft_fail":false,"target_va":5381912304,"va":5387581040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::expand
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D14
// 首条位置: {"module":497,"inlinee":27924}
// 编译实例 RVAs: 0x11ff680
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
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11430
// 首条位置: {"module":1412,"record_offset":8660,"rva":18872048,"va":5387581168}
// 编译实例 RVAs: 0x11ff6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23960660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581168,"raw_hex":"e8549c6d01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387581239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1412,"record_offset":8816,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01412.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7ECD
// 首条位置: {"module":567,"inlinee":32461}
// 编译实例 RVAs: 0x11ff740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23960580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581248,"raw_hex":"e8049c6d01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387581319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":567,"record_offset":64884,"parent_record_offset":64852,"ancestor_record_offset":64852,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11811
// 首条位置: {"module":1532,"inlinee":71697}
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
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11812
// 首条位置: {"module":1532,"inlinee":71698}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":16472,"parent_record_offset":16444,"ancestor_record_offset":16444,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":16500,"parent_record_offset":16472,"ancestor_record_offset":16444,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":16528,"parent_record_offset":16500,"ancestor_record_offset":16444,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":16556,"parent_record_offset":16528,"ancestor_record_offset":16444,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11437
// 首条位置: {"module":1673,"inlinee":70711}
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
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1180F
// 首条位置: {"module":1532,"inlinee":71695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":17408,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":17612,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":17644,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":15896,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16804,"parent_record_offset":15828,"ancestor_record_offset":15828,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1532,"record_offset":16500,"parent_record_offset":16472,"ancestor_record_offset":15828,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":17176,"parent_record_offset":17092,"ancestor_record_offset":15828,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":17144,"parent_record_offset":17120,"ancestor_record_offset":15828,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":17120,"parent_record_offset":17092,"ancestor_record_offset":15828,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":16080,"parent_record_offset":16048,"ancestor_record_offset":15828,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":16048,"parent_record_offset":16016,"ancestor_record_offset":15828,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":16112,"parent_record_offset":16080,"ancestor_record_offset":15828,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":16528,"parent_record_offset":16500,"ancestor_record_offset":15828,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1532,"record_offset":16556,"parent_record_offset":16528,"ancestor_record_offset":15828,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1532,"record_offset":16472,"parent_record_offset":16444,"ancestor_record_offset":15828,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":16332,"parent_record_offset":16220,"ancestor_record_offset":15828,"inlinee":71697,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":16444,"parent_record_offset":16416,"ancestor_record_offset":15828,"inlinee":71698,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":16300,"parent_record_offset":16276,"ancestor_record_offset":15828,"inlinee":71675,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1532,"record_offset":16192,"parent_record_offset":16160,"ancestor_record_offset":15828,"inlinee":71673,"source_line":275,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1532,"record_offset":17568,"parent_record_offset":17544,"ancestor_record_offset":15828,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":17020,"parent_record_offset":16984,"ancestor_record_offset":15828,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":16984,"parent_record_offset":16948,"ancestor_record_offset":15828,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":16220,"parent_record_offset":16192,"ancestor_record_offset":15828,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":15828,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":17804,"parent_record_offset":17780,"ancestor_record_offset":15828,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1532,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":15828,"inlinee":6648,"source_line":126,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":15936,"parent_record_offset":15896,"ancestor_record_offset":15828,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16840,"parent_record_offset":16804,"ancestor_record_offset":15828,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16360,"parent_record_offset":16220,"ancestor_record_offset":15828,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16416,"parent_record_offset":16220,"ancestor_record_offset":15828,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16276,"parent_record_offset":16220,"ancestor_record_offset":15828,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":16160,"parent_record_offset":15976,"ancestor_record_offset":15828,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":16948,"parent_record_offset":16912,"ancestor_record_offset":15828,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17092,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17324,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":11516,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17352,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17268,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17296,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17208,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":17236,"parent_record_offset":17020,"ancestor_record_offset":15828,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":16016,"parent_record_offset":15976,"ancestor_record_offset":15828,"inlinee":6979,"source_line":1146,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":15976,"parent_record_offset":15936,"ancestor_record_offset":15828,"inlinee":7004,"source_line":1210,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":16876,"parent_record_offset":16840,"ancestor_record_offset":15828,"inlinee":7006,"source_line":1223,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11810
// 首条位置: {"module":1532,"inlinee":71696}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":18040,"parent_record_offset":17996,"ancestor_record_offset":17996,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1532,"record_offset":18828,"parent_record_offset":18744,"ancestor_record_offset":17996,"inlinee":6570,"source_line":1505,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":18796,"parent_record_offset":18772,"ancestor_record_offset":17996,"inlinee":6857,"source_line":716,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1532,"record_offset":18772,"parent_record_offset":18744,"ancestor_record_offset":17996,"inlinee":6856,"source_line":682,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":18148,"parent_record_offset":18124,"ancestor_record_offset":17996,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1532,"record_offset":18124,"parent_record_offset":18096,"ancestor_record_offset":17996,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1532,"record_offset":18172,"parent_record_offset":18148,"ancestor_record_offset":17996,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1532,"record_offset":18264,"parent_record_offset":18096,"ancestor_record_offset":17996,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/features/impl_serde.rs
//     evidence={"module":1532,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":17996,"inlinee":29596,"source_line":18,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":18636,"parent_record_offset":18596,"ancestor_record_offset":17996,"inlinee":71685,"source_line":1665,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1532,"record_offset":18596,"parent_record_offset":18556,"ancestor_record_offset":17996,"inlinee":71684,"source_line":1826,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":18068,"parent_record_offset":18040,"ancestor_record_offset":17996,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1532,"record_offset":18208,"parent_record_offset":18096,"ancestor_record_offset":17996,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18556,"parent_record_offset":18516,"ancestor_record_offset":17996,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18744,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":6569,"source_line":227,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18976,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":11521,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18920,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":11519,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18716,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":11517,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18948,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":11520,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18860,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":11518,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1532,"record_offset":18888,"parent_record_offset":18636,"ancestor_record_offset":17996,"inlinee":6568,"source_line":193,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1532,"record_offset":18096,"parent_record_offset":18068,"ancestor_record_offset":17996,"inlinee":7008,"source_line":963,"archive":"_pdb/B/modules/01532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::valid_enclosure::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11433
// 首条位置: {"module":1673,"inlinee":70707}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1673,"record_offset":67556,"parent_record_offset":67500,"ancestor_record_offset":67500,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01673.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>", "constructor": "enum2$", "arguments": ["country_core::systems::wall::valid_enclosure::IsEnclosed"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::valid_enclosure::ShapeParameters
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 18; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>", "constructor": "enum2$", "arguments": ["country_core::systems::wall::valid_enclosure::ShapeParameters"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/B-platform-geometry/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::wall::valid_enclosure::ShapeParameters", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::wall::valid_enclosure::ShapeParameters", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_circle
// 信息 ID: 27b84102ef76dd58c38e7296
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x1eacbb3 -> 0x11ff6f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=51392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=53612; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
// 信息 ID: fae5b1911091aaa18c53fb6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x776116 -> 0x11ff2b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7761c9 -> 0x11ff2b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95c028 -> 0x11ff2b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1be0b19 -> 0x11ff2b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x234a779 -> 0x11ff2b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=24948; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=117964; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
// 信息 ID: 1771f06df9c47ff444d88882
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c83c0 -> 0x11ff2f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c89e1 -> 0x11ff2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
// 信息 ID: 07c809bbb258e3f16323e634
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758777 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x7ca834 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0x7caa33 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::is_half_dormer @ 0x8144ed -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x815d64 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217e59 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a6ff -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121a85e -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121c499 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121c4b3 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x127abec -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x127c850 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x1fe6cf4 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x2001460 -> 0x11ff740 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=64852; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::center
// 信息 ID: e66004a85795f6bca0aa1843
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75c1a2 -> 0x11ff2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x766711 -> 0x11ff2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf92208 -> 0x11ff2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x21ab448 -> 0x11ff2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_beam_count_change_audio::timberframe_beam_count_change_audio @ 0x21b66e5 -> 0x11ff2a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0; module=498, inline_record=20168; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::pivot_3d; module=498, inline_record=22464; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=41092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=117492; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::systems::nani_update_stone_floor; module=3098, inline_record=144920; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
// 信息 ID: f22b0f087007251063ac28de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79fbf0 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a7058 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac5e6 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf921e9 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x1248f92 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0x124b214 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124bd0d -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x124d0fa -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x124fe9d -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251b05 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254db2 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::ui_place_tree @ 0x226931c -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_move_tree::ui_move_tree @ 0x226b92c -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings @ 0x231b697 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings @ 0x231fca7 -> 0xad2950 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls; module=519, inline_record=37276; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=524, inline_record=55544; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_pruner::ivy_pruner; module=524, inline_record=55580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_continue_wall::ui_continue_wall; module=563, inline_record=41616; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_continue_wall::ui_continue_wall; module=563, inline_record=42088; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=563, inline_record=51408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=563, inline_record=53920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::ui_draw_wall::ui_draw_wall; module=563, inline_record=54368; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape
// 信息 ID: ac8135ff1fc5c824f7253d55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd2fbe -> 0x11ff330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd313d -> 0x11ff330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::expand
// 信息 ID: 48caa012d6ffa53d206a9fe0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x7cc69f -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf9218f -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128ac7c -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128ad09 -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e343 -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e3aa -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd2f2d -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd2f50 -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd2f93 -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd3112 -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x1ce4444 -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x1ce445b -> 0x11ff680 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=497, inline_record=63352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=22680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=36664; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
// 信息 ID: 596ad154d288f7b8a8cb6c15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c8336 -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128a60f -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128a62e -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128ad21 -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128ad3d -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128de7f -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128df4c -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e3c6 -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e3e9 -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x1ce4bb8 -> 0x11ff660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x1ce4bdc -> 0x11ff660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
// 信息 ID: 98b5d98f072c96b779205998
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121b277 -> 0x11ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121b747 -> 0x11ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251a36 -> 0x11ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254cdc -> 0x11ff600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> @ 0x1fd1fda -> 0x11ff600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::get_pos_at_u
// 信息 ID: 7c03a0471bd1e5bda6a384bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128a641 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128a658 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128b1f9 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128b279 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128b2f8 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128b370 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128de96 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128df60 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e984 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e9ec -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ea84 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128eaec -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ec29 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ec62 -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ec9b -> 0x11ff620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ecff -> 0x11ff620 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
// 信息 ID: 9b42302f9852e3d0c0f6768d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x74fb59 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c836f -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0x7c8990 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline @ 0x95c0a0 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x127ac57 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x127c8cb -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh @ 0x128a3d7 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e0fa -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e3f8 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128e407 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_glade_border::display_glade_border::display_glade_border @ 0x1be0d8c -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd2fd2 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd300f -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd3151 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x1cd318a -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x1ce4c02 -> 0x11ff640 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=25648; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize
// 信息 ID: c5fa96d9c513962293990792
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=57488; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=57756; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=58284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >; module=1673, inline_record=59212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=66092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=66360; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=66888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=1673, inline_record=67868; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 8effa5b7a8ebd1632616abb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1673, inline_record=624; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__FieldVisitor>; module=1673, inline_record=844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes
// 信息 ID: 0433e9497008a32bd9bc1504
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=16332; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=16384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: 819d07ae029550f874bd89f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: bfe1c1c19a8872573b0c4db8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=16444; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=16612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 899428b2d5b82ba06324ff31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 570cda42ba5eb1d92381eaa1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=668; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=892; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=1564; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=1788; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=2460; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=2684; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=4264; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=4488; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 0a6ed6be1205830a315b20f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=560; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=1456; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=2352; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<serde_json::error::Error,core::marker::PhantomData<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Field> >; module=1532, inline_record=4156; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=16192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: eb63e58943a522bf63b685ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1673, inline_record=1512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 18170d267b23304543c6043a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=6612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes
// 信息 ID: 376214e7ba5eb51b31d9b1e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29232; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29284; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31192; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 31ab8154ad023dd1eaf13f40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29344; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29512; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31304; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 信息 ID: 44069afc484a3ac56439fc8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29176; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29204; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31136; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: f0882ae4a4900234ad5e8c96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=29056; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=31016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 信息 ID: c3aa40a4bad713fcb9d30a1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1673, inline_record=1080; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1673, inline_record=2340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 9e70ab19ac9169f2d1c076ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=28812; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=30772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: ec47cbd9f169fbb62c0b33f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=30192; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1532, inline_record=32152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >
// 信息 ID: 0031faf5aa9d6b2f29a6f2f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::__private::de::content::EnumRefDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1282d53 -> 0x16ec140 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1282e93 -> 0x16ecda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor> @ 0x1283033 -> 0x16ed020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: a8ee5e93654ce7a4cba3ba0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=15828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: b3f39dd957f300e39cb81c7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::__Visitor>; module=1532, inline_record=17996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 114fd19c397ffd04f6f286f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x15b264b -> 0x16ebaf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15ea190 -> 0x16ebaf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15ea219 -> 0x16ebaf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::roofs::roof_shape::_::impl$0::deserialize::__Visitor> @ 0x15a9445 -> 0x16ec620 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15dc730 -> 0x16ec620 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15dc7b9 -> 0x16ec620 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e3422 -> 0x16ec620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >
// 信息 ID: 321a032f40d4918d70b64a27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::__private::de::content::ContentRefDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1284cfd -> 0x16eca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1284d3a -> 0x16eca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x128500d -> 0x16eca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x128504a -> 0x16eca30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >
// 信息 ID: dfccaddd28a37ab0b4b8ae27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::__private::de::missing_field::MissingFieldDeserializer -> ["serde_json::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize -> ["serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1284da3 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$26::struct_variant<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x12850b3 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::__Visitor> @ 0x15bba6b -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::roofs::roof_shape::_::impl$0::deserialize::__Visitor> @ 0x15c5a73 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15dc890 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15dc930 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15e356e -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15ea2f0 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15ea390 -> 0x16ebe70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize
// 信息 ID: 5a4aad1d9b8cdd5ddb0f85e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1628, inline_record=14556; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1628, inline_record=176832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: f8e40db40f80bde4590fe856
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x4bcb98 -> 0x259300 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> > @ 0x4b9168 -> 0x259550 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x1583308 -> 0x16eb8a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> > @ 0x15867d8 -> 0x16ec2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 8c73f775bf23132371e78cd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::wall::valid_enclosure::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> > @ 0x1593ffe -> 0x16eb640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$0::from
// 信息 ID: 408ff4cfdcc90899daacf4d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=58880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$2::from
// 信息 ID: 209ad08d9fd024636fc348a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=67500; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> >; module=1673, inline_record=69132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$3::clone
// 信息 ID: 652a11af3a10451161ec8388
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 33 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::deferred_wall_history::history_record_merge_walls; module=497, inline_record=36404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall; module=508, inline_record=80612; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=100932; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=102832; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=104440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=109096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=508, inline_record=110708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls; module=519, inline_record=41580; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$4::fmt
// 信息 ID: e91a7319df5f6bee038f9352
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$5::clone
// 信息 ID: dac3fa813d15664c1cb0a6bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=435, inline_record=25268; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::cloned::impl$1::next<bevy_ecs::query::iter::QueryIter<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,country_core::resources::roofs::roof_shape::Roof>; module=572, inline_record=3412; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::component::component_clone_via_clone<country_core::resources::roofs::roof_shape::Roof>; module=1584, inline_record=98440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$6::fmt
// 信息 ID: 35cf26f8701162ad309103d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=1673, inline_record=3484; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >; module=4342, inline_record=440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::valid_enclosure::impl$9::eq
// 信息 ID: 1a9e1d90104c7600cf317354
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2599, inline_record=22440; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3465, inline_record=4172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
