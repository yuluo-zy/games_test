// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/history/replay_pop_bucket.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fda63e4e61178362b796afb7e280acb0138abf1c43a93cf71cc6ae0da662ce34
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_pop_bucket.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 586, 1729
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
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.7.1"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_pop_bucket::replay_pop_bucket
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":27264,"rva":11764304,"va":5380473424}
// 编译实例 RVAs: 0xb38250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140894e90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2766269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e843cad5ff","size":5,"soft_fail":false,"target_va":5377707664,"va":5380473928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b50cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8ae880100","size":5,"soft_fail":false,"target_va":5380574400,"va":5380473869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b50d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e842890100","size":5,"soft_fail":false,"target_va":5380574560,"va":5380473881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t147420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8dc3f0200","size":5,"soft_fail":false,"target_va":5380621344,"va":5380473919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cb60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t149443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8c3470200","size":5,"soft_fail":false,"target_va":5380623200,"va":5380473752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b743e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t246005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8f5c00300","size":5,"soft_fail":false,"target_va":5380719584,"va":5380473574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14155ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10630822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8a636a200","size":5,"soft_fail":false,"target_va":5391104560,"va":5380473733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157cec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10767030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8b64aa400","size":5,"soft_fail":false,"target_va":5391240896,"va":5380473861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::pop_front
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1337878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8ea95ebff","size":5,"soft_fail":false,"target_va":5379135728,"va":5380473601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::apply_diff
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2324556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8b487dcff","size":5,"soft_fail":false,"target_va":5378149280,"va":5380473831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::make_backup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2331729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8af6bdcff","size":5,"soft_fail":false,"target_va":5378142176,"va":5380473900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::restore_backup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2331076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e83c6edcff","size":5,"soft_fail":false,"target_va":5378142736,"va":5380473807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_diff
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-47011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e85d48ffff","size":5,"soft_fail":false,"target_va":5380426544,"va":5380473550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::flush_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2535197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380473424,"raw_hex":"e8e350d9ff","size":5,"soft_fail":false,"target_va":5377938880,"va":5380474072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":586,"record_offset":28312,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":586,"record_offset":27852,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":586,"record_offset":27440,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":586,"record_offset":28136,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":27936,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":586,"record_offset":27496,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::StairsState::clear_backup
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":586,"record_offset":28988,"parent_record_offset":27264,"ancestor_record_offset":27264,"inlinee":28291,"source_line":129,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":586,"record_offset":27628,"parent_record_offset":27600,"ancestor_record_offset":27264,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":586,"record_offset":27520,"parent_record_offset":27496,"ancestor_record_offset":27264,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":586,"record_offset":27544,"parent_record_offset":27520,"ancestor_record_offset":27264,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":586,"record_offset":27728,"parent_record_offset":27544,"ancestor_record_offset":27264,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":586,"record_offset":27572,"parent_record_offset":27544,"ancestor_record_offset":27264,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":586,"record_offset":27600,"parent_record_offset":27572,"ancestor_record_offset":27264,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":27660,"parent_record_offset":27628,"ancestor_record_offset":27264,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":27752,"parent_record_offset":27728,"ancestor_record_offset":27264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":27684,"parent_record_offset":27660,"ancestor_record_offset":27264,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":27800,"parent_record_offset":27776,"ancestor_record_offset":27264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":586,"record_offset":27776,"parent_record_offset":27752,"ancestor_record_offset":27264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":586,"record_offset":29228,"parent_record_offset":29192,"ancestor_record_offset":27264,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":586,"record_offset":28544,"parent_record_offset":28520,"ancestor_record_offset":27264,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":586,"record_offset":27464,"parent_record_offset":27440,"ancestor_record_offset":27264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":586,"record_offset":28160,"parent_record_offset":28136,"ancestor_record_offset":27264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":586,"record_offset":29260,"parent_record_offset":29228,"ancestor_record_offset":27264,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":28188,"parent_record_offset":28160,"ancestor_record_offset":27264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":586,"record_offset":28216,"parent_record_offset":28160,"ancestor_record_offset":27264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":586,"record_offset":28580,"parent_record_offset":28544,"ancestor_record_offset":27264,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":586,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":27264,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":586,"record_offset":28636,"parent_record_offset":28544,"ancestor_record_offset":27264,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":586,"record_offset":28032,"parent_record_offset":28008,"ancestor_record_offset":27264,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map.rs
//     evidence={"module":586,"record_offset":29048,"parent_record_offset":29020,"ancestor_record_offset":27264,"inlinee":10619,"source_line":278,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":586,"record_offset":29020,"parent_record_offset":28988,"ancestor_record_offset":27264,"inlinee":10653,"source_line":286,"archive":"_pdb/B/modules/00586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13012
// 首条位置: {"module":1729,"inlinee":77842}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: replay_pop_bucket.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_pop_bucket.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_pop_bucket.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_pop_bucket::replay_pop_bucket
// 信息 ID: 809970086f22820f78cb6664
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] be1df2568c79dd5728dabb61: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)> > > @ 0x14fe80d -> 0xb38250 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)> > @ 0x1734d1b -> 0xb38250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
// 信息 ID: b83a891443c142e3dc8e7a8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_pop_bucket::replay_pop_bucket", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0 <- country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0 <- country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1729, inline_record=9892; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1729, inline_record=10608; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1729, inline_record=10672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
