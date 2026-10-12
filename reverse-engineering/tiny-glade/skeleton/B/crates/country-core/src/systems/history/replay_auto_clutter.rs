// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/history/replay_auto_clutter.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c92fd924e3e1ec324aa160ecca0249ac51418e36b54bf8e0af8e407aef848767
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_auto_clutter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 490, 561, 579, 1613
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_auto_clutter::replay_auto_clutter
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":41536,"rva":11160160,"va":5379869280}
// 编译实例 RVAs: 0xaa4a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8235353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8a75682ff","size":5,"soft_fail":false,"target_va":5371634416,"va":5379869764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8122681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8c70e84ff","size":5,"soft_fail":false,"target_va":5371747712,"va":5379870388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4789138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e86eecb6ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379870637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406978e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4249238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e86a29bfff","size":5,"soft_fail":false,"target_va":5375621344,"va":5379870577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2174787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8bdd0deff","size":5,"soft_fail":false,"target_va":5377695408,"va":5379870190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t749743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8af700b00","size":5,"soft_fail":false,"target_va":5380620352,"va":5379870604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t750513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8b1730b00","size":5,"soft_fail":false,"target_va":5380620960,"va":5379870442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t753152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8007e0b00","size":5,"soft_fail":false,"target_va":5380623680,"va":5379870523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t753954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e822810b00","size":5,"soft_fail":false,"target_va":5380624320,"va":5379870361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t753926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e806810b00","size":5,"soft_fail":false,"target_va":5380624400,"va":5379870469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b7a160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t873766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e826550d00","size":5,"soft_fail":false,"target_va":5380743520,"va":5379869749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_auto_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31976870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e8a6ede701","size":5,"soft_fail":false,"target_va":5411847184,"va":5379870309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t905571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e863d10d00","size":5,"soft_fail":false,"target_va":5380775408,"va":5379869832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31670535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379869280,"raw_hex":"e80741e301","size":5,"soft_fail":false,"target_va":5411541225,"va":5379870685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":42172,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":42284,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":46424,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":44100,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":561,"record_offset":41740,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":43780,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":561,"record_offset":42468,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":44232,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":42200,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":43988,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":561,"record_offset":41804,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":561,"record_offset":43724,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":561,"record_offset":42728,"parent_record_offset":41536,"ancestor_record_offset":41536,"inlinee":25284,"source_line":341,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":46976,"parent_record_offset":46952,"ancestor_record_offset":41536,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":46952,"parent_record_offset":46864,"ancestor_record_offset":41536,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":561,"record_offset":41940,"parent_record_offset":41912,"ancestor_record_offset":41536,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":561,"record_offset":41828,"parent_record_offset":41804,"ancestor_record_offset":41536,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":561,"record_offset":41852,"parent_record_offset":41828,"ancestor_record_offset":41536,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":561,"record_offset":42040,"parent_record_offset":41852,"ancestor_record_offset":41536,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":561,"record_offset":41884,"parent_record_offset":41852,"ancestor_record_offset":41536,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":561,"record_offset":41912,"parent_record_offset":41884,"ancestor_record_offset":41536,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":41972,"parent_record_offset":41940,"ancestor_record_offset":41536,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":42072,"parent_record_offset":42040,"ancestor_record_offset":41536,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":41996,"parent_record_offset":41972,"ancestor_record_offset":41536,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":46892,"parent_record_offset":46864,"ancestor_record_offset":41536,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":46864,"parent_record_offset":46840,"ancestor_record_offset":41536,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":42120,"parent_record_offset":42096,"ancestor_record_offset":41536,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":42096,"parent_record_offset":42072,"ancestor_record_offset":41536,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":561,"record_offset":46840,"parent_record_offset":46816,"ancestor_record_offset":41536,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":561,"record_offset":44344,"parent_record_offset":44308,"ancestor_record_offset":41536,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":561,"record_offset":44308,"parent_record_offset":44284,"ancestor_record_offset":41536,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":42324,"parent_record_offset":42284,"ancestor_record_offset":41536,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":46472,"parent_record_offset":46424,"ancestor_record_offset":41536,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":46688,"parent_record_offset":46424,"ancestor_record_offset":41536,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":46744,"parent_record_offset":46720,"ancestor_record_offset":41536,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":46792,"parent_record_offset":46768,"ancestor_record_offset":41536,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":561,"record_offset":41768,"parent_record_offset":41740,"ancestor_record_offset":41536,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":43672,"parent_record_offset":43648,"ancestor_record_offset":41536,"inlinee":16296,"source_line":1742,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":43648,"parent_record_offset":43624,"ancestor_record_offset":41536,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":42980,"parent_record_offset":42952,"ancestor_record_offset":41536,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":43040,"parent_record_offset":43008,"ancestor_record_offset":41536,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":561,"record_offset":44060,"parent_record_offset":44016,"ancestor_record_offset":41536,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":46920,"parent_record_offset":46892,"ancestor_record_offset":41536,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":43256,"parent_record_offset":43232,"ancestor_record_offset":41536,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":42924,"parent_record_offset":42896,"ancestor_record_offset":41536,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":44260,"parent_record_offset":44232,"ancestor_record_offset":41536,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":42248,"parent_record_offset":42224,"ancestor_record_offset":41536,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":43476,"parent_record_offset":43452,"ancestor_record_offset":41536,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":42628,"parent_record_offset":42604,"ancestor_record_offset":41536,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":44376,"parent_record_offset":44344,"ancestor_record_offset":41536,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":44172,"parent_record_offset":44100,"ancestor_record_offset":41536,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":42548,"parent_record_offset":42504,"ancestor_record_offset":41536,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":44196,"parent_record_offset":44172,"ancestor_record_offset":41536,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":42436,"parent_record_offset":42284,"ancestor_record_offset":41536,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":44016,"parent_record_offset":43988,"ancestor_record_offset":41536,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":42224,"parent_record_offset":42200,"ancestor_record_offset":41536,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":42604,"parent_record_offset":42580,"ancestor_record_offset":41536,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":42504,"parent_record_offset":42468,"ancestor_record_offset":41536,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":42580,"parent_record_offset":42504,"ancestor_record_offset":41536,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":561,"record_offset":44404,"parent_record_offset":44344,"ancestor_record_offset":41536,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":561,"record_offset":44436,"parent_record_offset":44308,"ancestor_record_offset":41536,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_auto_clutter::replay_auto_clutter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_auto_clutter.rs
//     evidence={"module":561,"record_offset":43624,"parent_record_offset":43600,"ancestor_record_offset":41536,"inlinee":27167,"source_line":38,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":43200,"parent_record_offset":43168,"ancestor_record_offset":41536,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":43232,"parent_record_offset":43200,"ancestor_record_offset":41536,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":43168,"parent_record_offset":43120,"ancestor_record_offset":41536,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":42952,"parent_record_offset":42896,"ancestor_record_offset":41536,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":43008,"parent_record_offset":42952,"ancestor_record_offset":41536,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":43352,"parent_record_offset":43120,"ancestor_record_offset":41536,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":561,"record_offset":42776,"parent_record_offset":42752,"ancestor_record_offset":41536,"inlinee":17919,"source_line":917,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":43452,"parent_record_offset":43120,"ancestor_record_offset":41536,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":42896,"parent_record_offset":42836,"ancestor_record_offset":41536,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":43120,"parent_record_offset":43088,"ancestor_record_offset":41536,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":42808,"parent_record_offset":42776,"ancestor_record_offset":41536,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":42868,"parent_record_offset":42836,"ancestor_record_offset":41536,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":42836,"parent_record_offset":42808,"ancestor_record_offset":41536,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":561,"record_offset":43088,"parent_record_offset":42776,"ancestor_record_offset":41536,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":561,"record_offset":43748,"parent_record_offset":43724,"ancestor_record_offset":41536,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":561,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":41536,"inlinee":25285,"source_line":368,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$1::retain::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":561,"record_offset":43600,"parent_record_offset":42776,"ancestor_record_offset":41536,"inlinee":25287,"source_line":372,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_auto_clutter::replay_auto_clutter::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A1F
// 首条位置: {"module":490,"inlinee":27167}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":43648,"parent_record_offset":43624,"ancestor_record_offset":43624,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":43672,"parent_record_offset":43648,"ancestor_record_offset":43624,"inlinee":16296,"source_line":1742,"archive":"_pdb/B/modules/00561.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_auto_clutter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_auto_clutter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_auto_clutter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_auto_clutter::replay_auto_clutter
// 信息 ID: 58d2abf31bb78aef5218ca69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0103bc1d475ed50b624e8f2f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)> > @ 0xa9db49 -> 0xaa4a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)> > > @ 0x14d511b -> 0xaa4a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_auto_clutter::replay_auto_clutter::closure$0
// 信息 ID: 2269a32a11abefbceb91795d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_auto_clutter::replay_auto_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_auto_clutter::replay_auto_clutter; module=561, inline_record=43624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
