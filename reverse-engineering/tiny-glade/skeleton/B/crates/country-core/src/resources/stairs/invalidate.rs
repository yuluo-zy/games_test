// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/stairs/invalidate.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2df6a65f318e64092045d693466db99099201f3b89202adb459da3285a03de85
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: invalidate.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 586, 1610, 1613
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
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.7.1"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":12512,"rva":11760000,"va":5380469120}
// 编译实例 RVAs: 0xb37180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106b70
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10686409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e837f05cff","size":5,"soft_fail":false,"target_va":5369785200,"va":5380471604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10412073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8d71f61ff","size":5,"soft_fail":false,"target_va":5370060624,"va":5380472692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140252ad0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9325756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e844b371ff","size":5,"soft_fail":false,"target_va":5371144912,"va":5380470663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5795086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8f292a7ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5380471417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b14b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5793109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8ab9aa7ff","size":5,"soft_fail":false,"target_va":5374678192,"va":5380471296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5790028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8b4a6a7ff","size":5,"soft_fail":false,"target_va":5374680032,"va":5380470055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5389988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e85cc1adff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380471487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4f360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t96276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e814780100","size":5,"soft_fail":false,"target_va":5380567904,"va":5380471623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t151154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8724e0200","size":5,"soft_fail":false,"target_va":5380621856,"va":5380470697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t151677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e87d500200","size":5,"soft_fail":false,"target_va":5380623056,"va":5380471374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b765b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t256691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8b3ea0300","size":5,"soft_fail":false,"target_va":5380728240,"va":5380471544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b77c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t263992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e838070400","size":5,"soft_fail":false,"target_va":5380734080,"va":5380470083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b78550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t265009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8310b0400","size":5,"soft_fail":false,"target_va":5380736336,"va":5380471322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b78a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t266216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8e80f0400","size":5,"soft_fail":false,"target_va":5380737664,"va":5380471443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141268520
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7540603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e87b0f7300","size":5,"soft_fail":false,"target_va":5388010784,"va":5380470176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141477ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9702326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8b60b9400","size":5,"soft_fail":false,"target_va":5390172144,"va":5380469813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141523430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10404111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e80fc19e00","size":5,"soft_fail":false,"target_va":5390873648,"va":5380469532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: invalidate.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31374359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e817bcde01","size":5,"soft_fail":false,"target_va":5411847200,"va":5380472836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31069691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8fb15da01","size":5,"soft_fail":false,"target_va":5411542296,"va":5380472600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2320863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e82196dcff","size":5,"soft_fail":false,"target_va":5378150016,"va":5380470874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::visual::StairsVisualState::remove_edge
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-586584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8a80cf7ff","size":5,"soft_fail":false,"target_va":5379884576,"va":5380471155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2188572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e81c652100","size":5,"soft_fail":false,"target_va":5382657856,"va":5380469279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2284457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380469120,"raw_hex":"e8a9db2200","size":5,"soft_fail":false,"target_va":5382753792,"va":5380469330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":586,"record_offset":12744,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":586,"record_offset":13232,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":18652,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":586,"record_offset":15520,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtWorld::remove
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":586,"record_offset":21760,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":26306,"source_line":190,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_assembly_params::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":586,"record_offset":21732,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":29562,"source_line":126,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::StairGraphEdge::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":586,"record_offset":24108,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":28265,"source_line":56,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::StairsState::drain_dirty_edges
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":586,"record_offset":12800,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":28335,"source_line":489,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::visual::StairsVisualState::force_rebuild
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/visual.rs
//     evidence={"module":586,"record_offset":19676,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":32251,"source_line":132,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastWorld::retain_colliders
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":586,"record_offset":17032,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":25519,"source_line":651,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":586,"record_offset":20768,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":4972,"source_line":454,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":586,"record_offset":23964,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":10600,"source_line":719,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":586,"record_offset":16800,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":15604,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":24753,"source_line":246,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":15688,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":25277,"source_line":1513,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":17088,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":4801,"source_line":1545,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":16100,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$25::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":17740,"parent_record_offset":12512,"ancestor_record_offset":12512,"inlinee":4820,"source_line":1604,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":586,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":12512,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":586,"record_offset":22584,"parent_record_offset":22500,"ancestor_record_offset":12512,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":12512,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":22528,"parent_record_offset":22500,"ancestor_record_offset":12512,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":22500,"parent_record_offset":22476,"ancestor_record_offset":12512,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":12512,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":21072,"parent_record_offset":21048,"ancestor_record_offset":12512,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":22476,"parent_record_offset":22452,"ancestor_record_offset":12512,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":586,"record_offset":18784,"parent_record_offset":18748,"ancestor_record_offset":12512,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":586,"record_offset":18748,"parent_record_offset":18724,"ancestor_record_offset":12512,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":586,"record_offset":21024,"parent_record_offset":20996,"ancestor_record_offset":12512,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":586,"record_offset":20996,"parent_record_offset":20968,"ancestor_record_offset":12512,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":586,"record_offset":20968,"parent_record_offset":20940,"ancestor_record_offset":12512,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":586,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":12512,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":586,"record_offset":13832,"parent_record_offset":13800,"ancestor_record_offset":12512,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":586,"record_offset":13892,"parent_record_offset":13860,"ancestor_record_offset":12512,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":586,"record_offset":20492,"parent_record_offset":20468,"ancestor_record_offset":12512,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":586,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12512,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":586,"record_offset":20000,"parent_record_offset":19976,"ancestor_record_offset":12512,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":586,"record_offset":16004,"parent_record_offset":15972,"ancestor_record_offset":12512,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":586,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":12512,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":586,"record_offset":20588,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":586,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":12512,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":586,"record_offset":15472,"parent_record_offset":15448,"ancestor_record_offset":12512,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":586,"record_offset":22404,"parent_record_offset":22348,"ancestor_record_offset":12512,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":586,"record_offset":13152,"parent_record_offset":13128,"ancestor_record_offset":12512,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":586,"record_offset":17628,"parent_record_offset":17600,"ancestor_record_offset":12512,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":586,"record_offset":22552,"parent_record_offset":22528,"ancestor_record_offset":12512,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":586,"record_offset":13860,"parent_record_offset":13800,"ancestor_record_offset":12512,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":586,"record_offset":18084,"parent_record_offset":18052,"ancestor_record_offset":12512,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":586,"record_offset":16384,"parent_record_offset":16360,"ancestor_record_offset":12512,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":586,"record_offset":13436,"parent_record_offset":13400,"ancestor_record_offset":12512,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":586,"record_offset":24048,"parent_record_offset":23964,"ancestor_record_offset":12512,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":586,"record_offset":13552,"parent_record_offset":13524,"ancestor_record_offset":12512,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":586,"record_offset":16040,"parent_record_offset":15888,"ancestor_record_offset":12512,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":14084,"parent_record_offset":13268,"ancestor_record_offset":12512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":586,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":12512,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":586,"record_offset":14420,"parent_record_offset":14180,"ancestor_record_offset":12512,"inlinee":9631,"source_line":1475,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":586,"record_offset":18464,"parent_record_offset":17852,"ancestor_record_offset":12512,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":586,"record_offset":13640,"parent_record_offset":13524,"ancestor_record_offset":12512,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":586,"record_offset":16604,"parent_record_offset":16580,"ancestor_record_offset":12512,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":586,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":12512,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":586,"record_offset":18816,"parent_record_offset":18784,"ancestor_record_offset":12512,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":586,"record_offset":24016,"parent_record_offset":23988,"ancestor_record_offset":12512,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":13716,"parent_record_offset":13688,"ancestor_record_offset":12512,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":12512,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":12512,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":586,"record_offset":21612,"parent_record_offset":21440,"ancestor_record_offset":12512,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":586,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":12512,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":586,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":12512,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":586,"record_offset":22012,"parent_record_offset":21976,"ancestor_record_offset":12512,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":586,"record_offset":18844,"parent_record_offset":18784,"ancestor_record_offset":12512,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":586,"record_offset":18876,"parent_record_offset":18748,"ancestor_record_offset":12512,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":586,"record_offset":20516,"parent_record_offset":20492,"ancestor_record_offset":12512,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":586,"record_offset":20540,"parent_record_offset":20516,"ancestor_record_offset":12512,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":586,"record_offset":21152,"parent_record_offset":20864,"ancestor_record_offset":12512,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":586,"record_offset":16328,"parent_record_offset":16296,"ancestor_record_offset":12512,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":586,"record_offset":16360,"parent_record_offset":16328,"ancestor_record_offset":12512,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":586,"record_offset":18128,"parent_record_offset":17940,"ancestor_record_offset":12512,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":586,"record_offset":16296,"parent_record_offset":16248,"ancestor_record_offset":12512,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":586,"record_offset":19976,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":586,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":12512,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":586,"record_offset":20616,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":586,"record_offset":15972,"parent_record_offset":15916,"ancestor_record_offset":12512,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":586,"record_offset":16480,"parent_record_offset":16248,"ancestor_record_offset":12512,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":586,"record_offset":20060,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":586,"record_offset":19948,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":586,"record_offset":14300,"parent_record_offset":14240,"ancestor_record_offset":12512,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":586,"record_offset":20468,"parent_record_offset":20444,"ancestor_record_offset":12512,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":12872,"parent_record_offset":12848,"ancestor_record_offset":12512,"inlinee":10289,"source_line":888,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":19756,"parent_record_offset":19732,"ancestor_record_offset":12512,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":19732,"parent_record_offset":19708,"ancestor_record_offset":12512,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":12512,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":15784,"parent_record_offset":15760,"ancestor_record_offset":12512,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":12512,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":16824,"parent_record_offset":16800,"ancestor_record_offset":12512,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":16848,"parent_record_offset":16824,"ancestor_record_offset":12512,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":20444,"parent_record_offset":20276,"ancestor_record_offset":12512,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":17168,"parent_record_offset":17128,"ancestor_record_offset":12512,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":16184,"parent_record_offset":16156,"ancestor_record_offset":12512,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":17796,"parent_record_offset":17768,"ancestor_record_offset":12512,"inlinee":4824,"source_line":3275,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":16156,"parent_record_offset":16128,"ancestor_record_offset":12512,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":12512,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":586,"record_offset":16876,"parent_record_offset":16848,"ancestor_record_offset":12512,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":20384,"parent_record_offset":20300,"ancestor_record_offset":12512,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":20328,"parent_record_offset":20300,"ancestor_record_offset":12512,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":16580,"parent_record_offset":16248,"ancestor_record_offset":12512,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":20708,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":13036,"parent_record_offset":12948,"ancestor_record_offset":12512,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":16248,"parent_record_offset":16212,"ancestor_record_offset":12512,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":12512,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":12512,"inlinee":6290,"source_line":1313,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":13128,"parent_record_offset":12896,"ancestor_record_offset":12512,"inlinee":6279,"source_line":1328,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":19840,"parent_record_offset":19816,"ancestor_record_offset":12512,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":19816,"parent_record_offset":19756,"ancestor_record_offset":12512,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":12512,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":17520,"parent_record_offset":17208,"ancestor_record_offset":12512,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":12512,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":26848,"parent_record_offset":26816,"ancestor_record_offset":12512,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":13004,"parent_record_offset":12948,"ancestor_record_offset":12512,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":14208,"parent_record_offset":14180,"ancestor_record_offset":12512,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":14240,"parent_record_offset":14208,"ancestor_record_offset":12512,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":12976,"parent_record_offset":12948,"ancestor_record_offset":12512,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":26760,"parent_record_offset":26736,"ancestor_record_offset":12512,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":19864,"parent_record_offset":19840,"ancestor_record_offset":12512,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":26816,"parent_record_offset":26760,"ancestor_record_offset":12512,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":26788,"parent_record_offset":26760,"ancestor_record_offset":12512,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":12512,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":14272,"parent_record_offset":14240,"ancestor_record_offset":12512,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":19920,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":17600,"parent_record_offset":17556,"ancestor_record_offset":12512,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":14388,"parent_record_offset":14208,"ancestor_record_offset":12512,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":26736,"parent_record_offset":26712,"ancestor_record_offset":12512,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":17208,"parent_record_offset":17168,"ancestor_record_offset":12512,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":16212,"parent_record_offset":16184,"ancestor_record_offset":12512,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":26536,"parent_record_offset":26512,"ancestor_record_offset":12512,"inlinee":4904,"source_line":3846,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$41::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":17824,"parent_record_offset":17796,"ancestor_record_offset":12512,"inlinee":4826,"source_line":3887,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$47::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":12512,"inlinee":11906,"source_line":3937,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":20276,"parent_record_offset":19864,"ancestor_record_offset":12512,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":12848,"parent_record_offset":12824,"ancestor_record_offset":12512,"inlinee":28302,"source_line":348,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":12512,"inlinee":24754,"source_line":326,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":15736,"parent_record_offset":15712,"ancestor_record_offset":12512,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":12512,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":17128,"parent_record_offset":17088,"ancestor_record_offset":12512,"inlinee":4803,"source_line":1774,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":16128,"parent_record_offset":16100,"ancestor_record_offset":12512,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":586,"record_offset":17768,"parent_record_offset":17740,"ancestor_record_offset":12512,"inlinee":4822,"source_line":1843,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map.rs
//     evidence={"module":586,"record_offset":19644,"parent_record_offset":19580,"ancestor_record_offset":12512,"inlinee":4653,"source_line":711,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get_index_of
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map.rs
//     evidence={"module":586,"record_offset":20864,"parent_record_offset":20812,"ancestor_record_offset":12512,"inlinee":4654,"source_line":755,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::core::impl$1::as_entries
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map/core.rs
//     evidence={"module":586,"record_offset":20940,"parent_record_offset":20912,"ancestor_record_offset":12512,"inlinee":4658,"source_line":121,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::impl$1::as_entries
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map.rs
//     evidence={"module":586,"record_offset":20912,"parent_record_offset":20864,"ancestor_record_offset":12512,"inlinee":4656,"source_line":125,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":586,"record_offset":24072,"parent_record_offset":24048,"ancestor_record_offset":12512,"inlinee":10594,"source_line":725,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":586,"record_offset":22376,"parent_record_offset":22348,"ancestor_record_offset":12512,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":586,"record_offset":22348,"parent_record_offset":22320,"ancestor_record_offset":12512,"inlinee":6212,"source_line":2118,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":586,"record_offset":19708,"parent_record_offset":19676,"ancestor_record_offset":12512,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":586,"record_offset":21788,"parent_record_offset":21760,"ancestor_record_offset":12512,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":586,"record_offset":17056,"parent_record_offset":17032,"ancestor_record_offset":12512,"inlinee":17336,"source_line":705,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":12824,"parent_record_offset":12800,"ancestor_record_offset":12512,"inlinee":28301,"source_line":275,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":15712,"parent_record_offset":15688,"ancestor_record_offset":12512,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":14000,"parent_record_offset":13268,"ancestor_record_offset":12512,"inlinee":13714,"source_line":392,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":13268,"parent_record_offset":13232,"ancestor_record_offset":12512,"inlinee":13713,"source_line":1067,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":586,"record_offset":14028,"parent_record_offset":13268,"ancestor_record_offset":12512,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":586,"record_offset":13344,"parent_record_offset":13316,"ancestor_record_offset":12512,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":586,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":12512,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":586,"record_offset":13464,"parent_record_offset":13436,"ancestor_record_offset":12512,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":586,"record_offset":13800,"parent_record_offset":13400,"ancestor_record_offset":12512,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":586,"record_offset":13316,"parent_record_offset":13268,"ancestor_record_offset":12512,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":586,"record_offset":13492,"parent_record_offset":13464,"ancestor_record_offset":12512,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":586,"record_offset":13524,"parent_record_offset":13492,"ancestor_record_offset":12512,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":586,"record_offset":13400,"parent_record_offset":13372,"ancestor_record_offset":12512,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":586,"record_offset":13372,"parent_record_offset":13344,"ancestor_record_offset":12512,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12094
// 首条位置: {"module":1610,"inlinee":73876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1610,"record_offset":75784,"parent_record_offset":75740,"ancestor_record_offset":75740,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1610,"record_offset":76416,"parent_record_offset":76392,"ancestor_record_offset":75740,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1610,"record_offset":76064,"parent_record_offset":76040,"ancestor_record_offset":75740,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1610,"record_offset":76148,"parent_record_offset":76124,"ancestor_record_offset":75740,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":76088,"parent_record_offset":76064,"ancestor_record_offset":75740,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":76512,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1610,"record_offset":76300,"parent_record_offset":76272,"ancestor_record_offset":75740,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1610,"record_offset":76272,"parent_record_offset":76244,"ancestor_record_offset":75740,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1610,"record_offset":76440,"parent_record_offset":76416,"ancestor_record_offset":75740,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1610,"record_offset":76464,"parent_record_offset":76440,"ancestor_record_offset":75740,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1610,"record_offset":76208,"parent_record_offset":76180,"ancestor_record_offset":75740,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1610,"record_offset":76244,"parent_record_offset":76208,"ancestor_record_offset":75740,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1610,"record_offset":76180,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1610,"record_offset":76040,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1610,"record_offset":76540,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1610,"record_offset":76124,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1610,"record_offset":76012,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1610,"record_offset":76392,"parent_record_offset":76368,"ancestor_record_offset":75740,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1610,"record_offset":75832,"parent_record_offset":75808,"ancestor_record_offset":75740,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1610,"record_offset":75856,"parent_record_offset":75832,"ancestor_record_offset":75740,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1610,"record_offset":76368,"parent_record_offset":76344,"ancestor_record_offset":75740,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1610,"record_offset":75884,"parent_record_offset":75856,"ancestor_record_offset":75740,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":76632,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":75936,"parent_record_offset":75912,"ancestor_record_offset":75740,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":75912,"parent_record_offset":75856,"ancestor_record_offset":75740,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":75960,"parent_record_offset":75936,"ancestor_record_offset":75740,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":76344,"parent_record_offset":75960,"ancestor_record_offset":75740,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1610,"record_offset":75808,"parent_record_offset":75784,"ancestor_record_offset":75740,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120F9
// 首条位置: {"module":1613,"inlinee":73977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1613,"record_offset":241360,"parent_record_offset":241332,"ancestor_record_offset":241332,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1613,"record_offset":242040,"parent_record_offset":242016,"ancestor_record_offset":241332,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1613,"record_offset":241692,"parent_record_offset":241668,"ancestor_record_offset":241332,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1613,"record_offset":241776,"parent_record_offset":241752,"ancestor_record_offset":241332,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1613,"record_offset":241716,"parent_record_offset":241692,"ancestor_record_offset":241332,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1613,"record_offset":242108,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1613,"record_offset":241924,"parent_record_offset":241896,"ancestor_record_offset":241332,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1613,"record_offset":241896,"parent_record_offset":241868,"ancestor_record_offset":241332,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1613,"record_offset":242064,"parent_record_offset":242040,"ancestor_record_offset":241332,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1613,"record_offset":241836,"parent_record_offset":241808,"ancestor_record_offset":241332,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1613,"record_offset":241868,"parent_record_offset":241836,"ancestor_record_offset":241332,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1613,"record_offset":241808,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1613,"record_offset":241668,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1613,"record_offset":242136,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1613,"record_offset":241752,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1613,"record_offset":241640,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1613,"record_offset":242016,"parent_record_offset":241992,"ancestor_record_offset":241332,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":241416,"parent_record_offset":241388,"ancestor_record_offset":241332,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":241444,"parent_record_offset":241416,"ancestor_record_offset":241332,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":241992,"parent_record_offset":241968,"ancestor_record_offset":241332,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":241480,"parent_record_offset":241444,"ancestor_record_offset":241332,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":242228,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":241532,"parent_record_offset":241508,"ancestor_record_offset":241332,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":241508,"parent_record_offset":241444,"ancestor_record_offset":241332,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":241556,"parent_record_offset":241532,"ancestor_record_offset":241332,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":241612,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":241968,"parent_record_offset":241556,"ancestor_record_offset":241332,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1613,"record_offset":241388,"parent_record_offset":241360,"ancestor_record_offset":241332,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01613.jsonl.gz"}
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
// 完整结构及原始类型记录: invalidate.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: invalidate.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: invalidate.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts
// 信息 ID: d55c459c816e32cd9824ad70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f576eb5984a3d27bff6e342f: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands)> > @ 0x33b92e -> 0xb37180 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands)> > > @ 0x42d45d -> 0xb37180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure$0
// 信息 ID: 4d372bb5c170b4b6d0d23e9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$0> > > > > >; module=1610, inline_record=75740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure$1
// 信息 ID: 0bc649850620706d10fb59f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$1 <- hashbrown::map::HashMap::retain<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::ColliderMeshInstance,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::collision::world::impl$13::retain_colliders::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] hashbrown::map::HashMap::retain<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::ColliderMeshInstance,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::collision::world::impl$13::retain_colliders::closure_env$0<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env$1> >; module=1613, inline_record=241332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
