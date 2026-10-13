// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/onboarding/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e296d5f8d28eeb33918c0068ba9b3a0b02a2b00947dc5cb9a2017e58a12d5c5a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 306, 1047, 2507, 2508, 2521, 3718
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use system_onboarding; // namespace candidate
//   package identities: [{"name":"system-onboarding","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::impl$0::add_onboarding
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":306,"record_offset":1476,"rva":6622544,"va":5375331664}
// 编译实例 RVAs: 0x650d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=3
//     target_aliases=53 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6455165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e883809dff","size":5,"soft_fail":false,"target_va":5368879232,"va":5375334392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=22
//     target_aliases=19 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5715617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e85fc9a8ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375331724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8758905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e879a68500","size":5,"soft_fail":false,"target_va":5384091168,"va":5375332258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8835165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e85dd08600","size":5,"soft_fail":false,"target_va":5384167552,"va":5375332382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8818740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e834908600","size":5,"soft_fail":false,"target_va":5384150896,"va":5375332151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8775305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e889e68500","size":5,"soft_fail":false,"target_va":5384107136,"va":5375331826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8808916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e8d4698600","size":5,"soft_fail":false,"target_va":5384141200,"va":5375332279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8818530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e8628f8600","size":5,"soft_fail":false,"target_va":5384150704,"va":5375332169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8767282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e832c78500","size":5,"soft_fail":false,"target_va":5384099104,"va":5375331817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36207547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331664,"raw_hex":"e8bb7b2802","size":5,"soft_fail":false,"target_va":5411541904,"va":5375334352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":306,"record_offset":2888,"parent_record_offset":1476,"ancestor_record_offset":1476,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":306,"record_offset":1612,"parent_record_offset":1476,"ancestor_record_offset":1476,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":306,"record_offset":2760,"parent_record_offset":1476,"ancestor_record_offset":1476,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":306,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1476,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":306,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1476,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":306,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1476,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":306,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1476,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":306,"record_offset":2120,"parent_record_offset":2080,"ancestor_record_offset":1476,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":306,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1476,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":306,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":1476,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":306,"record_offset":1644,"parent_record_offset":1612,"ancestor_record_offset":1476,"inlinee":9172,"source_line":211,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":306,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":1476,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":306,"record_offset":2696,"parent_record_offset":1644,"ancestor_record_offset":1476,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":306,"record_offset":2800,"parent_record_offset":2760,"ancestor_record_offset":1476,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":306,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":1476,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":306,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1476,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":306,"record_offset":2392,"parent_record_offset":2360,"ancestor_record_offset":1476,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":306,"record_offset":2336,"parent_record_offset":1804,"ancestor_record_offset":1476,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":306,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1476,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":306,"record_offset":1836,"parent_record_offset":1804,"ancestor_record_offset":1476,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":306,"record_offset":2420,"parent_record_offset":2360,"ancestor_record_offset":1476,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":306,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1476,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":306,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1476,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":306,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1476,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":306,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":1476,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":306,"record_offset":1716,"parent_record_offset":1680,"ancestor_record_offset":1476,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":306,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1476,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":306,"record_offset":1680,"parent_record_offset":1644,"ancestor_record_offset":1476,"inlinee":9173,"source_line":2000,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":306,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":1476,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":306,"record_offset":2512,"parent_record_offset":1804,"ancestor_record_offset":1476,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":306,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":1476,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":306,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1476,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":306,"record_offset":1804,"parent_record_offset":1740,"ancestor_record_offset":1476,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":306,"record_offset":2636,"parent_record_offset":1680,"ancestor_record_offset":1476,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":306,"record_offset":2664,"parent_record_offset":1680,"ancestor_record_offset":1476,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":306,"record_offset":1972,"parent_record_offset":1912,"ancestor_record_offset":1476,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":306,"record_offset":2972,"parent_record_offset":2940,"ancestor_record_offset":1476,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":306,"record_offset":1992,"parent_record_offset":1972,"ancestor_record_offset":1476,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":306,"record_offset":3476,"parent_record_offset":2940,"ancestor_record_offset":1476,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":306,"record_offset":2032,"parent_record_offset":1860,"ancestor_record_offset":1476,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":306,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":1476,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":306,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":1476,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":306,"record_offset":1944,"parent_record_offset":1912,"ancestor_record_offset":1476,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":306,"record_offset":2260,"parent_record_offset":2080,"ancestor_record_offset":1476,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":306,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1476,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":306,"record_offset":2584,"parent_record_offset":2560,"ancestor_record_offset":1476,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::prompt_rollercoaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3718,"record_offset":264,"rva":36158320,"va":5404867440}
// 编译实例 RVAs: 0x227bb70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e8d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25767520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404867440,"raw_hex":"e8a0d176fe","size":5,"soft_fail":false,"target_va":5379100048,"va":5404867563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eaaa50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20779724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404867440,"raw_hex":"e834edc2fe","size":5,"soft_fail":false,"target_va":5384088144,"va":5404867863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c4e4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6477619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404867440,"raw_hex":"e8cd289dff","size":5,"soft_fail":false,"target_va":5398390000,"va":5404867614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30510456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404867440,"raw_hex":"e888722efe","size":5,"soft_fail":false,"target_va":5374357296,"va":5404867747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3718,"record_offset":964,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3718,"record_offset":400,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3718,"record_offset":1208,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_orchestrator::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_orchestrator.rs
//     evidence={"module":3718,"record_offset":1152,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":97646,"source_line":353,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3718,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3718,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3718,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3718,"record_offset":1400,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3718,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3718,"record_offset":1348,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3718,"record_offset":612,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3718,"record_offset":1276,"parent_record_offset":1236,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3718,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3718,"record_offset":1304,"parent_record_offset":1236,"ancestor_record_offset":264,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3718,"record_offset":480,"parent_record_offset":432,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3718,"record_offset":512,"parent_record_offset":480,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3718,"record_offset":996,"parent_record_offset":964,"ancestor_record_offset":264,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3718,"record_offset":432,"parent_record_offset":400,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3718,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3718,"record_offset":1028,"parent_record_offset":996,"ancestor_record_offset":264,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3718,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3718,"record_offset":556,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3718,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3718,"record_offset":1116,"parent_record_offset":996,"ancestor_record_offset":264,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3718,"record_offset":1548,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3718,"record_offset":1520,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3718,"record_offset":668,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3718,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03718.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3718,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03718.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 31; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>", "tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>", "bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>,bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>", "bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>", "bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<system_onboarding::hint_orchestrator::HintType>,tuple$<> >,bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::system::query::Query<ref$<system_onboarding::hint_orchestrator::HintType>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_onboarding::hint_orchestrator::HintType>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>", "bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_orchestrator::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_orchestrator::HintPlacement> > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::onboarding::NotifyHintSystemCameraMoved> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >", "bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::onboarding::NotifyHintSystemCameraMoved> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::onboarding::NotifyHintSystemCameraMoved> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<system_onboarding::hint_display_system::HintAnimation>,enum2$<core::option::Option<ref$<system_onboarding::hint_display_system::HintMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<system_onboarding::hint_display_system::HintAnimation>,enum2$<core::option::Option<ref$<system_onboarding::hint_display_system::HintMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<system_onboarding::hint_display_system::HintAnimation>,enum2$<core::option::Option<ref$<system_onboarding::hint_display_system::HintMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::system::system_param::Local<system_onboarding::exponential_back_off::TimeSinceMouseMoved>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::system::system_param::Local<system_onboarding::exponential_back_off::TimeSinceMouseMoved>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>", "bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_onboarding::exponential_back_off::TimeSinceMouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_onboarding::impl$0::add_onboarding
// 信息 ID: ac42f342aa8ea5eb26611dcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a2178 -> 0x650d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::prompt_rollercoaster
// 信息 ID: 8649e4af2b9fb2fdbb52793d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1478f56fd60dff30029a310e: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)> > > @ 0x1c2c4ea -> 0x227bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)> > @ 0x1c4889c -> 0x227bb70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
