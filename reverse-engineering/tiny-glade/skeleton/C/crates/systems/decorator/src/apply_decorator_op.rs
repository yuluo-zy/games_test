// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/apply_decorator_op.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a39e8ac4925d7a55ce55f9c0d11c0bcce80c4081f0c20f03ef79023c8c81fd37
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_decorator_op.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1571
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
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// 首条位置: {"module":1571,"record_offset":272,"rva":21826608,"va":5390535728}
// 编译实例 RVAs: 0x14d0c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c0fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-64981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e82b02ffff","size":5,"soft_fail":false,"target_va":5390471104,"va":5390536080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414dccb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t49026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e882bf0000","size":5,"soft_fail":false,"target_va":5390585008,"va":5390535977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25814374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e866e58901","size":5,"soft_fail":false,"target_va":5416350384,"va":5390536005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: apply_decorator_op.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26520933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e865ad9401","size":5,"soft_fail":false,"target_va":5417057056,"va":5390536118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26084406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e836048e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390537189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26085026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e8a2068e01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390537177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorLocator::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12408922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e8a6a742ff","size":5,"soft_fail":false,"target_va":5378127152,"va":5390536069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12409041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e82fa742ff","size":5,"soft_fail":false,"target_va":5378128096,"va":5390537132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_change
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11033318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e81aa557ff","size":5,"soft_fail":false,"target_va":5379503664,"va":5390536977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_place
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11037663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e8219457ff","size":5,"soft_fail":false,"target_va":5379499152,"va":5390536810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11031792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390535728,"raw_hex":"e810ab57ff","size":5,"soft_fail":false,"target_va":5379504768,"va":5390536555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1571,"record_offset":868,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1571,"record_offset":432,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$65::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1571,"record_offset":1612,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26911,"source_line":277,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_change_op_history
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1571,"record_offset":900,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":80484,"source_line":166,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_place_op_history
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1571,"record_offset":468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":80483,"source_line":156,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::apply_decorator_op::write_remove_op_history
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/apply_decorator_op.rs
//     evidence={"module":1571,"record_offset":1340,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":80485,"source_line":182,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":580,"parent_record_offset":544,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1571,"record_offset":1184,"parent_record_offset":900,"ancestor_record_offset":272,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1571,"record_offset":1024,"parent_record_offset":984,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1571,"record_offset":700,"parent_record_offset":544,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$120::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1571,"record_offset":1684,"parent_record_offset":1612,"ancestor_record_offset":272,"inlinee":26915,"source_line":403,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$130::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1571,"record_offset":1656,"parent_record_offset":1612,"ancestor_record_offset":272,"inlinee":26913,"source_line":570,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1571,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":272,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1571,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":272,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":496,"parent_record_offset":468,"ancestor_record_offset":272,"inlinee":27782,"source_line":34,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":740,"parent_record_offset":468,"ancestor_record_offset":272,"inlinee":27784,"source_line":48,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_place_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A63
// 首条位置: {"module":1571,"inlinee":80483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":496,"parent_record_offset":468,"ancestor_record_offset":468,"inlinee":27782,"source_line":34,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":740,"parent_record_offset":468,"ancestor_record_offset":468,"inlinee":27784,"source_line":48,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":468,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":468,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":468,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":580,"parent_record_offset":544,"ancestor_record_offset":468,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":468,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":468,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1571,"record_offset":700,"parent_record_offset":544,"ancestor_record_offset":468,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1571,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":468,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1571,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":468,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_change_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A64
// 首条位置: {"module":1571,"inlinee":80484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1571,"record_offset":1184,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":936,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":27782,"source_line":34,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add_potentially_surviving_affected_decorator
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":1212,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":27784,"source_line":48,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":1056,"parent_record_offset":1024,"ancestor_record_offset":900,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":900,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":900,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":900,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":900,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1571,"record_offset":1024,"parent_record_offset":984,"ancestor_record_offset":900,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1571,"record_offset":1144,"parent_record_offset":984,"ancestor_record_offset":900,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1571,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":900,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1571,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":900,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::apply_decorator_op::write_remove_op_history
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A65
// 首条位置: {"module":1571,"inlinee":80485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":1571,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":1340,"inlinee":27782,"source_line":34,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":1480,"parent_record_offset":1448,"ancestor_record_offset":1340,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":1340,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1571,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":1340,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":1448,"parent_record_offset":1412,"ancestor_record_offset":1340,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":1340,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1571,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":1340,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1571,"record_offset":1568,"parent_record_offset":1412,"ancestor_record_offset":1340,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01571.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::apply_decorator_op
// 信息 ID: da64dda00bc032517dcc9823
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 30 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85bf38 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85d961 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85aee3 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85a38c -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x862ed3 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x86385e -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator::closure$3 @ 0x863b03 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864b6b -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864e45 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x866d07 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x865ed6 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x86653a -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x86830d -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x868c28 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_stair_height::ui_change_stair_height @ 0x86a366 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_chimney_height::ui_change_chimney_height @ 0x86aee2 -> 0x14d0c30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_change_op_history
// 信息 ID: 8ed7a23c265215bd251cf67c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1571, inline_record=900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_place_op_history
// 信息 ID: ecd8e2c734431fef28a0b592
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1571, inline_record=468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::apply_decorator_op::write_remove_op_history
// 信息 ID: e99402cefb0bd41a34496857
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::apply_decorator_op::apply_decorator_op; module=1571, inline_record=1340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
