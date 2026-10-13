// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/history/replay_pop_bucket.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 29b7ac3a97727501cb683f861c776de6d44c23cb18b26ed7b42d708457532d18
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_pop_bucket.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1439, 1447
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_pop_bucket::replay_pop_bucket
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1447,"record_offset":15680,"rva":17859632,"va":5386568752}
// 编译实例 RVAs: 0x1108430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bce80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14988871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e8b9491bff","size":5,"soft_fail":false,"target_va":5371580032,"va":5386568898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cea40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14916408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e8c8641cff","size":5,"soft_fail":false,"target_va":5371652672,"va":5386569075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dff80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11699685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e81b7a4dff","size":5,"soft_fail":false,"target_va":5374869376,"va":5386569056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ed690
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_pop_bucket.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10596090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e806515eff","size":5,"soft_fail":false,"target_va":5375973008,"va":5386569093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::pop_front
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2363887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e811eedbff","size":5,"soft_fail":false,"target_va":5384205040,"va":5386568922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_diff
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11852847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e8d1234bff","size":5,"soft_fail":false,"target_va":5374716032,"va":5386568874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::flush_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12057336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386568752,"raw_hex":"e8080548ff","size":5,"soft_fail":false,"target_va":5374511872,"va":5386569203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1447,"record_offset":16576,"parent_record_offset":15680,"ancestor_record_offset":15680,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1447,"record_offset":16136,"parent_record_offset":15680,"ancestor_record_offset":15680,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1447,"record_offset":15852,"parent_record_offset":15680,"ancestor_record_offset":15680,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1447,"record_offset":16400,"parent_record_offset":15680,"ancestor_record_offset":15680,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":16220,"parent_record_offset":15680,"ancestor_record_offset":15680,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":1447,"record_offset":15908,"parent_record_offset":15680,"ancestor_record_offset":15680,"inlinee":23761,"source_line":165,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1447,"record_offset":15932,"parent_record_offset":15908,"ancestor_record_offset":15680,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1447,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":15680,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1447,"record_offset":15984,"parent_record_offset":15956,"ancestor_record_offset":15680,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1447,"record_offset":16040,"parent_record_offset":15956,"ancestor_record_offset":15680,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1447,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":15680,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1447,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":15680,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":15680,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1447,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":15680,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1447,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":15680,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1447,"record_offset":16424,"parent_record_offset":16400,"ancestor_record_offset":15680,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":16240,"parent_record_offset":16220,"ancestor_record_offset":15680,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":16452,"parent_record_offset":16424,"ancestor_record_offset":15680,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":16480,"parent_record_offset":16424,"ancestor_record_offset":15680,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1447,"record_offset":16844,"parent_record_offset":16808,"ancestor_record_offset":15680,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1447,"record_offset":16868,"parent_record_offset":16844,"ancestor_record_offset":15680,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1447,"record_offset":16900,"parent_record_offset":16808,"ancestor_record_offset":15680,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":16300,"parent_record_offset":16280,"ancestor_record_offset":15680,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10408
// 首条位置: {"module":1439,"inlinee":66568}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::DeferredDecoratorOpEvents::send
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1439,"record_offset":10688,"parent_record_offset":10652,"ancestor_record_offset":10652,"inlinee":24244,"source_line":430,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1439,"record_offset":10968,"parent_record_offset":10936,"ancestor_record_offset":10652,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":10936,"parent_record_offset":10896,"ancestor_record_offset":10652,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1439,"record_offset":10896,"parent_record_offset":10688,"ancestor_record_offset":10652,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":11032,"parent_record_offset":10688,"ancestor_record_offset":10652,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1439,"record_offset":11000,"parent_record_offset":10896,"ancestor_record_offset":10652,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":10832,"parent_record_offset":10748,"ancestor_record_offset":10652,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1439,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":10652,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":11112,"parent_record_offset":11080,"ancestor_record_offset":10652,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":10776,"parent_record_offset":10748,"ancestor_record_offset":10652,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":10652,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1439,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":10652,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1439,"record_offset":10724,"parent_record_offset":10688,"ancestor_record_offset":10652,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1439,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":10652,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1439,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":10652,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01439.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1439,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":10652,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01439.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_pop_bucket::replay_pop_bucket
// 信息 ID: 708c511211f43109ca643cba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9ceb8591527b18138539230a: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)> @ 0x60d95e -> 0x1108430 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)> > @ 0xe40950 -> 0x1108430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0
// 信息 ID: 25b0e7a2264ea623e6a19093
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_pop_bucket::replay_pop_bucket", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0 <- country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::history::decorator_edit::DecoratorHistoryState> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,country_core::resources::history::decorator_edit::impl$8::restore_decorators::closure_env$0<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0 <- country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1439, inline_record=10652; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1439, inline_record=11392; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::decorator_edit::send_state_diff_events<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>; module=1439, inline_record=12124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
