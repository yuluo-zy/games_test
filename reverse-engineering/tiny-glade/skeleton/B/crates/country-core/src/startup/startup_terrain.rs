// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/startup_terrain.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fb7c50de9e026606dac2b88bbb00a23b469d5f100f1850088efe1644d9e3957a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 507, 1553
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::startup_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":24404,"rva":9595888,"va":5378305008}
// 编译实例 RVAs: 0x926bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8704287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8e12e7bff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378305082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-651941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e85b0df6ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5378309584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140928100
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e82f140000","size":5,"soft_fail":false,"target_va":5378310400,"va":5378305228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0eeb0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1998023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8c77c1e00","size":5,"soft_fail":false,"target_va":5380304560,"va":5378306532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2199706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e89a902100","size":5,"soft_fail":false,"target_va":5380505152,"va":5378305441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t33038297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8d91ff801","size":5,"soft_fail":false,"target_va":5411347040,"va":5378308738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4345339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8fb4d4200","size":5,"soft_fail":false,"target_va":5382655024,"va":5378309680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4118763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8ebd83e00","size":5,"soft_fail":false,"target_va":5382424224,"va":5378305456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33232352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8e015fb01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378309419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t33233682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8121bfb01","size":5,"soft_fail":false,"target_va":5411542880,"va":5378309193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t66536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8e8030100","size":5,"soft_fail":false,"target_va":5378374304,"va":5378307763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::rocky_terrain::RockyTerrainMesh::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-438561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8df4ef9ff","size":5,"soft_fail":false,"target_va":5377867600,"va":5378306156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2047811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8433f1f00","size":5,"soft_fail":false,"target_va":5380353184,"va":5378305368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33227950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378305008,"raw_hex":"e8ae04fb01","size":5,"soft_fail":false,"target_va":5411537104,"va":5378309149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":507,"record_offset":26348,"parent_record_offset":24404,"ancestor_record_offset":24404,"inlinee":9431,"source_line":851,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":507,"record_offset":25040,"parent_record_offset":24404,"ancestor_record_offset":24404,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":507,"record_offset":28724,"parent_record_offset":24404,"ancestor_record_offset":24404,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":507,"record_offset":26276,"parent_record_offset":24404,"ancestor_record_offset":24404,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":507,"record_offset":24984,"parent_record_offset":24404,"ancestor_record_offset":24404,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid_cw
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":507,"record_offset":24564,"parent_record_offset":24404,"ancestor_record_offset":24404,"inlinee":28841,"source_line":264,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":25224,"parent_record_offset":25196,"ancestor_record_offset":24404,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":25308,"parent_record_offset":25284,"ancestor_record_offset":24404,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":25284,"parent_record_offset":25252,"ancestor_record_offset":24404,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":25252,"parent_record_offset":25224,"ancestor_record_offset":24404,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":507,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":24404,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":25196,"parent_record_offset":25160,"ancestor_record_offset":24404,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":507,"record_offset":25128,"parent_record_offset":25100,"ancestor_record_offset":24404,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":507,"record_offset":25160,"parent_record_offset":25128,"ancestor_record_offset":24404,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":507,"record_offset":25072,"parent_record_offset":25040,"ancestor_record_offset":24404,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":507,"record_offset":25100,"parent_record_offset":25072,"ancestor_record_offset":24404,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":507,"record_offset":24812,"parent_record_offset":24788,"ancestor_record_offset":24404,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":507,"record_offset":24864,"parent_record_offset":24840,"ancestor_record_offset":24404,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":507,"record_offset":24840,"parent_record_offset":24788,"ancestor_record_offset":24404,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":507,"record_offset":24888,"parent_record_offset":24864,"ancestor_record_offset":24404,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":507,"record_offset":24760,"parent_record_offset":24732,"ancestor_record_offset":24404,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":26316,"parent_record_offset":26276,"ancestor_record_offset":24404,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":24404,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":507,"record_offset":24608,"parent_record_offset":24564,"ancestor_record_offset":24404,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":24636,"parent_record_offset":24608,"ancestor_record_offset":24404,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":507,"record_offset":24684,"parent_record_offset":24660,"ancestor_record_offset":24404,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":507,"record_offset":24788,"parent_record_offset":24732,"ancestor_record_offset":24404,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":507,"record_offset":24660,"parent_record_offset":24636,"ancestor_record_offset":24404,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":507,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":24404,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":507,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":24404,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::generate_grid_cw
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70A9
// 首条位置: {"module":507,"inlinee":28841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":507,"record_offset":24608,"parent_record_offset":24564,"ancestor_record_offset":24564,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":507,"record_offset":24812,"parent_record_offset":24788,"ancestor_record_offset":24564,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":507,"record_offset":24864,"parent_record_offset":24840,"ancestor_record_offset":24564,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":507,"record_offset":24840,"parent_record_offset":24788,"ancestor_record_offset":24564,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":507,"record_offset":24888,"parent_record_offset":24864,"ancestor_record_offset":24564,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":507,"record_offset":24760,"parent_record_offset":24732,"ancestor_record_offset":24564,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":24564,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":507,"record_offset":24636,"parent_record_offset":24608,"ancestor_record_offset":24564,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":507,"record_offset":24684,"parent_record_offset":24660,"ancestor_record_offset":24564,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":507,"record_offset":24788,"parent_record_offset":24732,"ancestor_record_offset":24564,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":507,"record_offset":24660,"parent_record_offset":24636,"ancestor_record_offset":24564,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":507,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":24564,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":507,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":24564,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::grid_gen_cw
// 记录形式: compiled_procedure; instances=1
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":30196,"rva":9601280,"va":5378310400}
// 编译实例 RVAs: 0x928100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140928100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378310400,"raw_hex":"e8a2ffffff","size":5,"soft_fail":false,"target_va":5378310400,"va":5378310489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t14156115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378310400,"raw_hex":"e85301d800","size":5,"soft_fail":false,"target_va":5392466848,"va":5378310728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33230797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378310400,"raw_hex":"e8cd0ffb01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378310974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":30532,"parent_record_offset":30196,"ancestor_record_offset":30196,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":30412,"parent_record_offset":30196,"ancestor_record_offset":30196,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":507,"record_offset":30440,"parent_record_offset":30196,"ancestor_record_offset":30196,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":30348,"parent_record_offset":30196,"ancestor_record_offset":30196,"inlinee":15138,"source_line":653,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":507,"record_offset":30376,"parent_record_offset":30196,"ancestor_record_offset":30196,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":30196,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":30644,"parent_record_offset":30620,"ancestor_record_offset":30196,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":30620,"parent_record_offset":30596,"ancestor_record_offset":30196,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":30572,"parent_record_offset":30532,"ancestor_record_offset":30196,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":30496,"parent_record_offset":30468,"ancestor_record_offset":30196,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":507,"record_offset":30468,"parent_record_offset":30440,"ancestor_record_offset":30196,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":30684,"parent_record_offset":30532,"ancestor_record_offset":30196,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::generate_grid_ccw
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70CC
// 首条位置: {"module":1553,"inlinee":28876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1553,"record_offset":23752,"parent_record_offset":23708,"ancestor_record_offset":23708,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":23960,"parent_record_offset":23932,"ancestor_record_offset":23708,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":23708,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":23988,"parent_record_offset":23932,"ancestor_record_offset":23708,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23708,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1553,"record_offset":23904,"parent_record_offset":23876,"ancestor_record_offset":23708,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":24060,"parent_record_offset":24036,"ancestor_record_offset":23708,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":23780,"parent_record_offset":23752,"ancestor_record_offset":23708,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":23828,"parent_record_offset":23804,"ancestor_record_offset":23708,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":23932,"parent_record_offset":23876,"ancestor_record_offset":23708,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":23804,"parent_record_offset":23780,"ancestor_record_offset":23708,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":23876,"parent_record_offset":23852,"ancestor_record_offset":23708,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":23852,"parent_record_offset":23828,"ancestor_record_offset":23708,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::grid_gen_ccw
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":31696,"rva":9601872,"va":5378310992}
// 编译实例 RVAs: 0x928350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140928350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378310992,"raw_hex":"e8a2ffffff","size":5,"soft_fail":false,"target_va":5378310992,"va":5378311081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t14155523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378310992,"raw_hex":"e803ffd700","size":5,"soft_fail":false,"target_va":5392466848,"va":5378311320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33230206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378310992,"raw_hex":"e87e0dfb01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378311565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":32032,"parent_record_offset":31696,"ancestor_record_offset":31696,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":31912,"parent_record_offset":31696,"ancestor_record_offset":31696,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":507,"record_offset":31940,"parent_record_offset":31696,"ancestor_record_offset":31696,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":31848,"parent_record_offset":31696,"ancestor_record_offset":31696,"inlinee":15138,"source_line":653,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":507,"record_offset":31876,"parent_record_offset":31696,"ancestor_record_offset":31696,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":32096,"parent_record_offset":32072,"ancestor_record_offset":31696,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":32144,"parent_record_offset":32120,"ancestor_record_offset":31696,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":507,"record_offset":32120,"parent_record_offset":32096,"ancestor_record_offset":31696,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":32072,"parent_record_offset":32032,"ancestor_record_offset":31696,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":31996,"parent_record_offset":31968,"ancestor_record_offset":31696,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":507,"record_offset":31968,"parent_record_offset":31940,"ancestor_record_offset":31696,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":32184,"parent_record_offset":32032,"ancestor_record_offset":31696,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00507.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::generate_grid_ccw
// 信息 ID: d292fbb94e23993bcfe4ec85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=23708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::generate_grid_cw
// 信息 ID: 3a09acb6687c4fb400f571e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_terrain; module=507, inline_record=24564; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_terrain; module=507, inline_record=25380; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_terrain; module=507, inline_record=30024; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=22060; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1553, inline_record=28220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::grid_gen_ccw
// 信息 ID: fe72c62e7d2e1c6c2ba8f8a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_terrain::grid_gen_ccw @ 0x9283a9 -> 0x928350 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0x129fa36 -> 0x928350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::grid_gen_cw
// 信息 ID: 295b206008d73b17b09ab783
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_terrain::startup_terrain @ 0x926ccc -> 0x928100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::startup_terrain @ 0x926ee3 -> 0x928100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::grid_gen_cw @ 0x928159 -> 0x928100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0x129f716 -> 0x928100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0x12a0005 -> 0x928100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::startup_terrain
// 信息 ID: d3dbeb41e05d854eb31ee3ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a55b28c5dd0b0b6191734f41: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > > @ 0x14c0f69 -> 0x926bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0x1768894 -> 0x926bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
