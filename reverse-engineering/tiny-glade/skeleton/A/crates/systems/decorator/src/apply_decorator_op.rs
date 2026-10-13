// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/apply_decorator_op.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f02531f0890077eb0743340684fd1c8c61ef3801fed73f9a5362cdcbcf8e4b8d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_decorator_op.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1281
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::apply_decorator_op
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1281,"record_offset":256,"rva":13656448,"va":5382365568}
// 编译实例 RVAs: 0xd06180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404442d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9183486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e802df73ff","size":5,"soft_fail":false,"target_va":5373182672,"va":5382366153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfd8f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e81775ffff","size":5,"soft_fail":false,"target_va":5382330608,"va":5382366164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cffdb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e8569bffff","size":5,"soft_fail":false,"target_va":5382340016,"va":5382365781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-11060587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e8953a57ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5382365814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20612065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e8e1833a01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382366810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20612349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e8fd843a01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382366798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7795078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e87a0e89ff","size":5,"soft_fail":false,"target_va":5374571680,"va":5382366753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_change
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7643287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e8695f8bff","size":5,"soft_fail":false,"target_va":5374723152,"va":5382366434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_place
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7642325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e82b638bff","size":5,"soft_fail":false,"target_va":5374724304,"va":5382366624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_remove
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7643690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382365568,"raw_hex":"e8d65d8bff","size":5,"soft_fail":false,"target_va":5374722640,"va":5382366325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1281,"record_offset":776,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1281,"record_offset":412,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_change_op_history
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1281,"record_offset":808,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":60238,"source_line":159,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_place_op_history
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1281,"record_offset":448,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":60235,"source_line":149,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_remove_op_history
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1281,"record_offset":1160,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":60240,"source_line":175,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1281,"record_offset":576,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":544,"parent_record_offset":508,"ancestor_record_offset":256,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":508,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1281,"record_offset":1000,"parent_record_offset":808,"ancestor_record_offset":256,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1281,"record_offset":608,"parent_record_offset":508,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1281,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":256,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1281,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":256,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":480,"parent_record_offset":448,"ancestor_record_offset":256,"inlinee":60237,"source_line":41,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":644,"parent_record_offset":448,"ancestor_record_offset":256,"inlinee":60239,"source_line":60,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_place_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB4B
// 首条位置: {"module":1281,"inlinee":60235}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":480,"parent_record_offset":448,"ancestor_record_offset":448,"inlinee":60237,"source_line":41,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":644,"parent_record_offset":448,"ancestor_record_offset":448,"inlinee":60239,"source_line":60,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1281,"record_offset":576,"parent_record_offset":544,"ancestor_record_offset":448,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":544,"parent_record_offset":508,"ancestor_record_offset":448,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":508,"parent_record_offset":480,"ancestor_record_offset":448,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1281,"record_offset":608,"parent_record_offset":508,"ancestor_record_offset":448,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1281,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":448,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1281,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":448,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_change_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB4E
// 首条位置: {"module":1281,"inlinee":60238}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1281,"record_offset":1000,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":844,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":60237,"source_line":41,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":1028,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":60239,"source_line":60,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1281,"record_offset":932,"parent_record_offset":900,"ancestor_record_offset":808,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":900,"parent_record_offset":868,"ancestor_record_offset":808,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":808,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1281,"record_offset":964,"parent_record_offset":868,"ancestor_record_offset":808,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1281,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":808,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1281,"record_offset":1060,"parent_record_offset":1028,"ancestor_record_offset":808,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_remove_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB50
// 首条位置: {"module":1281,"inlinee":60240}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1281,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":60237,"source_line":41,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1281,"record_offset":1272,"parent_record_offset":1240,"ancestor_record_offset":1160,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":1240,"parent_record_offset":1208,"ancestor_record_offset":1160,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1281,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":1160,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1281,"record_offset":1304,"parent_record_offset":1208,"ancestor_record_offset":1160,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01281.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::apply_decorator_op
// 信息 ID: a0e9cc31a73f465f389b5e90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::cull_oob_decorators @ 0x5003c8 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::handle_decorator_deletion_cmds::handle_decorator_deletion_cmds @ 0x1c5db44 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6e6ce -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_lantern @ 0x1c74ee0 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x1c75867 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x1c7791e -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x1c78788 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x1c78975 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x1e68625 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x1e687c5 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x1e6894e -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e6a301 -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e6a4be -> 0xd06180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part @ 0x1e6a649 -> 0xd06180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_change_op_history
// 信息 ID: fd4e180c2b990ff4a8aee2dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1281, inline_record=808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_place_op_history
// 信息 ID: 7d76c2a0c35392a265ed3e00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1281, inline_record=448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_remove_op_history
// 信息 ID: 759310ff086d4a14aadb139f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1281, inline_record=1160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
