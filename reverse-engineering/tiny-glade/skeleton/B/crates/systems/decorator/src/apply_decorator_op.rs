// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/apply_decorator_op.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a39e8ac4925d7a55ce55f9c0d11c0bcce80c4081f0c20f03ef79023c8c81fd37
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_decorator_op.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1513
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::apply_decorator_op
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1513,"record_offset":264,"rva":19361328,"va":5388070448}
// 编译实例 RVAs: 0x1276e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141267870
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-63261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e8e308ffff","size":5,"soft_fail":false,"target_va":5388007536,"va":5388070792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412b4a40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t252709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e825db0300","size":5,"soft_fail":false,"target_va":5388323392,"va":5388070678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13714196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e8ecbc2eff","size":5,"soft_fail":false,"target_va":5374356528,"va":5388070719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23469885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e83d1f6601","size":5,"soft_fail":false,"target_va":5411541776,"va":5388071886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23470185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e869206601","size":5,"soft_fail":false,"target_va":5411542064,"va":5388071874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorLocator::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10763586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e8bec25bff","size":5,"soft_fail":false,"target_va":5377307200,"va":5388070781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10763498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e816c35bff","size":5,"soft_fail":false,"target_va":5377308336,"va":5388071829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_change
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8743555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e87d957aff","size":5,"soft_fail":false,"target_va":5379328144,"va":5388071694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_place
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8742384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e8109a7aff","size":5,"soft_fail":false,"target_va":5379329136,"va":5388071515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8743735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388070448,"raw_hex":"e8c9947aff","size":5,"soft_fail":false,"target_va":5379327600,"va":5388071330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1513,"record_offset":836,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1513,"record_offset":424,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$58::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1513,"record_offset":1524,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25001,"source_line":277,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_change_op_history
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1513,"record_offset":868,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":71483,"source_line":166,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_place_op_history
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1513,"record_offset":460,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":71482,"source_line":156,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_remove_op_history
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1513,"record_offset":1280,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":71484,"source_line":182,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":548,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":264,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1513,"record_offset":1124,"parent_record_offset":868,"ancestor_record_offset":264,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1513,"record_offset":968,"parent_record_offset":928,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1513,"record_offset":668,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1513,"record_offset":1560,"parent_record_offset":1524,"ancestor_record_offset":264,"inlinee":25003,"source_line":351,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1513,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":264,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1513,"record_offset":736,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":264,"inlinee":25703,"source_line":34,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":704,"parent_record_offset":460,"ancestor_record_offset":264,"inlinee":25705,"source_line":48,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_place_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1173A
// 首条位置: {"module":1513,"inlinee":71482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":460,"inlinee":25703,"source_line":34,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":704,"parent_record_offset":460,"ancestor_record_offset":460,"inlinee":25705,"source_line":48,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":460,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":460,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":460,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":548,"parent_record_offset":512,"ancestor_record_offset":460,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":460,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1513,"record_offset":668,"parent_record_offset":512,"ancestor_record_offset":460,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1513,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":460,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1513,"record_offset":736,"parent_record_offset":704,"ancestor_record_offset":460,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_change_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1173B
// 首条位置: {"module":1513,"inlinee":71483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1513,"record_offset":1124,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":904,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":25703,"source_line":34,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":1152,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":25705,"source_line":48,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":868,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":868,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":868,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":868,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1513,"record_offset":968,"parent_record_offset":928,"ancestor_record_offset":868,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1513,"record_offset":1088,"parent_record_offset":928,"ancestor_record_offset":868,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1513,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":868,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1513,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":868,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_remove_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1173C
// 首条位置: {"module":1513,"inlinee":71484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1513,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":1280,"inlinee":25703,"source_line":34,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":1396,"parent_record_offset":1364,"ancestor_record_offset":1280,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":1280,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1513,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":1280,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":1364,"parent_record_offset":1328,"ancestor_record_offset":1280,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1513,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":1280,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1513,"record_offset":1484,"parent_record_offset":1328,"ancestor_record_offset":1280,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01513.jsonl.gz"}
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
// 完整结构及原始类型记录: apply_decorator_op.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: apply_decorator_op.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: apply_decorator_op.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::apply_decorator_op
// 信息 ID: 66c39dc6db1b8f758507d62d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 30 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x7a1ba7 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79ff26 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79f3f5 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x79e43d -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6eb8 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x7a7b3e -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x7a7de4 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x7a8b1a -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x7a8dfa -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ab50e -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ab678 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7aa54b -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7aab8f -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac832 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ad1b8 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_stair_height::ui_change_stair_height @ 0x7ae242 -> 0x1276e30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_change_op_history
// 信息 ID: b46045ddeda2c9e9b19c8e78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1513, inline_record=868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_place_op_history
// 信息 ID: 7cccda42800364bf9ae2f673
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1513, inline_record=460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_remove_op_history
// 信息 ID: 1370060914e00d437f2d1888
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1513, inline_record=1280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
