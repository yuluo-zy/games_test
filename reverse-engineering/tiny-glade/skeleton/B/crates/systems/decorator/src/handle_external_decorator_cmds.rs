// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/handle_external_decorator_cmds.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5396fce61a9776d1e9059e84fbb0cabda9347b4849e2955250c6dacaa08cda7f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_external_decorator_cmds.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 463
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
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":463,"record_offset":264,"rva":8496000,"va":5377205120}
// 编译实例 RVAs: 0x81a380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=211 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8254145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e83f0d82ff","size":5,"soft_fail":false,"target_va":5368953712,"va":5377207852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7423543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8c9b98eff","size":5,"soft_fail":false,"target_va":5369782624,"va":5377206162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140274480
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5923469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8739da5ff","size":5,"soft_fail":false,"target_va":5371282560,"va":5377206024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2125168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e89092dfff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377206651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2125315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8fd91dfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377206814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881b90
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t419354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e81a660600","size":5,"soft_fail":false,"target_va":5377629072,"va":5377209713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140901260
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t945364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8d46c0e00","size":5,"soft_fail":false,"target_va":5378151008,"va":5377205639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140959e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1308495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e84ff71300","size":5,"soft_fail":false,"target_va":5378514544,"va":5377206044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4675255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8b7564700","size":5,"soft_fail":false,"target_va":5381883040,"va":5377207780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141263bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10786254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8ce95a400","size":5,"soft_fail":false,"target_va":5387992000,"va":5377205741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10810118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e806f3a400","size":5,"soft_fail":false,"target_va":5388016864,"va":5377206741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_external_decorator_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34640468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e854921002","size":5,"soft_fail":false,"target_va":5411847168,"va":5377206695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34331443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e833db0b02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377209544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34331393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e801db0b02","size":5,"soft_fail":false,"target_va":5411540963,"va":5377209565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2530471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8a79c2600","size":5,"soft_fail":false,"target_va":5379739328,"va":5377208852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2096245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e875fc1f00","size":5,"soft_fail":false,"target_va":5379304320,"va":5377208070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2104555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e8eb1c2000","size":5,"soft_fail":false,"target_va":5379310272,"va":5377205712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10862173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377205120,"raw_hex":"e85dbea500","size":5,"soft_fail":false,"target_va":5388070448,"va":5377208270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":463,"record_offset":12960,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":463,"record_offset":11016,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":463,"record_offset":1184,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":463,"record_offset":476,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":463,"record_offset":10924,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":463,"record_offset":5568,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":463,"record_offset":1548,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":463,"record_offset":10952,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":11184,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":463,"record_offset":2696,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":2808,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":1696,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":2024,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":13008,"parent_record_offset":12984,"ancestor_record_offset":264,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6080,"parent_record_offset":5904,"ancestor_record_offset":264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6556,"parent_record_offset":6460,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":463,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":264,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5840,"parent_record_offset":5808,"ancestor_record_offset":264,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":264,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5872,"parent_record_offset":5840,"ancestor_record_offset":264,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5940,"parent_record_offset":5904,"ancestor_record_offset":264,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":5768,"parent_record_offset":5736,"ancestor_record_offset":264,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":5736,"parent_record_offset":5668,"ancestor_record_offset":264,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":812,"parent_record_offset":788,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":5808,"parent_record_offset":5768,"ancestor_record_offset":264,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":5636,"parent_record_offset":5600,"ancestor_record_offset":264,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":788,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":463,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":463,"record_offset":540,"parent_record_offset":500,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":463,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":463,"record_offset":1212,"parent_record_offset":1184,"ancestor_record_offset":264,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":463,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":463,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":264,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":463,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":264,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":463,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":463,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":264,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":463,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":264,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":463,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":264,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":463,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":264,"inlinee":12248,"source_line":694,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":463,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":264,"inlinee":8966,"source_line":381,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":463,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":264,"inlinee":7757,"source_line":811,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":463,"record_offset":3068,"parent_record_offset":2912,"ancestor_record_offset":264,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":264,"inlinee":9025,"source_line":185,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":3024,"parent_record_offset":2996,"ancestor_record_offset":264,"inlinee":9026,"source_line":200,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":264,"inlinee":9023,"source_line":178,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":264,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":264,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":264,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":3360,"parent_record_offset":3312,"ancestor_record_offset":264,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":463,"record_offset":3704,"parent_record_offset":3312,"ancestor_record_offset":264,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":463,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":463,"record_offset":5392,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":463,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":264,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":463,"record_offset":1248,"parent_record_offset":1212,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":463,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":264,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":744,"parent_record_offset":716,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":264,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":604,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":3448,"parent_record_offset":3360,"ancestor_record_offset":264,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":264,"inlinee":8967,"source_line":3479,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":3420,"parent_record_offset":3360,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":463,"record_offset":2304,"parent_record_offset":2272,"ancestor_record_offset":264,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":463,"record_offset":2272,"parent_record_offset":2240,"ancestor_record_offset":264,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":463,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":463,"record_offset":1340,"parent_record_offset":1248,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":463,"record_offset":1972,"parent_record_offset":1820,"ancestor_record_offset":264,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":463,"record_offset":6180,"parent_record_offset":5768,"ancestor_record_offset":264,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":264,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":463,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":13108,"parent_record_offset":13080,"ancestor_record_offset":264,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":463,"record_offset":6252,"parent_record_offset":5600,"ancestor_record_offset":264,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":660,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":1028,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":463,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":463,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":463,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":1416,"parent_record_offset":1364,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":463,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":463,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":264,"inlinee":4725,"source_line":120,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$22::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":463,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":264,"inlinee":25743,"source_line":120,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":463,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":264,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::Decorators::decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":463,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":264,"inlinee":24992,"source_line":64,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$1::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":10984,"parent_record_offset":10952,"ancestor_record_offset":264,"inlinee":24906,"source_line":1045,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":463,"record_offset":2200,"parent_record_offset":2160,"ancestor_record_offset":264,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":463,"record_offset":2240,"parent_record_offset":2200,"ancestor_record_offset":264,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":463,"record_offset":2348,"parent_record_offset":2160,"ancestor_record_offset":264,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":463,"record_offset":2160,"parent_record_offset":2108,"ancestor_record_offset":264,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":463,"record_offset":4808,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":463,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":264,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":463,"record_offset":5420,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":463,"record_offset":1904,"parent_record_offset":1848,"ancestor_record_offset":264,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":463,"record_offset":2440,"parent_record_offset":2108,"ancestor_record_offset":264,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":463,"record_offset":4892,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":463,"record_offset":4752,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":463,"record_offset":5300,"parent_record_offset":5276,"ancestor_record_offset":264,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":264,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":264,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":264,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":5276,"parent_record_offset":5108,"ancestor_record_offset":264,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":264,"inlinee":11988,"source_line":191,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":264,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":2888,"parent_record_offset":2856,"ancestor_record_offset":264,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":5216,"parent_record_offset":5132,"ancestor_record_offset":264,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":264,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":2540,"parent_record_offset":2108,"ancestor_record_offset":264,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":5512,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":1820,"parent_record_offset":1792,"ancestor_record_offset":264,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":2108,"parent_record_offset":2072,"ancestor_record_offset":264,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":5132,"parent_record_offset":5108,"ancestor_record_offset":264,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":264,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":4648,"parent_record_offset":2856,"ancestor_record_offset":264,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":264,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11392,"parent_record_offset":11360,"ancestor_record_offset":264,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11424,"parent_record_offset":11392,"ancestor_record_offset":264,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":264,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":264,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11360,"parent_record_offset":11304,"ancestor_record_offset":264,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":264,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":264,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":4780,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11452,"parent_record_offset":11392,"ancestor_record_offset":264,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":264,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":264,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":463,"record_offset":5108,"parent_record_offset":4696,"ancestor_record_offset":264,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":264,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":1628,"parent_record_offset":1576,"ancestor_record_offset":264,"inlinee":15924,"source_line":1353,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":463,"record_offset":2944,"parent_record_offset":2912,"ancestor_record_offset":264,"inlinee":9021,"source_line":85,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":463,"record_offset":3264,"parent_record_offset":2912,"ancestor_record_offset":264,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":463,"record_offset":3192,"parent_record_offset":3140,"ancestor_record_offset":264,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/handle_external_decorator_cmds.rs
//     evidence={"module":463,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":264,"inlinee":25741,"source_line":86,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/handle_external_decorator_cmds.rs
//     evidence={"module":463,"record_offset":5600,"parent_record_offset":5568,"ancestor_record_offset":264,"inlinee":25744,"source_line":99,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/handle_external_decorator_cmds.rs
//     evidence={"module":463,"record_offset":6320,"parent_record_offset":6292,"ancestor_record_offset":264,"inlinee":25745,"source_line":106,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/handle_external_decorator_cmds.rs
//     evidence={"module":463,"record_offset":9664,"parent_record_offset":9632,"ancestor_record_offset":264,"inlinee":25746,"source_line":112,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/handle_external_decorator_cmds.rs
//     evidence={"module":463,"record_offset":10372,"parent_record_offset":10344,"ancestor_record_offset":264,"inlinee":25747,"source_line":119,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x648D
// 首条位置: {"module":463,"inlinee":25741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::Decorators::decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":463,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":24992,"source_line":64,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":1628,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":15924,"source_line":1353,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":463,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":1576,"inlinee":11988,"source_line":191,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6490
// 首条位置: {"module":463,"inlinee":25744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":5636,"parent_record_offset":5600,"ancestor_record_offset":5600,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":463,"record_offset":6252,"parent_record_offset":5600,"ancestor_record_offset":5600,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5600,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5600,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6080,"parent_record_offset":5904,"ancestor_record_offset":5600,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5840,"parent_record_offset":5808,"ancestor_record_offset":5600,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":5600,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5872,"parent_record_offset":5840,"ancestor_record_offset":5600,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":5940,"parent_record_offset":5904,"ancestor_record_offset":5600,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":5768,"parent_record_offset":5736,"ancestor_record_offset":5600,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":5736,"parent_record_offset":5668,"ancestor_record_offset":5600,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":5708,"parent_record_offset":5668,"ancestor_record_offset":5600,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":5808,"parent_record_offset":5768,"ancestor_record_offset":5600,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":5600,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":463,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":5600,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":463,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":5600,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":463,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":5600,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":5600,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":5600,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":463,"record_offset":6180,"parent_record_offset":5768,"ancestor_record_offset":5600,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6491
// 首条位置: {"module":463,"inlinee":25745}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":6320,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":6320,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":6556,"parent_record_offset":6460,"ancestor_record_offset":6320,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":6320,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":6320,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":6320,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":6320,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":6376,"parent_record_offset":6348,"ancestor_record_offset":6320,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6492
// 首条位置: {"module":463,"inlinee":25746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":9700,"parent_record_offset":9664,"ancestor_record_offset":9664,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":463,"record_offset":10304,"parent_record_offset":9664,"ancestor_record_offset":9664,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":9664,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":9664,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":10132,"parent_record_offset":9956,"ancestor_record_offset":9664,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":9896,"parent_record_offset":9868,"ancestor_record_offset":9664,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":9956,"parent_record_offset":9924,"ancestor_record_offset":9664,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":9924,"parent_record_offset":9896,"ancestor_record_offset":9664,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":9992,"parent_record_offset":9956,"ancestor_record_offset":9664,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":9832,"parent_record_offset":9800,"ancestor_record_offset":9664,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":9800,"parent_record_offset":9732,"ancestor_record_offset":9664,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":9772,"parent_record_offset":9732,"ancestor_record_offset":9664,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":9868,"parent_record_offset":9832,"ancestor_record_offset":9664,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":9732,"parent_record_offset":9700,"ancestor_record_offset":9664,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":463,"record_offset":10016,"parent_record_offset":9992,"ancestor_record_offset":9664,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":463,"record_offset":10040,"parent_record_offset":10016,"ancestor_record_offset":9664,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":463,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":9664,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":9664,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":10088,"parent_record_offset":10064,"ancestor_record_offset":9664,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":463,"record_offset":10232,"parent_record_offset":9832,"ancestor_record_offset":9664,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6493
// 首条位置: {"module":463,"inlinee":25747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":10400,"parent_record_offset":10372,"ancestor_record_offset":10372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":10372,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":10608,"parent_record_offset":10512,"ancestor_record_offset":10372,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":10544,"parent_record_offset":10512,"ancestor_record_offset":10372,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":10512,"parent_record_offset":10484,"ancestor_record_offset":10372,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":10484,"parent_record_offset":10456,"ancestor_record_offset":10372,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":10576,"parent_record_offset":10544,"ancestor_record_offset":10372,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":10428,"parent_record_offset":10400,"ancestor_record_offset":10372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00463.jsonl.gz"}
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
// 完整结构及原始类型记录: handle_external_decorator_cmds.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: handle_external_decorator_cmds.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: handle_external_decorator_cmds.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds
// 信息 ID: eb0199b67f6ba69320976341
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2820ec54b8cccbe2cc3794e9: void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > @ 0x273b5a -> 0x81a380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > > @ 0x4464b4 -> 0x81a380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$0
// 信息 ID: 24616de0150e46af765e08db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds; module=463, inline_record=1576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$1
// 信息 ID: 66e0cf21ed51bc4ec85abc6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds; module=463, inline_record=5600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$2
// 信息 ID: 311119f3e148bd14ce502684
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds; module=463, inline_record=6320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$3
// 信息 ID: de88a1feb6f6f708d2d5cd37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds; module=463, inline_record=9664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds::closure$4
// 信息 ID: fa928fad9cbc9d04c2e0557f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::handle_external_decorator_cmds::handle_external_decorator_cmds; module=463, inline_record=10372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
