// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/glade_themes/flowery.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fad8def2f499b5eafdb358b03b865c1964748d057ba027c44183e560f41e5b39
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flowery.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1615
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::glade_themes::flowery::startup_flowery
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":42396,"rva":22360352,"va":5391069472}
// 编译实例 RVAs: 0x1553120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: flowery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15988948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e82c070cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391070447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409365d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flowery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12701120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e840323eff","size":5,"soft_fail":false,"target_va":5378368976,"va":5391070091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0eeb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: flowery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10765034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e816bd5bff","size":5,"soft_fail":false,"target_va":5380304560,"va":5391069589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: flowery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8242314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e8763b82ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5391070037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flowery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20777610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e88a0a3d01","size":5,"soft_fail":false,"target_va":5411847808,"va":5391070193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flowery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20777957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e8e50b3d01","size":5,"soft_fail":false,"target_va":5411848144,"va":5391070182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20276628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e894653501","size":5,"soft_fail":false,"target_va":5411347040,"va":5391070407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8416490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e816937fff","size":5,"soft_fail":false,"target_va":5382655024,"va":5391071509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8645566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e842147cff","size":5,"soft_fail":false,"target_va":5382424224,"va":5391069785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20470715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e8bb5b3801","size":5,"soft_fail":false,"target_va":5411541776,"va":5391071056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20471833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e819603801","size":5,"soft_fail":false,"target_va":5411542880,"va":5391071042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12695447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e869483eff","size":5,"soft_fail":false,"target_va":5378374304,"va":5391069746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10716540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e8847a5cff","size":5,"soft_fail":false,"target_va":5380353184,"va":5391069719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8325954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e8bef480ff","size":5,"soft_fail":false,"target_va":5382744080,"va":5391070029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9136222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391069472,"raw_hex":"e8a29774ff","size":5,"soft_fail":false,"target_va":5381934464,"va":5391070681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1615,"record_offset":43028,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":42688,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":42652,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1615,"record_offset":43940,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1615,"record_offset":42580,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1615,"record_offset":43372,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1615,"record_offset":43792,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":26275,"source_line":75,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1615,"record_offset":43088,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":26641,"source_line":269,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":43340,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":43200,"parent_record_offset":42396,"ancestor_record_offset":42396,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1615,"record_offset":44148,"parent_record_offset":44124,"ancestor_record_offset":42396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1615,"record_offset":44124,"parent_record_offset":44036,"ancestor_record_offset":42396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":42872,"parent_record_offset":42844,"ancestor_record_offset":42396,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":42956,"parent_record_offset":42932,"ancestor_record_offset":42396,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":44064,"parent_record_offset":44036,"ancestor_record_offset":42396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":44036,"parent_record_offset":44012,"ancestor_record_offset":42396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":42932,"parent_record_offset":42900,"ancestor_record_offset":42396,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":42900,"parent_record_offset":42872,"ancestor_record_offset":42396,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":44012,"parent_record_offset":43988,"ancestor_record_offset":42396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":42844,"parent_record_offset":42808,"ancestor_record_offset":42396,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1615,"record_offset":43056,"parent_record_offset":43028,"ancestor_record_offset":42396,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":42776,"parent_record_offset":42748,"ancestor_record_offset":42396,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":42808,"parent_record_offset":42776,"ancestor_record_offset":42396,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":42720,"parent_record_offset":42688,"ancestor_record_offset":42396,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":42748,"parent_record_offset":42720,"ancestor_record_offset":42396,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1615,"record_offset":44092,"parent_record_offset":44064,"ancestor_record_offset":42396,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1615,"record_offset":42620,"parent_record_offset":42580,"ancestor_record_offset":42396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1615,"record_offset":43412,"parent_record_offset":43372,"ancestor_record_offset":42396,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1615,"record_offset":43136,"parent_record_offset":43112,"ancestor_record_offset":42396,"inlinee":26645,"source_line":53,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1615,"record_offset":43112,"parent_record_offset":43088,"ancestor_record_offset":42396,"inlinee":26643,"source_line":198,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1615,"record_offset":43844,"parent_record_offset":43820,"ancestor_record_offset":42396,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1615,"record_offset":43820,"parent_record_offset":43792,"ancestor_record_offset":42396,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1615,"record_offset":43892,"parent_record_offset":43868,"ancestor_record_offset":42396,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1615,"record_offset":43868,"parent_record_offset":43844,"ancestor_record_offset":42396,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1615,"record_offset":43248,"parent_record_offset":43224,"ancestor_record_offset":42396,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1615,"record_offset":43296,"parent_record_offset":43272,"ancestor_record_offset":42396,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1615,"record_offset":43272,"parent_record_offset":43248,"ancestor_record_offset":42396,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":43164,"parent_record_offset":43112,"ancestor_record_offset":42396,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":43224,"parent_record_offset":43200,"ancestor_record_offset":42396,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01615.jsonl.gz"}
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
// 完整结构及原始类型记录: flowery.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flowery.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flowery.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::startup::glade_themes::flowery::startup_flowery
// 信息 ID: d4eb730ebdb295285de2e13e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xa78df2 -> 0x1553120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
