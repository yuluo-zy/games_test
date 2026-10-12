// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/startup_nani.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5f5fc98f3456b4dd3817d6af332dd9c81b87d5741a2af49431316d87c2e8b32a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_nani.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 544
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
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::startup_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":67296,"rva":10716688,"va":5379425808}
// 编译实例 RVAs: 0xa38610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4348358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e83aa6bdff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379429857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4348222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e8c2a6bdff","size":5,"soft_fail":false,"target_va":5375081536,"va":5379429753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2654255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e8d17fd7ff","size":5,"soft_fail":false,"target_va":5376776208,"va":5379430458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891810
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1735661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e81384e5ff","size":5,"soft_fail":false,"target_va":5377693712,"va":5379429368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1077635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e883711000","size":5,"soft_fail":false,"target_va":5380505152,"va":5379427512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1793031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e8075c1b00","size":5,"soft_fail":false,"target_va":5381222304,"va":5379429268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412badf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8919083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e82b188800","size":5,"soft_fail":false,"target_va":5388348912,"va":5379429824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412bb570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8921024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e8c01f8800","size":5,"soft_fail":false,"target_va":5388350832,"va":5379429803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32417846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e836a8ee01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379429317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31917125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e84504e701","size":5,"soft_fail":false,"target_va":5411347040,"va":5379429910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32110725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e885f8e901","size":5,"soft_fail":false,"target_va":5411540963,"va":5379430233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2995689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e8e9b52d00","size":5,"soft_fail":false,"target_va":5382424224,"va":5379428530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32111885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e80dfde901","size":5,"soft_fail":false,"target_va":5411541904,"va":5379430014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32112664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e81800ea01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379430211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1054188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e814eaefff","size":5,"soft_fail":false,"target_va":5378374304,"va":5379428487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t710361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379425808,"raw_hex":"e8d9d60a00","size":5,"soft_fail":false,"target_va":5380138832,"va":5379428466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379425808,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379429130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":69440,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":544,"record_offset":68888,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":544,"record_offset":67804,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":68292,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":69724,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":544,"record_offset":67748,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::assign_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":69296,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":29951,"source_line":497,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":68944,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":29952,"source_line":501,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":68112,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":29945,"source_line":173,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::create_world_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":544,"record_offset":68384,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":31216,"source_line":166,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::runtime_check_limits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":544,"record_offset":67476,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":31215,"source_line":153,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":544,"record_offset":68048,"parent_record_offset":67296,"ancestor_record_offset":67296,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":69644,"parent_record_offset":69620,"ancestor_record_offset":67296,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":69668,"parent_record_offset":69644,"ancestor_record_offset":67296,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":69908,"parent_record_offset":69884,"ancestor_record_offset":67296,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":69620,"parent_record_offset":69584,"ancestor_record_offset":67296,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":69884,"parent_record_offset":69796,"ancestor_record_offset":67296,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68648,"parent_record_offset":68620,"ancestor_record_offset":67296,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":69512,"parent_record_offset":69476,"ancestor_record_offset":67296,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68732,"parent_record_offset":68708,"ancestor_record_offset":67296,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":69824,"parent_record_offset":69796,"ancestor_record_offset":67296,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":69796,"parent_record_offset":69772,"ancestor_record_offset":67296,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68708,"parent_record_offset":68676,"ancestor_record_offset":67296,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68676,"parent_record_offset":68648,"ancestor_record_offset":67296,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":69584,"parent_record_offset":69548,"ancestor_record_offset":67296,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":69548,"parent_record_offset":69512,"ancestor_record_offset":67296,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":69772,"parent_record_offset":69748,"ancestor_record_offset":67296,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":544,"record_offset":67772,"parent_record_offset":67748,"ancestor_record_offset":67296,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":68620,"parent_record_offset":68584,"ancestor_record_offset":67296,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":544,"record_offset":69476,"parent_record_offset":69440,"ancestor_record_offset":67296,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":544,"record_offset":68912,"parent_record_offset":68888,"ancestor_record_offset":67296,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68552,"parent_record_offset":68524,"ancestor_record_offset":67296,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68584,"parent_record_offset":68552,"ancestor_record_offset":67296,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68464,"parent_record_offset":68384,"ancestor_record_offset":67296,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68496,"parent_record_offset":68464,"ancestor_record_offset":67296,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68524,"parent_record_offset":68496,"ancestor_record_offset":67296,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":544,"record_offset":67912,"parent_record_offset":67836,"ancestor_record_offset":67296,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":69348,"parent_record_offset":69324,"ancestor_record_offset":67296,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":544,"record_offset":68348,"parent_record_offset":68324,"ancestor_record_offset":67296,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":544,"record_offset":67940,"parent_record_offset":67912,"ancestor_record_offset":67296,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":544,"record_offset":69128,"parent_record_offset":69104,"ancestor_record_offset":67296,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":69852,"parent_record_offset":69824,"ancestor_record_offset":67296,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":544,"record_offset":67884,"parent_record_offset":67836,"ancestor_record_offset":67296,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":544,"record_offset":67836,"parent_record_offset":67804,"ancestor_record_offset":67296,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":67580,"parent_record_offset":67536,"ancestor_record_offset":67296,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":67968,"parent_record_offset":67940,"ancestor_record_offset":67296,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":69380,"parent_record_offset":69296,"ancestor_record_offset":67296,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":67996,"parent_record_offset":67968,"ancestor_record_offset":67296,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":69160,"parent_record_offset":69024,"ancestor_record_offset":67296,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":69204,"parent_record_offset":68944,"ancestor_record_offset":67296,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":544,"record_offset":69076,"parent_record_offset":69024,"ancestor_record_offset":67296,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::gpu_world_subset_desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":68804,"parent_record_offset":68384,"ancestor_record_offset":67296,"inlinee":29817,"source_line":274,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":67652,"parent_record_offset":67476,"ancestor_record_offset":67296,"inlinee":27332,"source_line":208,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":67608,"parent_record_offset":67536,"ancestor_record_offset":67296,"inlinee":27330,"source_line":43,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":544,"record_offset":68988,"parent_record_offset":68944,"ancestor_record_offset":67296,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":544,"record_offset":69024,"parent_record_offset":68988,"ancestor_record_offset":67296,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_nani::startup_nani::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":544,"record_offset":68324,"parent_record_offset":68292,"ancestor_record_offset":67296,"inlinee":31218,"source_line":117,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":544,"record_offset":67536,"parent_record_offset":67476,"ancestor_record_offset":67296,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":544,"record_offset":69324,"parent_record_offset":69296,"ancestor_record_offset":67296,"inlinee":25969,"source_line":507,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":544,"record_offset":69104,"parent_record_offset":69024,"ancestor_record_offset":67296,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::startup_nani::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79F2
// 首条位置: {"module":544,"inlinee":31218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":544,"record_offset":68348,"parent_record_offset":68324,"ancestor_record_offset":68324,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::runtime_check_limits
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79EF
// 首条位置: {"module":544,"inlinee":31215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":67652,"parent_record_offset":67476,"ancestor_record_offset":67476,"inlinee":27332,"source_line":208,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":544,"record_offset":67536,"parent_record_offset":67476,"ancestor_record_offset":67476,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":67580,"parent_record_offset":67536,"ancestor_record_offset":67476,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":67608,"parent_record_offset":67536,"ancestor_record_offset":67476,"inlinee":27330,"source_line":43,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::create_world_subset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79F0
// 首条位置: {"module":544,"inlinee":31216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68464,"parent_record_offset":68384,"ancestor_record_offset":68384,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::gpu_world_subset_desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":68804,"parent_record_offset":68384,"ancestor_record_offset":68384,"inlinee":29817,"source_line":274,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":68832,"parent_record_offset":68384,"ancestor_record_offset":68384,"inlinee":27332,"source_line":208,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68648,"parent_record_offset":68620,"ancestor_record_offset":68384,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68732,"parent_record_offset":68708,"ancestor_record_offset":68384,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68708,"parent_record_offset":68676,"ancestor_record_offset":68384,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":68676,"parent_record_offset":68648,"ancestor_record_offset":68384,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":68620,"parent_record_offset":68584,"ancestor_record_offset":68384,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68552,"parent_record_offset":68524,"ancestor_record_offset":68384,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68584,"parent_record_offset":68552,"ancestor_record_offset":68384,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68496,"parent_record_offset":68464,"ancestor_record_offset":68384,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":544,"record_offset":68524,"parent_record_offset":68496,"ancestor_record_offset":68384,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::reload_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":70416,"rva":10721456,"va":5379430576}
// 编译实例 RVAs: 0xa398b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9829853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e823026aff","size":5,"soft_fail":false,"target_va":5369600800,"va":5379430648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4350130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e84e9fbdff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379431629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2655803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e8c579d7ff","size":5,"soft_fail":false,"target_va":5376776208,"va":5379432006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087bb00
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1827610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e8e61ce4ff","size":5,"soft_fail":false,"target_va":5377604352,"va":5379431957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b724f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1280615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e8678a1300","size":5,"soft_fail":false,"target_va":5380711664,"va":5379431044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b761c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1296292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e8a4c71300","size":5,"soft_fail":false,"target_va":5380727232,"va":5379430935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1790775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e837531b00","size":5,"soft_fail":false,"target_va":5381222304,"va":5379431524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428c08b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32009719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e8f76de801","size":5,"soft_fail":false,"target_va":5411440816,"va":5379431092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32111128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e818fae901","size":5,"soft_fail":false,"target_va":5411542880,"va":5379431747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t707445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e875cb0a00","size":5,"soft_fail":false,"target_va":5380138832,"va":5379431382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32105338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379430576,"raw_hex":"e87ae3e901","size":5,"soft_fail":false,"target_va":5411537104,"va":5379431761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":544,"record_offset":74464,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":544,"record_offset":72524,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":544,"record_offset":71076,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":74796,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":544,"record_offset":74520,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":29952,"source_line":501,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":72884,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":24753,"source_line":246,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":71788,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":70592,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":4784,"source_line":144,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":73096,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":4801,"source_line":1545,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$25::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":73708,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":4820,"source_line":1604,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$3::clone_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":72980,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":25367,"source_line":1022,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":72100,"parent_record_offset":70416,"ancestor_record_offset":70416,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":74944,"parent_record_offset":74920,"ancestor_record_offset":70416,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":74920,"parent_record_offset":74892,"ancestor_record_offset":70416,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":71432,"parent_record_offset":71408,"ancestor_record_offset":70416,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":71408,"parent_record_offset":71164,"ancestor_record_offset":70416,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":544,"record_offset":74488,"parent_record_offset":74464,"ancestor_record_offset":70416,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":544,"record_offset":71140,"parent_record_offset":71100,"ancestor_record_offset":70416,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":544,"record_offset":71164,"parent_record_offset":71140,"ancestor_record_offset":70416,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":544,"record_offset":72552,"parent_record_offset":72524,"ancestor_record_offset":70416,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":544,"record_offset":71100,"parent_record_offset":71076,"ancestor_record_offset":70416,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":70872,"parent_record_offset":70848,"ancestor_record_offset":70416,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":70924,"parent_record_offset":70900,"ancestor_record_offset":70416,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":70900,"parent_record_offset":70848,"ancestor_record_offset":70416,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":544,"record_offset":73056,"parent_record_offset":73028,"ancestor_record_offset":70416,"inlinee":8396,"source_line":174,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":72040,"parent_record_offset":72016,"ancestor_record_offset":70416,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":73440,"parent_record_offset":73408,"ancestor_record_offset":70416,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":72252,"parent_record_offset":72220,"ancestor_record_offset":70416,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":544,"record_offset":72620,"parent_record_offset":72592,"ancestor_record_offset":70416,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":544,"record_offset":72592,"parent_record_offset":72552,"ancestor_record_offset":70416,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":544,"record_offset":72700,"parent_record_offset":72676,"ancestor_record_offset":70416,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":544,"record_offset":72336,"parent_record_offset":72312,"ancestor_record_offset":70416,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":544,"record_offset":72312,"parent_record_offset":72288,"ancestor_record_offset":70416,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":544,"record_offset":72288,"parent_record_offset":72156,"ancestor_record_offset":70416,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":544,"record_offset":70948,"parent_record_offset":70924,"ancestor_record_offset":70416,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":544,"record_offset":74688,"parent_record_offset":74664,"ancestor_record_offset":70416,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":73624,"parent_record_offset":73596,"ancestor_record_offset":70416,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":71360,"parent_record_offset":71328,"ancestor_record_offset":70416,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":71208,"parent_record_offset":71164,"ancestor_record_offset":70416,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":74028,"parent_record_offset":73996,"ancestor_record_offset":70416,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":73996,"parent_record_offset":73964,"ancestor_record_offset":70416,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":72644,"parent_record_offset":72620,"ancestor_record_offset":70416,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":72676,"parent_record_offset":72592,"ancestor_record_offset":70416,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":73476,"parent_record_offset":73316,"ancestor_record_offset":70416,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":70820,"parent_record_offset":70792,"ancestor_record_offset":70416,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":74820,"parent_record_offset":74796,"ancestor_record_offset":70416,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":544,"record_offset":71680,"parent_record_offset":71656,"ancestor_record_offset":70416,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":544,"record_offset":73656,"parent_record_offset":73552,"ancestor_record_offset":70416,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":544,"record_offset":72824,"parent_record_offset":72800,"ancestor_record_offset":70416,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":74412,"parent_record_offset":74352,"ancestor_record_offset":70416,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":70972,"parent_record_offset":70948,"ancestor_record_offset":70416,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":74720,"parent_record_offset":74592,"ancestor_record_offset":70416,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":74760,"parent_record_offset":74520,"ancestor_record_offset":70416,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":544,"record_offset":71264,"parent_record_offset":71164,"ancestor_record_offset":70416,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":544,"record_offset":74636,"parent_record_offset":74592,"ancestor_record_offset":70416,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":544,"record_offset":71632,"parent_record_offset":71164,"ancestor_record_offset":70416,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":544,"record_offset":71328,"parent_record_offset":71296,"ancestor_record_offset":70416,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":544,"record_offset":71296,"parent_record_offset":71264,"ancestor_record_offset":70416,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":544,"record_offset":71656,"parent_record_offset":71632,"ancestor_record_offset":70416,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":544,"record_offset":72800,"parent_record_offset":72776,"ancestor_record_offset":70416,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":544,"record_offset":72752,"parent_record_offset":72700,"ancestor_record_offset":70416,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":544,"record_offset":72776,"parent_record_offset":72752,"ancestor_record_offset":70416,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":544,"record_offset":74560,"parent_record_offset":74520,"ancestor_record_offset":70416,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":544,"record_offset":74592,"parent_record_offset":74560,"ancestor_record_offset":70416,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":73924,"parent_record_offset":73884,"ancestor_record_offset":70416,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":73964,"parent_record_offset":73924,"ancestor_record_offset":70416,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":74072,"parent_record_offset":73884,"ancestor_record_offset":70416,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":73884,"parent_record_offset":73836,"ancestor_record_offset":70416,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":72016,"parent_record_offset":71992,"ancestor_record_offset":70416,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":73408,"parent_record_offset":73348,"ancestor_record_offset":70416,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":74164,"parent_record_offset":73836,"ancestor_record_offset":70416,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":72940,"parent_record_offset":72912,"ancestor_record_offset":70416,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":71860,"parent_record_offset":71836,"ancestor_record_offset":70416,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":71836,"parent_record_offset":71812,"ancestor_record_offset":70416,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":72192,"parent_record_offset":72156,"ancestor_record_offset":70416,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":71032,"parent_record_offset":70676,"ancestor_record_offset":70416,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":73028,"parent_record_offset":73004,"ancestor_record_offset":70416,"inlinee":25308,"source_line":198,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":73176,"parent_record_offset":73136,"ancestor_record_offset":70416,"inlinee":4805,"source_line":3151,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":72432,"parent_record_offset":72408,"ancestor_record_offset":70416,"inlinee":18642,"source_line":3186,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":73756,"parent_record_offset":73732,"ancestor_record_offset":70416,"inlinee":4824,"source_line":3275,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":72408,"parent_record_offset":72384,"ancestor_record_offset":70416,"inlinee":25300,"source_line":3329,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":70676,"parent_record_offset":70648,"ancestor_record_offset":70416,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":72156,"parent_record_offset":72128,"ancestor_record_offset":70416,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":74352,"parent_record_offset":73804,"ancestor_record_offset":70416,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":74264,"parent_record_offset":73836,"ancestor_record_offset":70416,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":71992,"parent_record_offset":71908,"ancestor_record_offset":70416,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73836,"parent_record_offset":73804,"ancestor_record_offset":70416,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73552,"parent_record_offset":73516,"ancestor_record_offset":70416,"inlinee":4809,"source_line":1358,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73516,"parent_record_offset":73216,"ancestor_record_offset":70416,"inlinee":4808,"source_line":1344,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":71884,"parent_record_offset":71860,"ancestor_record_offset":70416,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":72220,"parent_record_offset":72192,"ancestor_record_offset":70416,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":71964,"parent_record_offset":71908,"ancestor_record_offset":70416,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":71936,"parent_record_offset":71908,"ancestor_record_offset":70416,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":71908,"parent_record_offset":71884,"ancestor_record_offset":70416,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73596,"parent_record_offset":73552,"ancestor_record_offset":70416,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73216,"parent_record_offset":73176,"ancestor_record_offset":70416,"inlinee":4807,"source_line":3343,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":72456,"parent_record_offset":72432,"ancestor_record_offset":70416,"inlinee":18112,"source_line":3685,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73804,"parent_record_offset":73780,"ancestor_record_offset":70416,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$39::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":74892,"parent_record_offset":74868,"ancestor_record_offset":70416,"inlinee":4904,"source_line":3846,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$41::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":73780,"parent_record_offset":73756,"ancestor_record_offset":70416,"inlinee":4826,"source_line":3887,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":72912,"parent_record_offset":72884,"ancestor_record_offset":70416,"inlinee":24754,"source_line":326,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":71812,"parent_record_offset":71788,"ancestor_record_offset":70416,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$0::clone_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":73004,"parent_record_offset":72980,"ancestor_record_offset":70416,"inlinee":25368,"source_line":125,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":72128,"parent_record_offset":72100,"ancestor_record_offset":70416,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":70648,"parent_record_offset":70620,"ancestor_record_offset":70416,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":73136,"parent_record_offset":73096,"ancestor_record_offset":70416,"inlinee":4803,"source_line":1774,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":72384,"parent_record_offset":72360,"ancestor_record_offset":70416,"inlinee":25299,"source_line":1809,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":544,"record_offset":73732,"parent_record_offset":73708,"ancestor_record_offset":70416,"inlinee":4822,"source_line":1843,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":544,"record_offset":74664,"parent_record_offset":74592,"ancestor_record_offset":70416,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":70620,"parent_record_offset":70592,"ancestor_record_offset":70416,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":544,"record_offset":72360,"parent_record_offset":72336,"ancestor_record_offset":70416,"inlinee":25298,"source_line":1574,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":70732,"parent_record_offset":70704,"ancestor_record_offset":70416,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":70848,"parent_record_offset":70792,"ancestor_record_offset":70416,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":70704,"parent_record_offset":70676,"ancestor_record_offset":70416,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":544,"record_offset":70792,"parent_record_offset":70760,"ancestor_record_offset":70416,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":544,"record_offset":70760,"parent_record_offset":70732,"ancestor_record_offset":70416,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00544.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_nani.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_nani.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_nani.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::create_world_subset
// 信息 ID: a25e1cad96fdd604b4e611db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=544, inline_record=68384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::reload_nani
// 信息 ID: 87c6e7bface2017f7d55f5ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4ea91b65aa9dc2033c00387a: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)> > @ 0x9c7e3c -> 0xa398b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)> > > @ 0x14f73ae -> 0xa398b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::runtime_check_limits
// 信息 ID: 0011027dceb0f3c1781d25da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=544, inline_record=67476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::startup_nani
// 信息 ID: 1744d75543bb3a0e6f708f5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2a3f8068f7c7fbe09edb845b: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)> > @ 0x9c7a5e -> 0xa38610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)> > > @ 0x14f6ce1 -> 0xa38610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::startup_nani::closure$0
// 信息 ID: e98c69bd04d47f955788ce84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_nani::startup_nani", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=544, inline_record=68324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
