// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/menu/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 46cf3fbd44ea581434cdafe33a6a5c734fa02511fc4dbab2a645acc0e4f090f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 339
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
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":339,"record_offset":1468,"rva":6872656,"va":5375581776}
// 编译实例 RVAs: 0x68de50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=9
//     target_aliases=53 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6703967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e8a1b499ff","size":5,"soft_fail":false,"target_va":5368879232,"va":5375583194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5966966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e88af3a4ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375583073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fdd640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9761690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e89af39400","size":5,"soft_fail":false,"target_va":5385344576,"va":5375582881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fddf80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9763990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e896fc9400","size":5,"soft_fail":false,"target_va":5385346944,"va":5375582949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ff3700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9852961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e821589600","size":5,"soft_fail":false,"target_va":5385434880,"va":5375581914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ff4d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9858643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e8536e9600","size":5,"soft_fail":false,"target_va":5385440576,"va":5375581928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ff69a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9865893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e8a58a9600","size":5,"soft_fail":false,"target_va":5385447840,"va":5375581942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9712911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e80f359400","size":5,"soft_fail":false,"target_va":5385294736,"va":5375581820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9718559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e81f4b9400","size":5,"soft_fail":false,"target_va":5385301456,"va":5375582892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9756613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e8c5df9400","size":5,"soft_fail":false,"target_va":5385339552,"va":5375582934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9849919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375581776,"raw_hex":"e83f4c9600","size":5,"soft_fail":false,"target_va":5385431824,"va":5375581900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t9812357","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375581776,"raw_hex":"e985b99500","size":5,"soft_fail":false,"target_va":5385395520,"va":5375583158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":339,"record_offset":1804,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":339,"record_offset":1676,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":339,"record_offset":1704,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":339,"record_offset":1600,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":20114,"source_line":95,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":339,"record_offset":1628,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":20116,"source_line":116,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$24::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":339,"record_offset":1652,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":20118,"source_line":123,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":339,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":339,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":339,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":339,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1468,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":339,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":1468,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":339,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1468,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":339,"record_offset":1832,"parent_record_offset":1804,"ancestor_record_offset":1468,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":339,"record_offset":1744,"parent_record_offset":1704,"ancestor_record_offset":1468,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":339,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":1468,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":339,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1468,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":339,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1468,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":339,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":1468,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":339,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1468,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":339,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":1468,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":339,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":1468,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":339,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":1468,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":339,"record_offset":2084,"parent_record_offset":2020,"ancestor_record_offset":1468,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":339,"record_offset":1892,"parent_record_offset":1860,"ancestor_record_offset":1468,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":339,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1468,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":339,"record_offset":2396,"parent_record_offset":1860,"ancestor_record_offset":1468,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":339,"record_offset":2148,"parent_record_offset":1968,"ancestor_record_offset":1468,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":339,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1468,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":339,"record_offset":2056,"parent_record_offset":2020,"ancestor_record_offset":1468,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":339,"record_offset":2524,"rva":6874480,"va":5375583600}
// 编译实例 RVAs: 0x68e570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9717811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375583600,"raw_hex":"e833489400","size":5,"soft_fail":false,"target_va":5385301456,"va":5375583640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9885902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375583600,"raw_hex":"e8ced89600","size":5,"soft_fail":false,"target_va":5385469536,"va":5375583629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":339,"record_offset":2636,"parent_record_offset":2524,"ancestor_record_offset":2524,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::advance_menu_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":339,"record_offset":2668,"rva":6874544,"va":5375583664}
// 编译实例 RVAs: 0x68e5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2822933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375583664,"raw_hex":"e815132b00","size":5,"soft_fail":false,"target_va":5378407120,"va":5375584182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d090
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5040235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375583664,"raw_hex":"e86be84c00","size":5,"soft_fail":false,"target_va":5380624528,"va":5375584288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1228378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375583664,"raw_hex":"e8a641edff","size":5,"soft_fail":false,"target_va":5374355568,"va":5375583941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":339,"record_offset":2804,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":339,"record_offset":2980,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":339,"record_offset":3468,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":339,"record_offset":2864,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":339,"record_offset":3556,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":339,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2668,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":339,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":2668,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":339,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":2668,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":339,"record_offset":3668,"parent_record_offset":3632,"ancestor_record_offset":2668,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":339,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2668,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":339,"record_offset":3188,"parent_record_offset":3148,"ancestor_record_offset":2668,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":339,"record_offset":3148,"parent_record_offset":3112,"ancestor_record_offset":2668,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":339,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":2668,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":339,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2668,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":339,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2668,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":339,"record_offset":3064,"parent_record_offset":2980,"ancestor_record_offset":2668,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":339,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2668,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":339,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":2668,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":339,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2668,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":339,"record_offset":3700,"parent_record_offset":3668,"ancestor_record_offset":2668,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":339,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":2668,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":339,"record_offset":2944,"parent_record_offset":2888,"ancestor_record_offset":2668,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":339,"record_offset":3728,"parent_record_offset":3668,"ancestor_record_offset":2668,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":339,"record_offset":3760,"parent_record_offset":3632,"ancestor_record_offset":2668,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00339.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::MenuFlowPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::MenuFlowPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$15::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::SplashScreenPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::SplashScreenPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreen", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>", "bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>

// ------------------------------------------------------------------
// 函数: toolbar::menu::advance_menu_state
// 信息 ID: 103f410f3ba5a1e52f9bca25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d4836eb0f181820b264feac9: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)> > @ 0xffae57 -> 0x68e5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustClosed>)> > > @ 0x1e0ebc9 -> 0x68e5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::impl$0::build
// 信息 ID: 2d448f596d7be0052bf1369e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::impl$1::build
// 信息 ID: c81b25ffc2d5cb4022d62b1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
