// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ui/ui_update.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2d0efe2ec3c8795c20ca1b94fd82ba9943c079e33f4d7c432e5641248199146e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 584
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::ui_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":584,"record_offset":52120,"rva":11722336,"va":5380431456}
// 编译实例 RVAs: 0xb2de60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5758282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8b622a8ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5380434613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c330
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t188468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e834e00200","size":5,"soft_fail":false,"target_va":5380621104,"va":5380432631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b79e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t306304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e880ac0400","size":5,"soft_fail":false,"target_va":5380742800,"va":5380436491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31234579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8139adc01","size":5,"soft_fail":false,"target_va":5411669968,"va":5380435384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t-6076340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e84c48a3ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5380432863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31104939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8ab9fda01","size":5,"soft_fail":false,"target_va":5411541584,"va":5380436640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31105619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e853a2da01","size":5,"soft_fail":false,"target_va":5411542296,"va":5380436672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8a2cfffff","size":5,"soft_fail":false,"target_va":5380419184,"va":5380431561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31111724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e82cbada01","size":5,"soft_fail":false,"target_va":5411543360,"va":5380431631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::ui_update::get_ui_volume
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8d9f8ffff","size":5,"soft_fail":false,"target_va":5380430944,"va":5380432770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31054538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8cadad901","size":5,"soft_fail":false,"target_va":5411488944,"va":5380434401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31055004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e89cdcd901","size":5,"soft_fail":false,"target_va":5411491664,"va":5380436655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1389041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8f1311500","size":5,"soft_fail":false,"target_va":5381823600,"va":5380434554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1389084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e81c321500","size":5,"soft_fail":false,"target_va":5381824016,"va":5380434927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t953449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8698c0e00","size":5,"soft_fail":false,"target_va":5381385536,"va":5380432082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t958638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8aea00e00","size":5,"soft_fail":false,"target_va":5381390640,"va":5380431997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t949923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8a37e0e00","size":5,"soft_fail":false,"target_va":5381386480,"va":5380436552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t953179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e85b8b0e00","size":5,"soft_fail":false,"target_va":5381384704,"va":5380431520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8388451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e863ff7f00","size":5,"soft_fail":false,"target_va":5388825296,"va":5380436840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31100471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380431456,"raw_hex":"e8378eda01","size":5,"soft_fail":false,"target_va":5411537104,"va":5380436628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":584,"record_offset":60236,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":584,"record_offset":57768,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":584,"record_offset":55352,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":584,"record_offset":54788,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":584,"record_offset":61028,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":584,"record_offset":58944,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":57736,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":57652,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":60324,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":60352,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":57824,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":584,"record_offset":57708,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":584,"record_offset":60204,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":27169,"source_line":253,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_released
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":584,"record_offset":60176,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":28553,"source_line":263,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::pressed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":584,"record_offset":66472,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":27178,"source_line":240,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":584,"record_offset":62116,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":584,"record_offset":66444,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":30120,"source_line":603,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetVolumeShape::get_origin
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":584,"record_offset":65688,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":28901,"source_line":312,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::ui_update::enter_hover
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":584,"record_offset":58092,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":33293,"source_line":295,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":584,"record_offset":67956,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":25969,"source_line":507,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":584,"record_offset":61772,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":53804,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":52352,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":52820,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":584,"record_offset":68012,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":584,"record_offset":62552,"parent_record_offset":52120,"ancestor_record_offset":52120,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":584,"record_offset":56656,"parent_record_offset":56632,"ancestor_record_offset":52120,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":584,"record_offset":56704,"parent_record_offset":56680,"ancestor_record_offset":52120,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":584,"record_offset":56680,"parent_record_offset":56656,"ancestor_record_offset":52120,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":584,"record_offset":57896,"parent_record_offset":57872,"ancestor_record_offset":52120,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":584,"record_offset":57872,"parent_record_offset":57848,"ancestor_record_offset":52120,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":58300,"parent_record_offset":58256,"ancestor_record_offset":52120,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":56632,"parent_record_offset":56604,"ancestor_record_offset":52120,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":55024,"parent_record_offset":55000,"ancestor_record_offset":52120,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":58256,"parent_record_offset":58220,"ancestor_record_offset":52120,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":65728,"parent_record_offset":65688,"ancestor_record_offset":52120,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":55000,"parent_record_offset":54900,"ancestor_record_offset":52120,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":584,"record_offset":57792,"parent_record_offset":57768,"ancestor_record_offset":52120,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":584,"record_offset":58156,"parent_record_offset":58132,"ancestor_record_offset":52120,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":584,"record_offset":66320,"parent_record_offset":66296,"ancestor_record_offset":52120,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":584,"record_offset":58220,"parent_record_offset":58156,"ancestor_record_offset":52120,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":584,"record_offset":54868,"parent_record_offset":54820,"ancestor_record_offset":52120,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":584,"record_offset":54900,"parent_record_offset":54868,"ancestor_record_offset":52120,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":584,"record_offset":55384,"parent_record_offset":55352,"ancestor_record_offset":52120,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":584,"record_offset":58132,"parent_record_offset":58092,"ancestor_record_offset":52120,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":584,"record_offset":54820,"parent_record_offset":54788,"ancestor_record_offset":52120,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54140,"parent_record_offset":54112,"ancestor_record_offset":52120,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54192,"parent_record_offset":54168,"ancestor_record_offset":52120,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54168,"parent_record_offset":54140,"ancestor_record_offset":52120,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54088,"parent_record_offset":54060,"ancestor_record_offset":52120,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54112,"parent_record_offset":54088,"ancestor_record_offset":52120,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54284,"parent_record_offset":54112,"ancestor_record_offset":52120,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":584,"record_offset":54396,"parent_record_offset":54372,"ancestor_record_offset":52120,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":584,"record_offset":63284,"parent_record_offset":63260,"ancestor_record_offset":52120,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":584,"record_offset":62848,"parent_record_offset":62824,"ancestor_record_offset":52120,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":584,"record_offset":62932,"parent_record_offset":62908,"ancestor_record_offset":52120,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":584,"record_offset":62872,"parent_record_offset":62848,"ancestor_record_offset":52120,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":584,"record_offset":63352,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":584,"record_offset":55416,"parent_record_offset":55384,"ancestor_record_offset":52120,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":584,"record_offset":54216,"parent_record_offset":54192,"ancestor_record_offset":52120,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":584,"record_offset":61932,"parent_record_offset":61840,"ancestor_record_offset":52120,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":584,"record_offset":55128,"parent_record_offset":55104,"ancestor_record_offset":52120,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":584,"record_offset":54944,"parent_record_offset":54900,"ancestor_record_offset":52120,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":584,"record_offset":52512,"parent_record_offset":52480,"ancestor_record_offset":52120,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":584,"record_offset":63084,"parent_record_offset":63056,"ancestor_record_offset":52120,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":584,"record_offset":63056,"parent_record_offset":63028,"ancestor_record_offset":52120,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":584,"record_offset":65904,"parent_record_offset":65876,"ancestor_record_offset":52120,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":584,"record_offset":53956,"parent_record_offset":53912,"ancestor_record_offset":52120,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":55444,"parent_record_offset":55416,"ancestor_record_offset":52120,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":57584,"parent_record_offset":57468,"ancestor_record_offset":52120,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":55504,"parent_record_offset":55384,"ancestor_record_offset":52120,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":584,"record_offset":52768,"parent_record_offset":52740,"ancestor_record_offset":52120,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":584,"record_offset":54744,"parent_record_offset":53912,"ancestor_record_offset":52120,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":54348,"parent_record_offset":54060,"ancestor_record_offset":52120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":584,"record_offset":57268,"parent_record_offset":57244,"ancestor_record_offset":52120,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":584,"record_offset":63200,"parent_record_offset":63176,"ancestor_record_offset":52120,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":584,"record_offset":57928,"parent_record_offset":57896,"ancestor_record_offset":52120,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":584,"record_offset":53236,"parent_record_offset":53212,"ancestor_record_offset":52120,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":584,"record_offset":53212,"parent_record_offset":53188,"ancestor_record_offset":52120,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":584,"record_offset":53420,"parent_record_offset":53392,"ancestor_record_offset":52120,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":54240,"parent_record_offset":54216,"ancestor_record_offset":52120,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":584,"record_offset":55056,"parent_record_offset":54900,"ancestor_record_offset":52120,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":584,"record_offset":57216,"parent_record_offset":56440,"ancestor_record_offset":52120,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":584,"record_offset":65756,"parent_record_offset":65728,"ancestor_record_offset":52120,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":584,"record_offset":65784,"parent_record_offset":65756,"ancestor_record_offset":52120,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":584,"record_offset":55104,"parent_record_offset":55080,"ancestor_record_offset":52120,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":584,"record_offset":55080,"parent_record_offset":55056,"ancestor_record_offset":52120,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":584,"record_offset":57244,"parent_record_offset":57216,"ancestor_record_offset":52120,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":584,"record_offset":53164,"parent_record_offset":53140,"ancestor_record_offset":52120,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":584,"record_offset":53188,"parent_record_offset":53164,"ancestor_record_offset":52120,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":584,"record_offset":53492,"parent_record_offset":53464,"ancestor_record_offset":52120,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":584,"record_offset":53140,"parent_record_offset":53024,"ancestor_record_offset":52120,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":584,"record_offset":52480,"parent_record_offset":52452,"ancestor_record_offset":52120,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":584,"record_offset":52852,"parent_record_offset":52820,"ancestor_record_offset":52120,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":584,"record_offset":52992,"parent_record_offset":52960,"ancestor_record_offset":52120,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":584,"record_offset":52960,"parent_record_offset":52924,"ancestor_record_offset":52120,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":584,"record_offset":52900,"parent_record_offset":52876,"ancestor_record_offset":52120,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":584,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":52120,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":584,"record_offset":52876,"parent_record_offset":52852,"ancestor_record_offset":52120,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":584,"record_offset":53464,"parent_record_offset":52924,"ancestor_record_offset":52120,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52452,"parent_record_offset":52420,"ancestor_record_offset":52120,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52420,"parent_record_offset":52392,"ancestor_record_offset":52120,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52620,"parent_record_offset":52584,"ancestor_record_offset":52120,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52716,"parent_record_offset":52620,"ancestor_record_offset":52120,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52584,"parent_record_offset":52556,"ancestor_record_offset":52120,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52392,"parent_record_offset":52352,"ancestor_record_offset":52120,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":584,"record_offset":52740,"parent_record_offset":52716,"ancestor_record_offset":52120,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":584,"record_offset":53024,"parent_record_offset":52992,"ancestor_record_offset":52120,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":584,"record_offset":53308,"parent_record_offset":53024,"ancestor_record_offset":52120,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":584,"record_offset":53280,"parent_record_offset":53024,"ancestor_record_offset":52120,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":584,"record_offset":53084,"parent_record_offset":53024,"ancestor_record_offset":52120,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":61960,"parent_record_offset":61840,"ancestor_record_offset":52120,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":60264,"parent_record_offset":60236,"ancestor_record_offset":52120,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":58524,"parent_record_offset":58488,"ancestor_record_offset":52120,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":61876,"parent_record_offset":61840,"ancestor_record_offset":52120,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":60288,"parent_record_offset":60264,"ancestor_record_offset":52120,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":61984,"parent_record_offset":61960,"ancestor_record_offset":52120,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":61900,"parent_record_offset":61876,"ancestor_record_offset":52120,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":57956,"parent_record_offset":57896,"ancestor_record_offset":52120,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":58052,"parent_record_offset":58020,"ancestor_record_offset":52120,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":584,"record_offset":58968,"parent_record_offset":58944,"ancestor_record_offset":52120,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::ui_update::ui_update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":584,"record_offset":57676,"parent_record_offset":57652,"ancestor_record_offset":52120,"inlinee":33291,"source_line":45,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::ui_update::ui_update::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/ui_update.rs
//     evidence={"module":584,"record_offset":65852,"parent_record_offset":65828,"ancestor_record_offset":52120,"inlinee":33294,"source_line":225,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":584,"record_offset":65876,"parent_record_offset":65852,"ancestor_record_offset":52120,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":584,"record_offset":62992,"parent_record_offset":62964,"ancestor_record_offset":52120,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":584,"record_offset":63028,"parent_record_offset":62992,"ancestor_record_offset":52120,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":584,"record_offset":62964,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":62824,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":63380,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":62908,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":584,"record_offset":62796,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":584,"record_offset":63260,"parent_record_offset":63236,"ancestor_record_offset":52120,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":68036,"parent_record_offset":68012,"ancestor_record_offset":52120,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":62576,"parent_record_offset":62552,"ancestor_record_offset":52120,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":62600,"parent_record_offset":62576,"ancestor_record_offset":52120,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":63236,"parent_record_offset":63128,"ancestor_record_offset":52120,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":62632,"parent_record_offset":62600,"ancestor_record_offset":52120,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":63176,"parent_record_offset":63152,"ancestor_record_offset":52120,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":63152,"parent_record_offset":63128,"ancestor_record_offset":52120,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":63472,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":62684,"parent_record_offset":62660,"ancestor_record_offset":52120,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":62660,"parent_record_offset":62600,"ancestor_record_offset":52120,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":62712,"parent_record_offset":62684,"ancestor_record_offset":52120,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":62768,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":63128,"parent_record_offset":62712,"ancestor_record_offset":52120,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":584,"record_offset":63644,"parent_record_offset":63616,"ancestor_record_offset":52120,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":62020,"parent_record_offset":61800,"ancestor_record_offset":52120,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":63708,"parent_record_offset":63672,"ancestor_record_offset":52120,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":61840,"parent_record_offset":61800,"ancestor_record_offset":52120,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":61800,"parent_record_offset":61772,"ancestor_record_offset":52120,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":63672,"parent_record_offset":63644,"ancestor_record_offset":52120,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":584,"record_offset":62048,"parent_record_offset":62020,"ancestor_record_offset":52120,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":584,"record_offset":63736,"parent_record_offset":63708,"ancestor_record_offset":52120,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":53840,"parent_record_offset":53804,"ancestor_record_offset":52120,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":68132,"parent_record_offset":68100,"ancestor_record_offset":52120,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":68632,"parent_record_offset":68372,"ancestor_record_offset":52120,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":584,"record_offset":53984,"parent_record_offset":53956,"ancestor_record_offset":52120,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":54060,"parent_record_offset":53912,"ancestor_record_offset":52120,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":584,"record_offset":54320,"parent_record_offset":54060,"ancestor_record_offset":52120,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":584,"record_offset":54012,"parent_record_offset":53984,"ancestor_record_offset":52120,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":584,"record_offset":53912,"parent_record_offset":53876,"ancestor_record_offset":52120,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":584,"record_offset":53876,"parent_record_offset":53840,"ancestor_record_offset":52120,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::get_ui_volume
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":584,"record_offset":50048,"rva":11721824,"va":5380430944}
// 编译实例 RVAs: 0xb2dc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: ui_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5754749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380430944,"raw_hex":"e88330a8ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5380431080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31110438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380430944,"raw_hex":"e826b5da01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380431333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31057925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380430944,"raw_hex":"e805e8d901","size":5,"soft_fail":false,"target_va":5411488944,"va":5380431014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31060316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380430944,"raw_hex":"e85cf1d901","size":5,"soft_fail":false,"target_va":5411491664,"va":5380431343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1392560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380430944,"raw_hex":"e8b03f1500","size":5,"soft_fail":false,"target_va":5381823600,"va":5380431035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1392744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380430944,"raw_hex":"e868401500","size":5,"soft_fail":false,"target_va":5381824016,"va":5380431267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":584,"record_offset":51504,"parent_record_offset":50048,"ancestor_record_offset":50048,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":584,"record_offset":51840,"parent_record_offset":50048,"ancestor_record_offset":50048,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":51592,"parent_record_offset":50048,"ancestor_record_offset":50048,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":584,"record_offset":50200,"parent_record_offset":50048,"ancestor_record_offset":50048,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":584,"record_offset":50532,"parent_record_offset":50048,"ancestor_record_offset":50048,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":584,"record_offset":51260,"parent_record_offset":51236,"ancestor_record_offset":50048,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":584,"record_offset":50828,"parent_record_offset":50804,"ancestor_record_offset":50048,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":584,"record_offset":50912,"parent_record_offset":50888,"ancestor_record_offset":50048,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":584,"record_offset":50852,"parent_record_offset":50828,"ancestor_record_offset":50048,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":584,"record_offset":51328,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":584,"record_offset":50352,"parent_record_offset":50260,"ancestor_record_offset":50048,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":584,"record_offset":51060,"parent_record_offset":51032,"ancestor_record_offset":50048,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":584,"record_offset":51032,"parent_record_offset":51004,"ancestor_record_offset":50048,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":584,"record_offset":51176,"parent_record_offset":51152,"ancestor_record_offset":50048,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":50380,"parent_record_offset":50260,"ancestor_record_offset":50048,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":51532,"parent_record_offset":51504,"ancestor_record_offset":50048,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":51772,"parent_record_offset":51648,"ancestor_record_offset":50048,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":50296,"parent_record_offset":50260,"ancestor_record_offset":50048,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":51556,"parent_record_offset":51532,"ancestor_record_offset":50048,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":50404,"parent_record_offset":50380,"ancestor_record_offset":50048,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":50320,"parent_record_offset":50296,"ancestor_record_offset":50048,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":51796,"parent_record_offset":51772,"ancestor_record_offset":50048,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":584,"record_offset":51284,"parent_record_offset":51260,"ancestor_record_offset":50048,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":584,"record_offset":50972,"parent_record_offset":50944,"ancestor_record_offset":50048,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":584,"record_offset":51004,"parent_record_offset":50972,"ancestor_record_offset":50048,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":584,"record_offset":50944,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":50804,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":51356,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":584,"record_offset":50888,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":584,"record_offset":50776,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":584,"record_offset":51236,"parent_record_offset":51212,"ancestor_record_offset":50048,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":50556,"parent_record_offset":50532,"ancestor_record_offset":50048,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":50580,"parent_record_offset":50556,"ancestor_record_offset":50048,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":51212,"parent_record_offset":51104,"ancestor_record_offset":50048,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":584,"record_offset":50612,"parent_record_offset":50580,"ancestor_record_offset":50048,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":51152,"parent_record_offset":51128,"ancestor_record_offset":50048,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":51128,"parent_record_offset":51104,"ancestor_record_offset":50048,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":51448,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":50664,"parent_record_offset":50640,"ancestor_record_offset":50048,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":50640,"parent_record_offset":50580,"ancestor_record_offset":50048,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":50692,"parent_record_offset":50664,"ancestor_record_offset":50048,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":50748,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":584,"record_offset":51104,"parent_record_offset":50692,"ancestor_record_offset":50048,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":584,"record_offset":51620,"parent_record_offset":51592,"ancestor_record_offset":50048,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":50440,"parent_record_offset":50224,"ancestor_record_offset":50048,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":51684,"parent_record_offset":51648,"ancestor_record_offset":50048,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":50260,"parent_record_offset":50224,"ancestor_record_offset":50048,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":50224,"parent_record_offset":50200,"ancestor_record_offset":50048,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":584,"record_offset":51648,"parent_record_offset":51620,"ancestor_record_offset":50048,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":584,"record_offset":50464,"parent_record_offset":50440,"ancestor_record_offset":50048,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":584,"record_offset":51712,"parent_record_offset":51684,"ancestor_record_offset":50048,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::ui_update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x820B
// 首条位置: {"module":584,"inlinee":33291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::ui_update::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x820E
// 首条位置: {"module":584,"inlinee":33294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":584,"record_offset":65876,"parent_record_offset":65852,"ancestor_record_offset":65852,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":584,"record_offset":65904,"parent_record_offset":65876,"ancestor_record_offset":65852,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::ui_update::enter_hover
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 295
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x820D
// 首条位置: {"module":584,"inlinee":33293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":584,"record_offset":58132,"parent_record_offset":58092,"ancestor_record_offset":58092,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":58464,"parent_record_offset":58092,"ancestor_record_offset":58092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":584,"record_offset":58332,"parent_record_offset":58300,"ancestor_record_offset":58092,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":584,"record_offset":58380,"parent_record_offset":58356,"ancestor_record_offset":58092,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":584,"record_offset":58356,"parent_record_offset":58332,"ancestor_record_offset":58092,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":584,"record_offset":58488,"parent_record_offset":58464,"ancestor_record_offset":58092,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":58300,"parent_record_offset":58256,"ancestor_record_offset":58092,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":584,"record_offset":58256,"parent_record_offset":58220,"ancestor_record_offset":58092,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":584,"record_offset":58192,"parent_record_offset":58156,"ancestor_record_offset":58092,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":584,"record_offset":58156,"parent_record_offset":58132,"ancestor_record_offset":58092,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":584,"record_offset":58220,"parent_record_offset":58156,"ancestor_record_offset":58092,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":584,"record_offset":58420,"parent_record_offset":58256,"ancestor_record_offset":58092,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":58524,"parent_record_offset":58488,"ancestor_record_offset":58092,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":58548,"parent_record_offset":58524,"ancestor_record_offset":58092,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00584.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":584,"record_offset":58580,"parent_record_offset":58488,"ancestor_record_offset":58092,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00584.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::enter_hover
// 信息 ID: b320aa241a794a68d6bfd283
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=584, inline_record=58092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=584, inline_record=59324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=584, inline_record=64836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::get_ui_volume
// 信息 ID: 20d6f2360be48337e9c4c73b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::ui_update::ui_update @ 0xb2edd4 -> 0xb2dc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::ui_update::ui_update @ 0xb2e491 -> 0xb2dc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::ui_update::ui_update @ 0xb2e50a -> 0xb2dc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::ui_update::ui_update @ 0xb2e382 -> 0xb2dc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::ui_update
// 信息 ID: 1c2af54453b9173ef5f53b9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d5185a46ddb66fed39e574ed: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > > @ 0x14e0714 -> 0xb2de60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > @ 0x17545a2 -> 0xb2de60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::ui_update::closure$0
// 信息 ID: 3973ec46d50fc1ec736f200c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui::ui_update::ui_update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=584, inline_record=57676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::ui_update::ui_update::closure$2
// 信息 ID: c694840d8b0c42e16f1fcbf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui::ui_update::ui_update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::ui_update::ui_update; module=584, inline_record=65852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
