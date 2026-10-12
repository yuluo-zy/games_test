// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/history/replay_terrain.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b5a2633b16e7a1fe3c54c237b11afd3cb06a8e6db03d70e97ac4946a4280f405
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 524
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::replay_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":22328,"rva":10047824,"va":5378756944}
// 编译实例 RVAs: 0x995150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7010729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8570695ff","size":5,"soft_fail":false,"target_va":5371747712,"va":5378758436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3676215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8c9e7c7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378757698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3676406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e80ae7c7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378757905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406978e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3137286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8fa20d0ff","size":5,"soft_fail":false,"target_va":5375621344,"va":5378758625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1062837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e84bc8efff","size":5,"soft_fail":false,"target_va":5377695408,"va":5378758240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1861695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e83f681c00","size":5,"soft_fail":false,"target_va":5380620352,"va":5378758652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1862465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8416b1c00","size":5,"soft_fail":false,"target_va":5380620960,"va":5378758490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1865104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e890751c00","size":5,"soft_fail":false,"target_va":5380623680,"va":5378758571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1865906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8b2781c00","size":5,"soft_fail":false,"target_va":5380624320,"va":5378758409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1865878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e896781c00","size":5,"soft_fail":false,"target_va":5380624400,"va":5378758517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141526d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12130346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e82a18b900","size":5,"soft_fail":false,"target_va":5390888208,"va":5378757857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33089411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e883e7f801","size":5,"soft_fail":false,"target_va":5411847168,"va":5378757752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33088822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e836e5f801","size":5,"soft_fail":false,"target_va":5411847184,"va":5378758357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2017817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e819ca1e00","size":5,"soft_fail":false,"target_va":5380775408,"va":5378757586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32782148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e84437f401","size":5,"soft_fail":false,"target_va":5411540963,"va":5378758810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32782459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e87b38f401","size":5,"soft_fail":false,"target_va":5411541225,"va":5378758761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32783272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8a83bf401","size":5,"soft_fail":false,"target_va":5411542064,"va":5378758787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::reapply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1294543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8313fecff","size":5,"soft_fail":false,"target_va":5377462992,"va":5378757530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::revert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1295935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756944,"raw_hex":"e8c139ecff","size":5,"soft_fail":false,"target_va":5377461552,"va":5378757482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":524,"record_offset":23552,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":22964,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":23076,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":27372,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":23676,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":25048,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":524,"record_offset":22532,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":23464,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":524,"record_offset":23260,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":24644,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":22992,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":24936,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":524,"record_offset":22596,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::set_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":524,"record_offset":24280,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":28869,"source_line":110,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_terrain::ease_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_terrain.rs
//     evidence={"module":524,"record_offset":23496,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":29993,"source_line":168,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":24132,"parent_record_offset":24108,"ancestor_record_offset":22328,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":22328,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":24588,"parent_record_offset":24564,"ancestor_record_offset":22328,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":24108,"parent_record_offset":23932,"ancestor_record_offset":22328,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":24564,"parent_record_offset":24480,"ancestor_record_offset":22328,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":22732,"parent_record_offset":22704,"ancestor_record_offset":22328,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":22620,"parent_record_offset":22596,"ancestor_record_offset":22328,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":22644,"parent_record_offset":22620,"ancestor_record_offset":22328,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":22832,"parent_record_offset":22644,"ancestor_record_offset":22328,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":22676,"parent_record_offset":22644,"ancestor_record_offset":22328,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":22704,"parent_record_offset":22676,"ancestor_record_offset":22328,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":22764,"parent_record_offset":22732,"ancestor_record_offset":22328,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":22864,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":23872,"parent_record_offset":23844,"ancestor_record_offset":22328,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":22788,"parent_record_offset":22764,"ancestor_record_offset":22328,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":24508,"parent_record_offset":24480,"ancestor_record_offset":22328,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":24480,"parent_record_offset":24456,"ancestor_record_offset":22328,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":22912,"parent_record_offset":22888,"ancestor_record_offset":22328,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":22888,"parent_record_offset":22864,"ancestor_record_offset":22328,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":23932,"parent_record_offset":23900,"ancestor_record_offset":22328,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":23900,"parent_record_offset":23872,"ancestor_record_offset":22328,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":24456,"parent_record_offset":24432,"ancestor_record_offset":22328,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":23968,"parent_record_offset":23932,"ancestor_record_offset":22328,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":23808,"parent_record_offset":23776,"ancestor_record_offset":22328,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":23776,"parent_record_offset":23708,"ancestor_record_offset":22328,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":524,"record_offset":25292,"parent_record_offset":25256,"ancestor_record_offset":22328,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":524,"record_offset":25256,"parent_record_offset":25232,"ancestor_record_offset":22328,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":23116,"parent_record_offset":23076,"ancestor_record_offset":22328,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":23748,"parent_record_offset":23708,"ancestor_record_offset":22328,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":27420,"parent_record_offset":27372,"ancestor_record_offset":22328,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":27636,"parent_record_offset":27372,"ancestor_record_offset":22328,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":23844,"parent_record_offset":23808,"ancestor_record_offset":22328,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":23708,"parent_record_offset":23676,"ancestor_record_offset":22328,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":27692,"parent_record_offset":27668,"ancestor_record_offset":22328,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":22328,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":524,"record_offset":22560,"parent_record_offset":22532,"ancestor_record_offset":22328,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":524,"record_offset":23992,"parent_record_offset":23968,"ancestor_record_offset":22328,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":524,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":22328,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":24328,"parent_record_offset":24280,"ancestor_record_offset":22328,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":524,"record_offset":24232,"parent_record_offset":24208,"ancestor_record_offset":22328,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":524,"record_offset":25008,"parent_record_offset":24964,"ancestor_record_offset":22328,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":22328,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":22328,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":24532,"parent_record_offset":24508,"ancestor_record_offset":22328,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":24208,"parent_record_offset":23808,"ancestor_record_offset":22328,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":24384,"parent_record_offset":24280,"ancestor_record_offset":22328,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":23040,"parent_record_offset":23016,"ancestor_record_offset":22328,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":23420,"parent_record_offset":23396,"ancestor_record_offset":22328,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":25324,"parent_record_offset":25292,"ancestor_record_offset":22328,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":25120,"parent_record_offset":25048,"ancestor_record_offset":22328,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":23340,"parent_record_offset":23296,"ancestor_record_offset":22328,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":25144,"parent_record_offset":25120,"ancestor_record_offset":22328,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":23228,"parent_record_offset":23076,"ancestor_record_offset":22328,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":24964,"parent_record_offset":24936,"ancestor_record_offset":22328,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":524,"record_offset":23016,"parent_record_offset":22992,"ancestor_record_offset":22328,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":23396,"parent_record_offset":23372,"ancestor_record_offset":22328,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":23296,"parent_record_offset":23260,"ancestor_record_offset":22328,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":23372,"parent_record_offset":23296,"ancestor_record_offset":22328,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":25352,"parent_record_offset":25292,"ancestor_record_offset":22328,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":25384,"parent_record_offset":25256,"ancestor_record_offset":22328,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":24356,"parent_record_offset":24280,"ancestor_record_offset":22328,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":524,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":22328,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::forward
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":21032,"rva":10047280,"va":5378756400}
// 编译实例 RVAs: 0x994f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1864302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756400,"raw_hex":"e86e721c00","size":5,"soft_fail":false,"target_va":5380620960,"va":5378756653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141476710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11409281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756400,"raw_hex":"e88117ae00","size":5,"soft_fail":false,"target_va":5390165776,"va":5378756490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32785447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756400,"raw_hex":"e82744f401","size":5,"soft_fail":false,"target_va":5411542064,"va":5378756612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":21524,"parent_record_offset":21032,"ancestor_record_offset":21032,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":524,"record_offset":21200,"parent_record_offset":21032,"ancestor_record_offset":21032,"inlinee":26515,"source_line":78,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_terrain::TerrainReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_terrain.rs
//     evidence={"module":524,"record_offset":21496,"parent_record_offset":21032,"ancestor_record_offset":21032,"inlinee":29991,"source_line":50,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":524,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":21032,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":21464,"parent_record_offset":21200,"ancestor_record_offset":21032,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":21436,"parent_record_offset":21200,"ancestor_record_offset":21032,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":524,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":21032,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":524,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":21032,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":21324,"parent_record_offset":21200,"ancestor_record_offset":21032,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":21380,"parent_record_offset":21200,"ancestor_record_offset":21032,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":21288,"parent_record_offset":21264,"ancestor_record_offset":21032,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":21240,"parent_record_offset":21200,"ancestor_record_offset":21032,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":524,"record_offset":21264,"parent_record_offset":21240,"ancestor_record_offset":21032,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":21584,"parent_record_offset":21548,"ancestor_record_offset":21032,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":21608,"parent_record_offset":21584,"ancestor_record_offset":21032,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":21640,"parent_record_offset":21548,"ancestor_record_offset":21032,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::backward
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":21680,"rva":10047552,"va":5378756672}
// 编译实例 RVAs: 0x995040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1864030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756672,"raw_hex":"e85e711c00","size":5,"soft_fail":false,"target_va":5380620960,"va":5378756925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141476710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11409009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756672,"raw_hex":"e87116ae00","size":5,"soft_fail":false,"target_va":5390165776,"va":5378756762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32785175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378756672,"raw_hex":"e81743f401","size":5,"soft_fail":false,"target_va":5411542064,"va":5378756884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":22172,"parent_record_offset":21680,"ancestor_record_offset":21680,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":524,"record_offset":21848,"parent_record_offset":21680,"ancestor_record_offset":21680,"inlinee":26515,"source_line":78,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_terrain::TerrainReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_terrain.rs
//     evidence={"module":524,"record_offset":22144,"parent_record_offset":21680,"ancestor_record_offset":21680,"inlinee":29991,"source_line":50,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":524,"record_offset":22196,"parent_record_offset":22172,"ancestor_record_offset":21680,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":22112,"parent_record_offset":21848,"ancestor_record_offset":21680,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":524,"record_offset":22084,"parent_record_offset":21848,"ancestor_record_offset":21680,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":524,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":21680,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":524,"record_offset":22052,"parent_record_offset":22028,"ancestor_record_offset":21680,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":21972,"parent_record_offset":21848,"ancestor_record_offset":21680,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":22028,"parent_record_offset":21848,"ancestor_record_offset":21680,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":21680,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":21888,"parent_record_offset":21848,"ancestor_record_offset":21680,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":524,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":21680,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":22232,"parent_record_offset":22196,"ancestor_record_offset":21680,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":22256,"parent_record_offset":22232,"ancestor_record_offset":21680,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":22288,"parent_record_offset":22196,"ancestor_record_offset":21680,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7527
// 首条位置: {"module":524,"inlinee":29991}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::ease_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7529
// 首条位置: {"module":524,"inlinee":29993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":524,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":23496,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00524.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_terrain::TerrainReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/B-core/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::history::replay_terrain::TerrainReplayCmd", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::backward
// 信息 ID: 35b073439d647831c8e1b345
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0> @ 0x143df07 -> 0x995040 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0> @ 0x1440797 -> 0x995040 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_path::generate_path_replay_cmds::closure_env$0> @ 0x1443027 -> 0x995040 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_water::replay_water::closure$1::closure_env$0> @ 0x14458b7 -> 0x995040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::forward
// 信息 ID: 1a575ea8072d321193471a6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0> @ 0x143df3d -> 0x994f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0> @ 0x14407cd -> 0x994f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_path::generate_path_replay_cmds::closure_env$0> @ 0x144305d -> 0x994f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_water::replay_water::closure$1::closure_env$0> @ 0x14458ed -> 0x994f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::new
// 信息 ID: 6f691485ba7e951a39071881
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::forward; module=524, inline_record=21496; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::backward; module=524, inline_record=22144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::ease_out
// 信息 ID: be909e764dab8bad5c7d5676
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::replay_terrain; module=524, inline_record=23496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::replay_terrain
// 信息 ID: 91705ac54ee89c49869ba50d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7e9c5a150bfed2ec8272361b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> > @ 0x8c3c40 -> 0x995150 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> > > @ 0x14cff12 -> 0x995150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
