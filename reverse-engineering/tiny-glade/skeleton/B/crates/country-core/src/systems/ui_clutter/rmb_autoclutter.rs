// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ui_clutter/rmb_autoclutter.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3d5f4a5247ec2aef766b100b6037fd285e8849b30ec8ea485c16708b8feb4271
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rmb_autoclutter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 519
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":9432,"rva":9923744,"va":5378632864}
// 编译实例 RVAs: 0x976ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6999425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e87f3295ff","size":5,"soft_fail":false,"target_va":5371634416,"va":5378633836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3956619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e875a0c3ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378632950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3552561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e8cfcac9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378634044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33214242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e822cffa01","size":5,"soft_fail":false,"target_va":5411847712,"va":5378633465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33214204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e8fccefa01","size":5,"soft_fail":false,"target_va":5411847808,"va":5378633599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33214378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e8aacffa01","size":5,"soft_fail":false,"target_va":5411847952,"va":5378633569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rmb_autoclutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33214551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e857d0fa01","size":5,"soft_fail":false,"target_va":5411848144,"va":5378633588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32906840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e8581ef601","size":5,"soft_fail":false,"target_va":5411540992,"va":5378634147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2001198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e82e891e00","size":5,"soft_fail":false,"target_va":5380635120,"va":5378633917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32908257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e8e123f601","size":5,"soft_fail":false,"target_va":5411541776,"va":5378633514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32908658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e87225f601","size":5,"soft_fail":false,"target_va":5411542880,"va":5378634217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_meshes::ClutterMeshTy::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-781486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e85213f4ff","size":5,"soft_fail":false,"target_va":5377852368,"va":5378633849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1105221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e845dd1000","size":5,"soft_fail":false,"target_va":5379739328,"va":5378634102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::start_input_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1110341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e845f11000","size":5,"soft_fail":false,"target_va":5379744368,"va":5378634022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::as_f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14140109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378632864,"raw_hex":"e8cdc2d700","size":5,"soft_fail":false,"target_va":5392773648,"va":5378633534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":14492,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":519,"record_offset":14160,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":519,"record_offset":10684,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":10648,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":519,"record_offset":14708,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":519,"record_offset":13396,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":26065,"source_line":895,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":519,"record_offset":9624,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":27690,"source_line":73,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_clutter::ui_autoclutter::get_clutter_rot_in_roof_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_clutter/ui_autoclutter.rs
//     evidence={"module":519,"record_offset":12324,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":29703,"source_line":403,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":14104,"parent_record_offset":9432,"ancestor_record_offset":9432,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":9432,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":9432,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":9432,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":14580,"parent_record_offset":14548,"ancestor_record_offset":9432,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":14520,"parent_record_offset":14492,"ancestor_record_offset":9432,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":519,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":9432,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":519,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":9432,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":519,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":9432,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":14328,"parent_record_offset":14284,"ancestor_record_offset":9432,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":9432,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":10984,"parent_record_offset":10884,"ancestor_record_offset":9432,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":14284,"parent_record_offset":14248,"ancestor_record_offset":9432,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":9432,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":519,"record_offset":11184,"parent_record_offset":10852,"ancestor_record_offset":9432,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":519,"record_offset":14220,"parent_record_offset":14184,"ancestor_record_offset":9432,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":519,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":9432,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":519,"record_offset":10852,"parent_record_offset":10800,"ancestor_record_offset":9432,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":519,"record_offset":14248,"parent_record_offset":14184,"ancestor_record_offset":9432,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":519,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":9432,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":519,"record_offset":11656,"parent_record_offset":10800,"ancestor_record_offset":9432,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":519,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":9432,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":519,"record_offset":12148,"parent_record_offset":10800,"ancestor_record_offset":9432,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":519,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":9432,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":519,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":9432,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":519,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":9432,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":519,"record_offset":11756,"parent_record_offset":11732,"ancestor_record_offset":9432,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":519,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":9432,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":519,"record_offset":11732,"parent_record_offset":11704,"ancestor_record_offset":9432,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":519,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":9432,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":519,"record_offset":11396,"parent_record_offset":10800,"ancestor_record_offset":9432,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":519,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":9432,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":519,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":9432,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":519,"record_offset":10716,"parent_record_offset":10684,"ancestor_record_offset":9432,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":519,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":9432,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":519,"record_offset":10800,"parent_record_offset":10684,"ancestor_record_offset":9432,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":519,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":9432,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":519,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":9432,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":519,"record_offset":11244,"parent_record_offset":11216,"ancestor_record_offset":9432,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":519,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":9432,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":519,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":9432,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":519,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":9432,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":519,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":9432,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":519,"record_offset":12828,"parent_record_offset":12408,"ancestor_record_offset":9432,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":519,"record_offset":12736,"parent_record_offset":12680,"ancestor_record_offset":9432,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":519,"record_offset":12504,"parent_record_offset":12480,"ancestor_record_offset":9432,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":519,"record_offset":12536,"parent_record_offset":12452,"ancestor_record_offset":9432,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":519,"record_offset":12600,"parent_record_offset":12408,"ancestor_record_offset":9432,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":519,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":9432,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":519,"record_offset":10440,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":519,"record_offset":10172,"parent_record_offset":10144,"ancestor_record_offset":9432,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":519,"record_offset":10144,"parent_record_offset":10116,"ancestor_record_offset":9432,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":519,"record_offset":11128,"parent_record_offset":10852,"ancestor_record_offset":9432,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":519,"record_offset":11868,"parent_record_offset":11808,"ancestor_record_offset":9432,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":11892,"parent_record_offset":11868,"ancestor_record_offset":9432,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":10616,"parent_record_offset":9624,"ancestor_record_offset":9432,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":11932,"parent_record_offset":11756,"ancestor_record_offset":9432,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":519,"record_offset":11348,"parent_record_offset":11316,"ancestor_record_offset":9432,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":519,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":9432,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":9432,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":14448,"parent_record_offset":14284,"ancestor_record_offset":9432,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":519,"record_offset":12288,"parent_record_offset":10800,"ancestor_record_offset":9432,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":519,"record_offset":10884,"parent_record_offset":10852,"ancestor_record_offset":9432,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":519,"record_offset":10952,"parent_record_offset":10924,"ancestor_record_offset":9432,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":519,"record_offset":10924,"parent_record_offset":10884,"ancestor_record_offset":9432,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":519,"record_offset":13068,"parent_record_offset":12324,"ancestor_record_offset":9432,"inlinee":28044,"source_line":298,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":519,"record_offset":12360,"parent_record_offset":12324,"ancestor_record_offset":9432,"inlinee":26065,"source_line":895,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":519,"record_offset":13008,"parent_record_offset":12360,"ancestor_record_offset":9432,"inlinee":25974,"source_line":889,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::DecoratorRotationXZ::offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":519,"record_offset":13188,"parent_record_offset":12324,"ancestor_record_offset":9432,"inlinee":26068,"source_line":915,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":519,"record_offset":13216,"parent_record_offset":12324,"ancestor_record_offset":9432,"inlinee":26066,"source_line":900,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":519,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":9432,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":519,"record_offset":11216,"parent_record_offset":10800,"ancestor_record_offset":9432,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":519,"record_offset":11292,"parent_record_offset":11268,"ancestor_record_offset":9432,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":519,"record_offset":11316,"parent_record_offset":11292,"ancestor_record_offset":9432,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":519,"record_offset":11268,"parent_record_offset":11244,"ancestor_record_offset":9432,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":519,"record_offset":13268,"parent_record_offset":13240,"ancestor_record_offset":9432,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":519,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":9432,"inlinee":25694,"source_line":168,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":519,"record_offset":13356,"parent_record_offset":13240,"ancestor_record_offset":9432,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":519,"record_offset":12384,"parent_record_offset":12360,"ancestor_record_offset":9432,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":519,"record_offset":12408,"parent_record_offset":12384,"ancestor_record_offset":9432,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":519,"record_offset":12856,"parent_record_offset":12408,"ancestor_record_offset":9432,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":519,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":9432,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":519,"record_offset":12452,"parent_record_offset":12408,"ancestor_record_offset":9432,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":519,"record_offset":10084,"parent_record_offset":10056,"ancestor_record_offset":9432,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":519,"record_offset":10116,"parent_record_offset":10084,"ancestor_record_offset":9432,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":519,"record_offset":10056,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":519,"record_offset":9916,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":519,"record_offset":10468,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":519,"record_offset":10000,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":519,"record_offset":9888,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":519,"record_offset":10348,"parent_record_offset":10324,"ancestor_record_offset":9432,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":519,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":9432,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":519,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9432,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":519,"record_offset":10324,"parent_record_offset":10216,"ancestor_record_offset":9432,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":519,"record_offset":9728,"parent_record_offset":9696,"ancestor_record_offset":9432,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":9432,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":9432,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":10560,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":9432,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":9756,"parent_record_offset":9696,"ancestor_record_offset":9432,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":9808,"parent_record_offset":9780,"ancestor_record_offset":9432,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":9860,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":519,"record_offset":10216,"parent_record_offset":9808,"ancestor_record_offset":9432,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":519,"record_offset":14128,"parent_record_offset":14104,"ancestor_record_offset":9432,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":519,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":9432,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":519,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":9432,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":519,"record_offset":13032,"parent_record_offset":13008,"ancestor_record_offset":9432,"inlinee":14897,"source_line":830,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":519,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":9432,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":519,"record_offset":13292,"parent_record_offset":13268,"ancestor_record_offset":9432,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::yaw_radians
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":519,"record_offset":13156,"parent_record_offset":13068,"ancestor_record_offset":9432,"inlinee":27994,"source_line":53,"archive":"_pdb/B/modules/00519.jsonl.gz"}
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
// 完整结构及原始类型记录: rmb_autoclutter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rmb_autoclutter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rmb_autoclutter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter
// 信息 ID: cc18ef90080eafe201e8484a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0x99d8c0 -> 0x976ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
