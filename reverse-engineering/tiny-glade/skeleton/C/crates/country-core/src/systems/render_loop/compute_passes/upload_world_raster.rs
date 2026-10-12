// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9466530062372a7a81bbb814922a0af2b8e7a154f2dc7c04f3aad86eabeac77d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: upload_world_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 254, 526
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":51224,"rva":6947888,"va":5375657008}
// 编译实例 RVAs: 0x6a0430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-810409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e857a2f3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375658004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3192888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8c847cfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375658403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8c8c6ffff","size":5,"soft_fail":false,"target_va":5375643296,"va":5375657939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069eba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8bde2ffff","size":5,"soft_fail":false,"target_va":5375650720,"va":5375658206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e890120000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375657947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t171220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8d49c0200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375657831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4718798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8ce004800","size":5,"soft_fail":false,"target_va":5380376688,"va":5375657885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41398909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e87db27702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375658142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40963852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e80c0f7102","size":5,"soft_fail":false,"target_va":5416621600,"va":5375657743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40963280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8d00c7102","size":5,"soft_fail":false,"target_va":5416621632,"va":5375658347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3621420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e82c423700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375657855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40916637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e89d567002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375658302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8327133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8dd0f7f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375658222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2259660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e83485ddff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375658119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7878473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e849377800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375658194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7881079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e877417800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375658068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2769978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375657008,"raw_hex":"e8c6bbd5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375658037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":56960,"parent_record_offset":51224,"ancestor_record_offset":51224,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":57288,"parent_record_offset":51224,"ancestor_record_offset":51224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":57044,"parent_record_offset":51224,"ancestor_record_offset":51224,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":57016,"parent_record_offset":51224,"ancestor_record_offset":51224,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":51496,"parent_record_offset":51224,"ancestor_record_offset":51224,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":52840,"parent_record_offset":52816,"ancestor_record_offset":51224,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":52888,"parent_record_offset":52864,"ancestor_record_offset":51224,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":52864,"parent_record_offset":52840,"ancestor_record_offset":51224,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":51224,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":52792,"parent_record_offset":52752,"ancestor_record_offset":51224,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":52752,"parent_record_offset":52728,"ancestor_record_offset":51224,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":56984,"parent_record_offset":56960,"ancestor_record_offset":51224,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":51552,"parent_record_offset":51524,"ancestor_record_offset":51224,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":53112,"parent_record_offset":53088,"ancestor_record_offset":51224,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":53088,"parent_record_offset":53064,"ancestor_record_offset":51224,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":53036,"parent_record_offset":53004,"ancestor_record_offset":51224,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":52980,"parent_record_offset":52472,"ancestor_record_offset":51224,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":53004,"parent_record_offset":52980,"ancestor_record_offset":51224,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":52504,"parent_record_offset":52472,"ancestor_record_offset":51224,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":53064,"parent_record_offset":53004,"ancestor_record_offset":51224,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":52552,"parent_record_offset":52528,"ancestor_record_offset":51224,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":52528,"parent_record_offset":52504,"ancestor_record_offset":51224,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":52728,"parent_record_offset":52704,"ancestor_record_offset":51224,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":53180,"parent_record_offset":53156,"ancestor_record_offset":51224,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":53156,"parent_record_offset":52472,"ancestor_record_offset":51224,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":51524,"parent_record_offset":51496,"ancestor_record_offset":51224,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":52472,"parent_record_offset":51524,"ancestor_record_offset":51224,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":51576,"parent_record_offset":51552,"ancestor_record_offset":51224,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":52220,"parent_record_offset":52196,"ancestor_record_offset":51224,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":52248,"parent_record_offset":52220,"ancestor_record_offset":51224,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":51788,"parent_record_offset":51764,"ancestor_record_offset":51224,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":51224,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":52020,"parent_record_offset":51992,"ancestor_record_offset":51224,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":51992,"parent_record_offset":51964,"ancestor_record_offset":51224,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":52640,"parent_record_offset":52580,"ancestor_record_offset":51224,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":52664,"parent_record_offset":52640,"ancestor_record_offset":51224,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":52704,"parent_record_offset":52528,"ancestor_record_offset":51224,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":53276,"parent_record_offset":51496,"ancestor_record_offset":51224,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":51812,"parent_record_offset":51788,"ancestor_record_offset":51224,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":52136,"parent_record_offset":52112,"ancestor_record_offset":51224,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":57148,"parent_record_offset":57112,"ancestor_record_offset":51224,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":52580,"parent_record_offset":52552,"ancestor_record_offset":51224,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":52612,"parent_record_offset":52580,"ancestor_record_offset":51224,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":52932,"parent_record_offset":52752,"ancestor_record_offset":51224,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":53204,"parent_record_offset":53180,"ancestor_record_offset":51224,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":57468,"parent_record_offset":57344,"ancestor_record_offset":51224,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":57492,"parent_record_offset":57468,"ancestor_record_offset":51224,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":53228,"parent_record_offset":53204,"ancestor_record_offset":51224,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":57084,"parent_record_offset":57044,"ancestor_record_offset":51224,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":57256,"parent_record_offset":57044,"ancestor_record_offset":51224,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":57112,"parent_record_offset":57044,"ancestor_record_offset":51224,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":52196,"parent_record_offset":52172,"ancestor_record_offset":51224,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":51932,"parent_record_offset":51904,"ancestor_record_offset":51224,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":51964,"parent_record_offset":51932,"ancestor_record_offset":51224,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":51904,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":51764,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":52324,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":51848,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":51608,"parent_record_offset":51576,"ancestor_record_offset":51224,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":52172,"parent_record_offset":52064,"ancestor_record_offset":51224,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":52112,"parent_record_offset":52088,"ancestor_record_offset":51224,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":52416,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":52088,"parent_record_offset":52064,"ancestor_record_offset":51224,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":51660,"parent_record_offset":51636,"ancestor_record_offset":51224,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":51636,"parent_record_offset":51608,"ancestor_record_offset":51224,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":51684,"parent_record_offset":51660,"ancestor_record_offset":51224,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":51736,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":52064,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":52296,"parent_record_offset":51684,"ancestor_record_offset":51224,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":57316,"parent_record_offset":57288,"ancestor_record_offset":51224,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":57380,"parent_record_offset":57344,"ancestor_record_offset":51224,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":57344,"parent_record_offset":57316,"ancestor_record_offset":51224,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":57408,"parent_record_offset":57380,"ancestor_record_offset":51224,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":57184,"parent_record_offset":57112,"ancestor_record_offset":51224,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":57224,"parent_record_offset":57112,"ancestor_record_offset":51224,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":70172,"rva":6952160,"va":5375661280}
// 编译实例 RVAs: 0x6a14e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-814681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e8a791f3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375662276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3197160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e81837cfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375662675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069c7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e828afffff","size":5,"soft_fail":false,"target_va":5375641520,"va":5375662211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069e900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e86dcfffff","size":5,"soft_fail":false,"target_va":5375650048,"va":5375662478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e8e0010000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375662219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t166948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e8248c0200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375662103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4714526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e81ef04700","size":5,"soft_fail":false,"target_va":5380376688,"va":5375662157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41394637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e8cda17702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375662414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40959580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e85cfe7002","size":5,"soft_fail":false,"target_va":5416621600,"va":5375662015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40959008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e820fc7002","size":5,"soft_fail":false,"target_va":5416621632,"va":5375662619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3617148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e87c313700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375662127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40912365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e8ed457002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375662574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8322861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e82dff7e00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375662494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2263932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e88474ddff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375662391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7874201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e899267800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375662466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7876807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e8c7307800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375662340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2774250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375661280,"raw_hex":"e816abd5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375662309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":75908,"parent_record_offset":70172,"ancestor_record_offset":70172,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":76236,"parent_record_offset":70172,"ancestor_record_offset":70172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":75992,"parent_record_offset":70172,"ancestor_record_offset":70172,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":75964,"parent_record_offset":70172,"ancestor_record_offset":70172,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":70444,"parent_record_offset":70172,"ancestor_record_offset":70172,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":71788,"parent_record_offset":71764,"ancestor_record_offset":70172,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":71836,"parent_record_offset":71812,"ancestor_record_offset":70172,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":71812,"parent_record_offset":71788,"ancestor_record_offset":70172,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":71764,"parent_record_offset":71740,"ancestor_record_offset":70172,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":71740,"parent_record_offset":71700,"ancestor_record_offset":70172,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":71700,"parent_record_offset":71676,"ancestor_record_offset":70172,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":75932,"parent_record_offset":75908,"ancestor_record_offset":70172,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":70500,"parent_record_offset":70472,"ancestor_record_offset":70172,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":72060,"parent_record_offset":72036,"ancestor_record_offset":70172,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":72036,"parent_record_offset":72012,"ancestor_record_offset":70172,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":71984,"parent_record_offset":71952,"ancestor_record_offset":70172,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":71928,"parent_record_offset":71420,"ancestor_record_offset":70172,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":71952,"parent_record_offset":71928,"ancestor_record_offset":70172,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":71452,"parent_record_offset":71420,"ancestor_record_offset":70172,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":72012,"parent_record_offset":71952,"ancestor_record_offset":70172,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":71500,"parent_record_offset":71476,"ancestor_record_offset":70172,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":71476,"parent_record_offset":71452,"ancestor_record_offset":70172,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":71676,"parent_record_offset":71652,"ancestor_record_offset":70172,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":72128,"parent_record_offset":72104,"ancestor_record_offset":70172,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":72104,"parent_record_offset":71420,"ancestor_record_offset":70172,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":70472,"parent_record_offset":70444,"ancestor_record_offset":70172,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":71420,"parent_record_offset":70472,"ancestor_record_offset":70172,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":70524,"parent_record_offset":70500,"ancestor_record_offset":70172,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":71168,"parent_record_offset":71144,"ancestor_record_offset":70172,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":71196,"parent_record_offset":71168,"ancestor_record_offset":70172,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":70736,"parent_record_offset":70712,"ancestor_record_offset":70172,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":70820,"parent_record_offset":70796,"ancestor_record_offset":70172,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":70968,"parent_record_offset":70940,"ancestor_record_offset":70172,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":70940,"parent_record_offset":70912,"ancestor_record_offset":70172,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":71588,"parent_record_offset":71528,"ancestor_record_offset":70172,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":71612,"parent_record_offset":71588,"ancestor_record_offset":70172,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":71652,"parent_record_offset":71476,"ancestor_record_offset":70172,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":72224,"parent_record_offset":70444,"ancestor_record_offset":70172,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":70760,"parent_record_offset":70736,"ancestor_record_offset":70172,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":71084,"parent_record_offset":71060,"ancestor_record_offset":70172,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":76096,"parent_record_offset":76060,"ancestor_record_offset":70172,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":71528,"parent_record_offset":71500,"ancestor_record_offset":70172,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":71560,"parent_record_offset":71528,"ancestor_record_offset":70172,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":71880,"parent_record_offset":71700,"ancestor_record_offset":70172,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":72152,"parent_record_offset":72128,"ancestor_record_offset":70172,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":76416,"parent_record_offset":76292,"ancestor_record_offset":70172,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":76440,"parent_record_offset":76416,"ancestor_record_offset":70172,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":72176,"parent_record_offset":72152,"ancestor_record_offset":70172,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":76032,"parent_record_offset":75992,"ancestor_record_offset":70172,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":76204,"parent_record_offset":75992,"ancestor_record_offset":70172,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":76060,"parent_record_offset":75992,"ancestor_record_offset":70172,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":71144,"parent_record_offset":71120,"ancestor_record_offset":70172,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":70880,"parent_record_offset":70852,"ancestor_record_offset":70172,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":70912,"parent_record_offset":70880,"ancestor_record_offset":70172,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":70852,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":70712,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":71272,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":70796,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":70556,"parent_record_offset":70524,"ancestor_record_offset":70172,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":71120,"parent_record_offset":71012,"ancestor_record_offset":70172,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":71060,"parent_record_offset":71036,"ancestor_record_offset":70172,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":71364,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":71036,"parent_record_offset":71012,"ancestor_record_offset":70172,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":70608,"parent_record_offset":70584,"ancestor_record_offset":70172,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":70584,"parent_record_offset":70556,"ancestor_record_offset":70172,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":70632,"parent_record_offset":70608,"ancestor_record_offset":70172,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":70684,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":71012,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":71244,"parent_record_offset":70632,"ancestor_record_offset":70172,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":76264,"parent_record_offset":76236,"ancestor_record_offset":70172,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":76328,"parent_record_offset":76292,"ancestor_record_offset":70172,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":76292,"parent_record_offset":76264,"ancestor_record_offset":70172,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":76356,"parent_record_offset":76328,"ancestor_record_offset":70172,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":76132,"parent_record_offset":76060,"ancestor_record_offset":70172,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":76172,"parent_record_offset":76060,"ancestor_record_offset":70172,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":57540,"rva":6949312,"va":5375658432}
// 编译实例 RVAs: 0x6a09c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-811833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e8c79cf3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375659428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3194312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e83842cfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375659827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069ca00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e898bcffff","size":5,"soft_fail":false,"target_va":5375642112,"va":5375659363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069ecf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e87ddeffff","size":5,"soft_fail":false,"target_va":5375651056,"va":5375659630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e8000d0000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375659371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t169796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e844970200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375659255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4717374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e83efb4700","size":5,"soft_fail":false,"target_va":5380376688,"va":5375659309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41397485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e8edac7702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375659566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40962428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e87c097102","size":5,"soft_fail":false,"target_va":5416621600,"va":5375659167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40961856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e840077102","size":5,"soft_fail":false,"target_va":5416621632,"va":5375659771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3619996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e89c3c3700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375659279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40915213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e80d517002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375659726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8325709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e84d0a7f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375659646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2261084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e8a47fddff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375659543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7877049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e8b9317800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375659618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7879655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e8e73b7800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375659492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2771402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375658432,"raw_hex":"e836b6d5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375659461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":63276,"parent_record_offset":57540,"ancestor_record_offset":57540,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":63604,"parent_record_offset":57540,"ancestor_record_offset":57540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":63360,"parent_record_offset":57540,"ancestor_record_offset":57540,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":63332,"parent_record_offset":57540,"ancestor_record_offset":57540,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":57812,"parent_record_offset":57540,"ancestor_record_offset":57540,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":59156,"parent_record_offset":59132,"ancestor_record_offset":57540,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":59204,"parent_record_offset":59180,"ancestor_record_offset":57540,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":59180,"parent_record_offset":59156,"ancestor_record_offset":57540,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":59132,"parent_record_offset":59108,"ancestor_record_offset":57540,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":59108,"parent_record_offset":59068,"ancestor_record_offset":57540,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":59068,"parent_record_offset":59044,"ancestor_record_offset":57540,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":63300,"parent_record_offset":63276,"ancestor_record_offset":57540,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":57868,"parent_record_offset":57840,"ancestor_record_offset":57540,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":59428,"parent_record_offset":59404,"ancestor_record_offset":57540,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":59404,"parent_record_offset":59380,"ancestor_record_offset":57540,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":59352,"parent_record_offset":59320,"ancestor_record_offset":57540,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":59296,"parent_record_offset":58788,"ancestor_record_offset":57540,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":59320,"parent_record_offset":59296,"ancestor_record_offset":57540,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":58820,"parent_record_offset":58788,"ancestor_record_offset":57540,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":59380,"parent_record_offset":59320,"ancestor_record_offset":57540,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":58868,"parent_record_offset":58844,"ancestor_record_offset":57540,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":58844,"parent_record_offset":58820,"ancestor_record_offset":57540,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":59044,"parent_record_offset":59020,"ancestor_record_offset":57540,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":59496,"parent_record_offset":59472,"ancestor_record_offset":57540,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":59472,"parent_record_offset":58788,"ancestor_record_offset":57540,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":57840,"parent_record_offset":57812,"ancestor_record_offset":57540,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":58788,"parent_record_offset":57840,"ancestor_record_offset":57540,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":57892,"parent_record_offset":57868,"ancestor_record_offset":57540,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":58536,"parent_record_offset":58512,"ancestor_record_offset":57540,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":58564,"parent_record_offset":58536,"ancestor_record_offset":57540,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":58104,"parent_record_offset":58080,"ancestor_record_offset":57540,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":58188,"parent_record_offset":58164,"ancestor_record_offset":57540,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":58336,"parent_record_offset":58308,"ancestor_record_offset":57540,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":58308,"parent_record_offset":58280,"ancestor_record_offset":57540,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":58956,"parent_record_offset":58896,"ancestor_record_offset":57540,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":58980,"parent_record_offset":58956,"ancestor_record_offset":57540,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":59020,"parent_record_offset":58844,"ancestor_record_offset":57540,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":59592,"parent_record_offset":57812,"ancestor_record_offset":57540,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":58128,"parent_record_offset":58104,"ancestor_record_offset":57540,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":58452,"parent_record_offset":58428,"ancestor_record_offset":57540,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":63464,"parent_record_offset":63428,"ancestor_record_offset":57540,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":58896,"parent_record_offset":58868,"ancestor_record_offset":57540,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":58928,"parent_record_offset":58896,"ancestor_record_offset":57540,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":59248,"parent_record_offset":59068,"ancestor_record_offset":57540,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":59520,"parent_record_offset":59496,"ancestor_record_offset":57540,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":63784,"parent_record_offset":63660,"ancestor_record_offset":57540,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":63808,"parent_record_offset":63784,"ancestor_record_offset":57540,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":59544,"parent_record_offset":59520,"ancestor_record_offset":57540,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":63400,"parent_record_offset":63360,"ancestor_record_offset":57540,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":63572,"parent_record_offset":63360,"ancestor_record_offset":57540,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":63428,"parent_record_offset":63360,"ancestor_record_offset":57540,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":58512,"parent_record_offset":58488,"ancestor_record_offset":57540,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":58248,"parent_record_offset":58220,"ancestor_record_offset":57540,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":58280,"parent_record_offset":58248,"ancestor_record_offset":57540,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":58220,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":58080,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":58640,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":58164,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":57924,"parent_record_offset":57892,"ancestor_record_offset":57540,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":58488,"parent_record_offset":58380,"ancestor_record_offset":57540,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":58428,"parent_record_offset":58404,"ancestor_record_offset":57540,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":58732,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":58404,"parent_record_offset":58380,"ancestor_record_offset":57540,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":57976,"parent_record_offset":57952,"ancestor_record_offset":57540,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":57952,"parent_record_offset":57924,"ancestor_record_offset":57540,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":58000,"parent_record_offset":57976,"ancestor_record_offset":57540,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":58052,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":58380,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":58612,"parent_record_offset":58000,"ancestor_record_offset":57540,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":63632,"parent_record_offset":63604,"ancestor_record_offset":57540,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":63696,"parent_record_offset":63660,"ancestor_record_offset":57540,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":63660,"parent_record_offset":63632,"ancestor_record_offset":57540,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":63724,"parent_record_offset":63696,"ancestor_record_offset":57540,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":63500,"parent_record_offset":63428,"ancestor_record_offset":57540,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":63540,"parent_record_offset":63428,"ancestor_record_offset":57540,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":32300,"rva":6943616,"va":5375652736}
// 编译实例 RVAs: 0x69f380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-806137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e807b3f3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375653732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3188616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e87858cfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375654131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069c310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8e8cbffff","size":5,"soft_fail":false,"target_va":5375640336,"va":5375653667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069f0e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8adf8ffff","size":5,"soft_fail":false,"target_va":5375652064,"va":5375653934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e840230000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375653675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t175492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e884ad0200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375653559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4723070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e87e114800","size":5,"soft_fail":false,"target_va":5380376688,"va":5375653613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41403181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e82dc37702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375653870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40968124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8bc1f7102","size":5,"soft_fail":false,"target_va":5416621600,"va":5375653471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40967552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8801d7102","size":5,"soft_fail":false,"target_va":5416621632,"va":5375654075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3625692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8dc523700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375653583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40920909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e84d677002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375654030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8331405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e88d207f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375653950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2255388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8e495ddff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375653847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7882745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e8f9477800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375653922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7885351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e827527800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375653796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2765706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375652736,"raw_hex":"e876ccd5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375653765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":38028,"parent_record_offset":32300,"ancestor_record_offset":32300,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":38356,"parent_record_offset":32300,"ancestor_record_offset":32300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":38112,"parent_record_offset":32300,"ancestor_record_offset":32300,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":38084,"parent_record_offset":32300,"ancestor_record_offset":32300,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":32564,"parent_record_offset":32300,"ancestor_record_offset":32300,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":33908,"parent_record_offset":33884,"ancestor_record_offset":32300,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":33956,"parent_record_offset":33932,"ancestor_record_offset":32300,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":32300,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":33884,"parent_record_offset":33860,"ancestor_record_offset":32300,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":33860,"parent_record_offset":33820,"ancestor_record_offset":32300,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":33820,"parent_record_offset":33796,"ancestor_record_offset":32300,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":38052,"parent_record_offset":38028,"ancestor_record_offset":32300,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":32620,"parent_record_offset":32592,"ancestor_record_offset":32300,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":34180,"parent_record_offset":34156,"ancestor_record_offset":32300,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":34156,"parent_record_offset":34132,"ancestor_record_offset":32300,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":34104,"parent_record_offset":34072,"ancestor_record_offset":32300,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":34048,"parent_record_offset":33540,"ancestor_record_offset":32300,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":34072,"parent_record_offset":34048,"ancestor_record_offset":32300,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":33572,"parent_record_offset":33540,"ancestor_record_offset":32300,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":34132,"parent_record_offset":34072,"ancestor_record_offset":32300,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":33620,"parent_record_offset":33596,"ancestor_record_offset":32300,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":33596,"parent_record_offset":33572,"ancestor_record_offset":32300,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":33796,"parent_record_offset":33772,"ancestor_record_offset":32300,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":34248,"parent_record_offset":34224,"ancestor_record_offset":32300,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":34224,"parent_record_offset":33540,"ancestor_record_offset":32300,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":32592,"parent_record_offset":32564,"ancestor_record_offset":32300,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":33540,"parent_record_offset":32592,"ancestor_record_offset":32300,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":32644,"parent_record_offset":32620,"ancestor_record_offset":32300,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":33288,"parent_record_offset":33264,"ancestor_record_offset":32300,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":33316,"parent_record_offset":33288,"ancestor_record_offset":32300,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":32856,"parent_record_offset":32832,"ancestor_record_offset":32300,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":32940,"parent_record_offset":32916,"ancestor_record_offset":32300,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":33088,"parent_record_offset":33060,"ancestor_record_offset":32300,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":33060,"parent_record_offset":33032,"ancestor_record_offset":32300,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":33708,"parent_record_offset":33648,"ancestor_record_offset":32300,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":33732,"parent_record_offset":33708,"ancestor_record_offset":32300,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":33772,"parent_record_offset":33596,"ancestor_record_offset":32300,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":34344,"parent_record_offset":32564,"ancestor_record_offset":32300,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":32880,"parent_record_offset":32856,"ancestor_record_offset":32300,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":33204,"parent_record_offset":33180,"ancestor_record_offset":32300,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":38216,"parent_record_offset":38180,"ancestor_record_offset":32300,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":33648,"parent_record_offset":33620,"ancestor_record_offset":32300,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":33680,"parent_record_offset":33648,"ancestor_record_offset":32300,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":34000,"parent_record_offset":33820,"ancestor_record_offset":32300,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":34272,"parent_record_offset":34248,"ancestor_record_offset":32300,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":38536,"parent_record_offset":38412,"ancestor_record_offset":32300,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":38560,"parent_record_offset":38536,"ancestor_record_offset":32300,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":34296,"parent_record_offset":34272,"ancestor_record_offset":32300,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":38152,"parent_record_offset":38112,"ancestor_record_offset":32300,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":38324,"parent_record_offset":38112,"ancestor_record_offset":32300,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":38180,"parent_record_offset":38112,"ancestor_record_offset":32300,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":33264,"parent_record_offset":33240,"ancestor_record_offset":32300,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":33000,"parent_record_offset":32972,"ancestor_record_offset":32300,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":33032,"parent_record_offset":33000,"ancestor_record_offset":32300,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":32972,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":32832,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":33392,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":32916,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":32676,"parent_record_offset":32644,"ancestor_record_offset":32300,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":33240,"parent_record_offset":33132,"ancestor_record_offset":32300,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":33180,"parent_record_offset":33156,"ancestor_record_offset":32300,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":33484,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":33156,"parent_record_offset":33132,"ancestor_record_offset":32300,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":32728,"parent_record_offset":32704,"ancestor_record_offset":32300,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":32704,"parent_record_offset":32676,"ancestor_record_offset":32300,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":32752,"parent_record_offset":32728,"ancestor_record_offset":32300,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":32804,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":33132,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":33364,"parent_record_offset":32752,"ancestor_record_offset":32300,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":38384,"parent_record_offset":38356,"ancestor_record_offset":32300,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":38448,"parent_record_offset":38412,"ancestor_record_offset":32300,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":38412,"parent_record_offset":38384,"ancestor_record_offset":32300,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":38476,"parent_record_offset":38448,"ancestor_record_offset":32300,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":38252,"parent_record_offset":38180,"ancestor_record_offset":32300,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":38292,"parent_record_offset":38180,"ancestor_record_offset":32300,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":44916,"rva":6946464,"va":5375655584}
// 编译实例 RVAs: 0x69fea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-808985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e8e7a7f3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375656580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3191464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e8584dcfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375656979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069c0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e878beffff","size":5,"soft_fail":false,"target_va":5375639744,"va":5375656515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069ea50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e8fde6ffff","size":5,"soft_fail":false,"target_va":5375650384,"va":5375656782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e820180000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375656523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t172644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e864a20200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375656407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4720222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e85e064800","size":5,"soft_fail":false,"target_va":5380376688,"va":5375656461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41400333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e80db87702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375656718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40965276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e89c147102","size":5,"soft_fail":false,"target_va":5416621600,"va":5375656319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40964704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e860127102","size":5,"soft_fail":false,"target_va":5416621632,"va":5375656923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3622844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e8bc473700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375656431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40918061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e82d5c7002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375656878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8328557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e86d157f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375656798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2258236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e8c48addff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375656695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7879897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e8d93c7800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375656770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7882503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e807477800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375656644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2768554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375655584,"raw_hex":"e856c1d5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375656613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":50644,"parent_record_offset":44916,"ancestor_record_offset":44916,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":50972,"parent_record_offset":44916,"ancestor_record_offset":44916,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":50728,"parent_record_offset":44916,"ancestor_record_offset":44916,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":50700,"parent_record_offset":44916,"ancestor_record_offset":44916,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":45180,"parent_record_offset":44916,"ancestor_record_offset":44916,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":46524,"parent_record_offset":46500,"ancestor_record_offset":44916,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":46572,"parent_record_offset":46548,"ancestor_record_offset":44916,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":46548,"parent_record_offset":46524,"ancestor_record_offset":44916,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":46500,"parent_record_offset":46476,"ancestor_record_offset":44916,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":46476,"parent_record_offset":46436,"ancestor_record_offset":44916,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":46436,"parent_record_offset":46412,"ancestor_record_offset":44916,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":50668,"parent_record_offset":50644,"ancestor_record_offset":44916,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":45236,"parent_record_offset":45208,"ancestor_record_offset":44916,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":46796,"parent_record_offset":46772,"ancestor_record_offset":44916,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":46772,"parent_record_offset":46748,"ancestor_record_offset":44916,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":46720,"parent_record_offset":46688,"ancestor_record_offset":44916,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":46664,"parent_record_offset":46156,"ancestor_record_offset":44916,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":46688,"parent_record_offset":46664,"ancestor_record_offset":44916,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":46188,"parent_record_offset":46156,"ancestor_record_offset":44916,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":46748,"parent_record_offset":46688,"ancestor_record_offset":44916,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":46236,"parent_record_offset":46212,"ancestor_record_offset":44916,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":46212,"parent_record_offset":46188,"ancestor_record_offset":44916,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":46412,"parent_record_offset":46388,"ancestor_record_offset":44916,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":46864,"parent_record_offset":46840,"ancestor_record_offset":44916,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":46840,"parent_record_offset":46156,"ancestor_record_offset":44916,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":45208,"parent_record_offset":45180,"ancestor_record_offset":44916,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":46156,"parent_record_offset":45208,"ancestor_record_offset":44916,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":45260,"parent_record_offset":45236,"ancestor_record_offset":44916,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":45904,"parent_record_offset":45880,"ancestor_record_offset":44916,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":45932,"parent_record_offset":45904,"ancestor_record_offset":44916,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":45472,"parent_record_offset":45448,"ancestor_record_offset":44916,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":45556,"parent_record_offset":45532,"ancestor_record_offset":44916,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":45704,"parent_record_offset":45676,"ancestor_record_offset":44916,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":45676,"parent_record_offset":45648,"ancestor_record_offset":44916,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":46324,"parent_record_offset":46264,"ancestor_record_offset":44916,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":46348,"parent_record_offset":46324,"ancestor_record_offset":44916,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":46388,"parent_record_offset":46212,"ancestor_record_offset":44916,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":46960,"parent_record_offset":45180,"ancestor_record_offset":44916,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":45496,"parent_record_offset":45472,"ancestor_record_offset":44916,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":45820,"parent_record_offset":45796,"ancestor_record_offset":44916,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":50832,"parent_record_offset":50796,"ancestor_record_offset":44916,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":46264,"parent_record_offset":46236,"ancestor_record_offset":44916,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":46296,"parent_record_offset":46264,"ancestor_record_offset":44916,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":46616,"parent_record_offset":46436,"ancestor_record_offset":44916,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":46888,"parent_record_offset":46864,"ancestor_record_offset":44916,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":51152,"parent_record_offset":51028,"ancestor_record_offset":44916,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":51176,"parent_record_offset":51152,"ancestor_record_offset":44916,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":46912,"parent_record_offset":46888,"ancestor_record_offset":44916,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":50768,"parent_record_offset":50728,"ancestor_record_offset":44916,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":50940,"parent_record_offset":50728,"ancestor_record_offset":44916,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":50796,"parent_record_offset":50728,"ancestor_record_offset":44916,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":45880,"parent_record_offset":45856,"ancestor_record_offset":44916,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":45616,"parent_record_offset":45588,"ancestor_record_offset":44916,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":45648,"parent_record_offset":45616,"ancestor_record_offset":44916,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":45588,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":45448,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":46008,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":45532,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":45292,"parent_record_offset":45260,"ancestor_record_offset":44916,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":45856,"parent_record_offset":45748,"ancestor_record_offset":44916,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45796,"parent_record_offset":45772,"ancestor_record_offset":44916,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":46100,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45772,"parent_record_offset":45748,"ancestor_record_offset":44916,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45344,"parent_record_offset":45320,"ancestor_record_offset":44916,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45320,"parent_record_offset":45292,"ancestor_record_offset":44916,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45368,"parent_record_offset":45344,"ancestor_record_offset":44916,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45420,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":45748,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":45980,"parent_record_offset":45368,"ancestor_record_offset":44916,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":51000,"parent_record_offset":50972,"ancestor_record_offset":44916,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":51064,"parent_record_offset":51028,"ancestor_record_offset":44916,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":51028,"parent_record_offset":51000,"ancestor_record_offset":44916,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":51092,"parent_record_offset":51064,"ancestor_record_offset":44916,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":50868,"parent_record_offset":50796,"ancestor_record_offset":44916,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":50908,"parent_record_offset":50796,"ancestor_record_offset":44916,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":63856,"rva":6950736,"va":5375659856}
// 编译实例 RVAs: 0x6a0f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-813257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e83797f3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375660852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3195736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8a83ccfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375661251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069c560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e868b2ffff","size":5,"soft_fail":false,"target_va":5375640928,"va":5375660787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069ef90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e88ddbffff","size":5,"soft_fail":false,"target_va":5375651728,"va":5375661054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e870070000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375660795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t168372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8b4910200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375660679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4715950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8aef54700","size":5,"soft_fail":false,"target_va":5380376688,"va":5375660733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41396061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e85da77702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375660990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40961004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8ec037102","size":5,"soft_fail":false,"target_va":5416621600,"va":5375660591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40960432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8b0017102","size":5,"soft_fail":false,"target_va":5416621632,"va":5375661195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3618572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e80c373700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375660703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40913789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e87d4b7002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375661150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8324285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8bd047f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375661070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2262508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8147addff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375660967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7875625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8292c7800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375661042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7878231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e857367800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375660916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2772826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375659856,"raw_hex":"e8a6b0d5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375660885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":69592,"parent_record_offset":63856,"ancestor_record_offset":63856,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":69920,"parent_record_offset":63856,"ancestor_record_offset":63856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":69676,"parent_record_offset":63856,"ancestor_record_offset":63856,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":69648,"parent_record_offset":63856,"ancestor_record_offset":63856,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":64128,"parent_record_offset":63856,"ancestor_record_offset":63856,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":65472,"parent_record_offset":65448,"ancestor_record_offset":63856,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":65520,"parent_record_offset":65496,"ancestor_record_offset":63856,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":65496,"parent_record_offset":65472,"ancestor_record_offset":63856,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":65448,"parent_record_offset":65424,"ancestor_record_offset":63856,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":65424,"parent_record_offset":65384,"ancestor_record_offset":63856,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":65384,"parent_record_offset":65360,"ancestor_record_offset":63856,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":69616,"parent_record_offset":69592,"ancestor_record_offset":63856,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":64184,"parent_record_offset":64156,"ancestor_record_offset":63856,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":65744,"parent_record_offset":65720,"ancestor_record_offset":63856,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":65720,"parent_record_offset":65696,"ancestor_record_offset":63856,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":65668,"parent_record_offset":65636,"ancestor_record_offset":63856,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":65612,"parent_record_offset":65104,"ancestor_record_offset":63856,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":65636,"parent_record_offset":65612,"ancestor_record_offset":63856,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":65136,"parent_record_offset":65104,"ancestor_record_offset":63856,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":65696,"parent_record_offset":65636,"ancestor_record_offset":63856,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":65184,"parent_record_offset":65160,"ancestor_record_offset":63856,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":65160,"parent_record_offset":65136,"ancestor_record_offset":63856,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":65360,"parent_record_offset":65336,"ancestor_record_offset":63856,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":65812,"parent_record_offset":65788,"ancestor_record_offset":63856,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":65788,"parent_record_offset":65104,"ancestor_record_offset":63856,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":64156,"parent_record_offset":64128,"ancestor_record_offset":63856,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":65104,"parent_record_offset":64156,"ancestor_record_offset":63856,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":64208,"parent_record_offset":64184,"ancestor_record_offset":63856,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":64852,"parent_record_offset":64828,"ancestor_record_offset":63856,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":64880,"parent_record_offset":64852,"ancestor_record_offset":63856,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":64420,"parent_record_offset":64396,"ancestor_record_offset":63856,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":64504,"parent_record_offset":64480,"ancestor_record_offset":63856,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":64652,"parent_record_offset":64624,"ancestor_record_offset":63856,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":64624,"parent_record_offset":64596,"ancestor_record_offset":63856,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":65272,"parent_record_offset":65212,"ancestor_record_offset":63856,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":65296,"parent_record_offset":65272,"ancestor_record_offset":63856,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":65336,"parent_record_offset":65160,"ancestor_record_offset":63856,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":65908,"parent_record_offset":64128,"ancestor_record_offset":63856,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":64444,"parent_record_offset":64420,"ancestor_record_offset":63856,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":64768,"parent_record_offset":64744,"ancestor_record_offset":63856,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":69780,"parent_record_offset":69744,"ancestor_record_offset":63856,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":65212,"parent_record_offset":65184,"ancestor_record_offset":63856,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":65244,"parent_record_offset":65212,"ancestor_record_offset":63856,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":65564,"parent_record_offset":65384,"ancestor_record_offset":63856,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":65836,"parent_record_offset":65812,"ancestor_record_offset":63856,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":70100,"parent_record_offset":69976,"ancestor_record_offset":63856,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":70124,"parent_record_offset":70100,"ancestor_record_offset":63856,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":65860,"parent_record_offset":65836,"ancestor_record_offset":63856,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":69716,"parent_record_offset":69676,"ancestor_record_offset":63856,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":69888,"parent_record_offset":69676,"ancestor_record_offset":63856,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":69744,"parent_record_offset":69676,"ancestor_record_offset":63856,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":64828,"parent_record_offset":64804,"ancestor_record_offset":63856,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":64564,"parent_record_offset":64536,"ancestor_record_offset":63856,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":64596,"parent_record_offset":64564,"ancestor_record_offset":63856,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":64536,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":64396,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":64956,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":64480,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":64240,"parent_record_offset":64208,"ancestor_record_offset":63856,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":64804,"parent_record_offset":64696,"ancestor_record_offset":63856,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64744,"parent_record_offset":64720,"ancestor_record_offset":63856,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":65048,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64720,"parent_record_offset":64696,"ancestor_record_offset":63856,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64292,"parent_record_offset":64268,"ancestor_record_offset":63856,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64268,"parent_record_offset":64240,"ancestor_record_offset":63856,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64316,"parent_record_offset":64292,"ancestor_record_offset":63856,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64368,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":64696,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":64928,"parent_record_offset":64316,"ancestor_record_offset":63856,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":69948,"parent_record_offset":69920,"ancestor_record_offset":63856,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":70012,"parent_record_offset":69976,"ancestor_record_offset":63856,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":69976,"parent_record_offset":69948,"ancestor_record_offset":63856,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":70040,"parent_record_offset":70012,"ancestor_record_offset":63856,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":69816,"parent_record_offset":69744,"ancestor_record_offset":63856,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":69856,"parent_record_offset":69744,"ancestor_record_offset":63856,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":254,"record_offset":38608,"rva":6945040,"va":5375654160}
// 编译实例 RVAs: 0x69f910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$country_core..shader..upload_cpu_raster..main$LT$P$GT$$u20$as$u20$shaderface..permutation..ToShaderPermutationKey$GT$::to_shader_permutation_key
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-807561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e877adf3ff","size":5,"soft_fail":false,"target_va":5374847600,"va":5375655156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3190040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8e852cfff","size":5,"soft_fail":false,"target_va":5372465520,"va":5375655555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069be70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8b8c1ffff","size":5,"soft_fail":false,"target_va":5375639152,"va":5375655091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069f230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e86df4ffff","size":5,"soft_fail":false,"target_va":5375652400,"va":5375655358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8b01d0000","size":5,"soft_fail":false,"target_va":5375662704,"va":5375655099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t174068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8f4a70200","size":5,"soft_fail":false,"target_va":5375829056,"va":5375654983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b20870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4721646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8ee0b4800","size":5,"soft_fail":false,"target_va":5380376688,"va":5375655037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41401757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e89dbd7702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375655294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40966700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e82c1a7102","size":5,"soft_fail":false,"target_va":5416621600,"va":5375654895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40966128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8f0177102","size":5,"soft_fail":false,"target_va":5416621632,"va":5375655499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3624268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e84c4d3700","size":5,"soft_fail":false,"target_va":5379279280,"va":5375655007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40919485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8bd617002","size":5,"soft_fail":false,"target_va":5416574944,"va":5375655454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8329981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8fd1a7f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5375655374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2256812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e85490ddff","size":5,"soft_fail":false,"target_va":5373398464,"va":5375655271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7881321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e869427800","size":5,"soft_fail":false,"target_va":5383536672,"va":5375655346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7883927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8974c7800","size":5,"soft_fail":false,"target_va":5383539152,"va":5375655220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2767130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375654160,"raw_hex":"e8e6c6d5ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5375655189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":44336,"parent_record_offset":38608,"ancestor_record_offset":38608,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":44664,"parent_record_offset":38608,"ancestor_record_offset":38608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":44420,"parent_record_offset":38608,"ancestor_record_offset":38608,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":254,"record_offset":44392,"parent_record_offset":38608,"ancestor_record_offset":38608,"inlinee":8361,"source_line":752,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":254,"record_offset":38872,"parent_record_offset":38608,"ancestor_record_offset":38608,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":40216,"parent_record_offset":40192,"ancestor_record_offset":38608,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":40264,"parent_record_offset":40240,"ancestor_record_offset":38608,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":254,"record_offset":40240,"parent_record_offset":40216,"ancestor_record_offset":38608,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":40192,"parent_record_offset":40168,"ancestor_record_offset":38608,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":40168,"parent_record_offset":40128,"ancestor_record_offset":38608,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":254,"record_offset":40128,"parent_record_offset":40104,"ancestor_record_offset":38608,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":254,"record_offset":44360,"parent_record_offset":44336,"ancestor_record_offset":38608,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":254,"record_offset":38928,"parent_record_offset":38900,"ancestor_record_offset":38608,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":40488,"parent_record_offset":40464,"ancestor_record_offset":38608,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":40464,"parent_record_offset":40440,"ancestor_record_offset":38608,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":254,"record_offset":40412,"parent_record_offset":40380,"ancestor_record_offset":38608,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":40356,"parent_record_offset":39848,"ancestor_record_offset":38608,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":40380,"parent_record_offset":40356,"ancestor_record_offset":38608,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":39880,"parent_record_offset":39848,"ancestor_record_offset":38608,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":254,"record_offset":40440,"parent_record_offset":40380,"ancestor_record_offset":38608,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":39928,"parent_record_offset":39904,"ancestor_record_offset":38608,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":39904,"parent_record_offset":39880,"ancestor_record_offset":38608,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":254,"record_offset":40104,"parent_record_offset":40080,"ancestor_record_offset":38608,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":254,"record_offset":40556,"parent_record_offset":40532,"ancestor_record_offset":38608,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":40532,"parent_record_offset":39848,"ancestor_record_offset":38608,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":38900,"parent_record_offset":38872,"ancestor_record_offset":38608,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":254,"record_offset":39848,"parent_record_offset":38900,"ancestor_record_offset":38608,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":254,"record_offset":38952,"parent_record_offset":38928,"ancestor_record_offset":38608,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":254,"record_offset":39596,"parent_record_offset":39572,"ancestor_record_offset":38608,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":254,"record_offset":39624,"parent_record_offset":39596,"ancestor_record_offset":38608,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":39164,"parent_record_offset":39140,"ancestor_record_offset":38608,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":254,"record_offset":39248,"parent_record_offset":39224,"ancestor_record_offset":38608,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":39396,"parent_record_offset":39368,"ancestor_record_offset":38608,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":254,"record_offset":39368,"parent_record_offset":39340,"ancestor_record_offset":38608,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":254,"record_offset":40016,"parent_record_offset":39956,"ancestor_record_offset":38608,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":40040,"parent_record_offset":40016,"ancestor_record_offset":38608,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":40080,"parent_record_offset":39904,"ancestor_record_offset":38608,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":254,"record_offset":40652,"parent_record_offset":38872,"ancestor_record_offset":38608,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":254,"record_offset":39188,"parent_record_offset":39164,"ancestor_record_offset":38608,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":254,"record_offset":39512,"parent_record_offset":39488,"ancestor_record_offset":38608,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":44524,"parent_record_offset":44488,"ancestor_record_offset":38608,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":254,"record_offset":39956,"parent_record_offset":39928,"ancestor_record_offset":38608,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":39988,"parent_record_offset":39956,"ancestor_record_offset":38608,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":254,"record_offset":40308,"parent_record_offset":40128,"ancestor_record_offset":38608,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":40580,"parent_record_offset":40556,"ancestor_record_offset":38608,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":44844,"parent_record_offset":44720,"ancestor_record_offset":38608,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":44868,"parent_record_offset":44844,"ancestor_record_offset":38608,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":254,"record_offset":40604,"parent_record_offset":40580,"ancestor_record_offset":38608,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":44460,"parent_record_offset":44420,"ancestor_record_offset":38608,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":254,"record_offset":44632,"parent_record_offset":44420,"ancestor_record_offset":38608,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":254,"record_offset":44488,"parent_record_offset":44420,"ancestor_record_offset":38608,"inlinee":18078,"source_line":39,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":254,"record_offset":39572,"parent_record_offset":39548,"ancestor_record_offset":38608,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":39308,"parent_record_offset":39280,"ancestor_record_offset":38608,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":39340,"parent_record_offset":39308,"ancestor_record_offset":38608,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":254,"record_offset":39280,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":39140,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":39700,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":254,"record_offset":39224,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":38984,"parent_record_offset":38952,"ancestor_record_offset":38608,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":254,"record_offset":39548,"parent_record_offset":39440,"ancestor_record_offset":38608,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39488,"parent_record_offset":39464,"ancestor_record_offset":38608,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39792,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39464,"parent_record_offset":39440,"ancestor_record_offset":38608,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39036,"parent_record_offset":39012,"ancestor_record_offset":38608,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39012,"parent_record_offset":38984,"ancestor_record_offset":38608,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39060,"parent_record_offset":39036,"ancestor_record_offset":38608,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39112,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":254,"record_offset":39440,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":254,"record_offset":39672,"parent_record_offset":39060,"ancestor_record_offset":38608,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":254,"record_offset":44692,"parent_record_offset":44664,"ancestor_record_offset":38608,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":44756,"parent_record_offset":44720,"ancestor_record_offset":38608,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":254,"record_offset":44720,"parent_record_offset":44692,"ancestor_record_offset":38608,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":254,"record_offset":44784,"parent_record_offset":44756,"ancestor_record_offset":38608,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":44560,"parent_record_offset":44488,"ancestor_record_offset":38608,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":44600,"parent_record_offset":44488,"ancestor_record_offset":38608,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x469E
// 首条位置: {"module":254,"inlinee":18078}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":254,"record_offset":38216,"parent_record_offset":38180,"ancestor_record_offset":38180,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":38252,"parent_record_offset":38180,"ancestor_record_offset":38180,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":254,"record_offset":38292,"parent_record_offset":38180,"ancestor_record_offset":38180,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":56116,"rva":10438656,"va":5379147776}
// 编译实例 RVAs: 0x9f4800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140395120
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6684056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e868029aff","size":5,"soft_fail":false,"target_va":5372465440,"va":5379149491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6683912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8f8029aff","size":5,"soft_fail":false,"target_va":5372465520,"va":5379149427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141537b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11808748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8ec2fb400","size":5,"soft_fail":false,"target_va":5390957408,"va":5379148655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141546450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11868151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8f717b500","size":5,"soft_fail":false,"target_va":5391017040,"va":5379148884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176c700
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14121294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e84e79d700","size":5,"soft_fail":false,"target_va":5393270528,"va":5379149229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14140042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e88ac2d700","size":5,"soft_fail":false,"target_va":5393288768,"va":5379148721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417982c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14300862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8be36da00","size":5,"soft_fail":false,"target_va":5393449664,"va":5379148797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37908231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8076f4202","size":5,"soft_fail":false,"target_va":5417057056,"va":5379148820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37473327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e82fcc3b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379148268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t127103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e87ff00100","size":5,"soft_fail":false,"target_va":5379276288,"va":5379149180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t130663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e867fe0100","size":5,"soft_fail":false,"target_va":5379279280,"va":5379148612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37425596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8bc113b02","size":5,"soft_fail":false,"target_va":5416574944,"va":5379149343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37422808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8d8063b02","size":5,"soft_fail":false,"target_va":5416572144,"va":5379149331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4836455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e867cc4900","size":5,"soft_fail":false,"target_va":5383985360,"va":5379148900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4387795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8d3f34200","size":5,"soft_fail":false,"target_va":5383536672,"va":5379148872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4390395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147776,"raw_hex":"e8fbfd4200","size":5,"soft_fail":false,"target_va":5383539152,"va":5379148752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":61804,"parent_record_offset":56116,"ancestor_record_offset":56116,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":62108,"parent_record_offset":56116,"ancestor_record_offset":56116,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":61888,"parent_record_offset":56116,"ancestor_record_offset":56116,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::on_uploaded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":526,"record_offset":61860,"parent_record_offset":56116,"ancestor_record_offset":56116,"inlinee":31068,"source_line":151,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":526,"record_offset":62356,"parent_record_offset":56116,"ancestor_record_offset":56116,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":526,"record_offset":56324,"parent_record_offset":56116,"ancestor_record_offset":56116,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":57676,"parent_record_offset":57652,"ancestor_record_offset":56116,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":57724,"parent_record_offset":57700,"ancestor_record_offset":56116,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":57700,"parent_record_offset":57676,"ancestor_record_offset":56116,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":57652,"parent_record_offset":57628,"ancestor_record_offset":56116,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":57628,"parent_record_offset":57588,"ancestor_record_offset":56116,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":57588,"parent_record_offset":57564,"ancestor_record_offset":56116,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":526,"record_offset":61828,"parent_record_offset":61804,"ancestor_record_offset":56116,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":526,"record_offset":56380,"parent_record_offset":56352,"ancestor_record_offset":56116,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":57948,"parent_record_offset":57924,"ancestor_record_offset":56116,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":57924,"parent_record_offset":57900,"ancestor_record_offset":56116,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":57872,"parent_record_offset":57840,"ancestor_record_offset":56116,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":63884,"parent_record_offset":63376,"ancestor_record_offset":56116,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":57816,"parent_record_offset":57304,"ancestor_record_offset":56116,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":57840,"parent_record_offset":57816,"ancestor_record_offset":56116,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":57336,"parent_record_offset":57304,"ancestor_record_offset":56116,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":63968,"parent_record_offset":63908,"ancestor_record_offset":56116,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":57900,"parent_record_offset":57840,"ancestor_record_offset":56116,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":57384,"parent_record_offset":57360,"ancestor_record_offset":56116,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":57360,"parent_record_offset":57336,"ancestor_record_offset":56116,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":57564,"parent_record_offset":57540,"ancestor_record_offset":56116,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":526,"record_offset":58016,"parent_record_offset":57992,"ancestor_record_offset":56116,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":57992,"parent_record_offset":57304,"ancestor_record_offset":56116,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":62388,"parent_record_offset":62356,"ancestor_record_offset":56116,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":63376,"parent_record_offset":62388,"ancestor_record_offset":56116,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":56352,"parent_record_offset":56324,"ancestor_record_offset":56116,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":57304,"parent_record_offset":56352,"ancestor_record_offset":56116,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":526,"record_offset":56404,"parent_record_offset":56380,"ancestor_record_offset":56116,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":526,"record_offset":57052,"parent_record_offset":57028,"ancestor_record_offset":56116,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":57080,"parent_record_offset":57052,"ancestor_record_offset":56116,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":56620,"parent_record_offset":56596,"ancestor_record_offset":56116,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":56704,"parent_record_offset":56680,"ancestor_record_offset":56116,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":56852,"parent_record_offset":56824,"ancestor_record_offset":56116,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":56824,"parent_record_offset":56796,"ancestor_record_offset":56116,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":526,"record_offset":57476,"parent_record_offset":57412,"ancestor_record_offset":56116,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":57500,"parent_record_offset":57476,"ancestor_record_offset":56116,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":57540,"parent_record_offset":57360,"ancestor_record_offset":56116,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":58112,"parent_record_offset":56324,"ancestor_record_offset":56116,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":56644,"parent_record_offset":56620,"ancestor_record_offset":56116,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":56968,"parent_record_offset":56944,"ancestor_record_offset":56116,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":57412,"parent_record_offset":57384,"ancestor_record_offset":56116,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":57448,"parent_record_offset":57412,"ancestor_record_offset":56116,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":57768,"parent_record_offset":57588,"ancestor_record_offset":56116,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":58040,"parent_record_offset":58016,"ancestor_record_offset":56116,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":62288,"parent_record_offset":62164,"ancestor_record_offset":56116,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":64280,"parent_record_offset":64156,"ancestor_record_offset":56116,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":62312,"parent_record_offset":62288,"ancestor_record_offset":56116,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":58064,"parent_record_offset":58040,"ancestor_record_offset":56116,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":64304,"parent_record_offset":64280,"ancestor_record_offset":56116,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":61920,"parent_record_offset":61888,"ancestor_record_offset":56116,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":62076,"parent_record_offset":61888,"ancestor_record_offset":56116,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::upload_plant_sample::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":526,"record_offset":62012,"parent_record_offset":61976,"ancestor_record_offset":56116,"inlinee":30657,"source_line":38400,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
//     evidence={"module":526,"record_offset":61948,"parent_record_offset":61888,"ancestor_record_offset":56116,"inlinee":31070,"source_line":67,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":526,"record_offset":57028,"parent_record_offset":57004,"ancestor_record_offset":56116,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":56764,"parent_record_offset":56736,"ancestor_record_offset":56116,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":56796,"parent_record_offset":56764,"ancestor_record_offset":56116,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":56736,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":56596,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":57156,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":56680,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":56440,"parent_record_offset":56404,"ancestor_record_offset":56116,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":57004,"parent_record_offset":56896,"ancestor_record_offset":56116,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56944,"parent_record_offset":56920,"ancestor_record_offset":56116,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":57248,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56920,"parent_record_offset":56896,"ancestor_record_offset":56116,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56492,"parent_record_offset":56468,"ancestor_record_offset":56116,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56468,"parent_record_offset":56440,"ancestor_record_offset":56116,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56516,"parent_record_offset":56492,"ancestor_record_offset":56116,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56568,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":56896,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":526,"record_offset":57128,"parent_record_offset":56516,"ancestor_record_offset":56116,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":64128,"parent_record_offset":64100,"ancestor_record_offset":56116,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":526,"record_offset":62136,"parent_record_offset":62108,"ancestor_record_offset":56116,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":62200,"parent_record_offset":62164,"ancestor_record_offset":56116,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":62164,"parent_record_offset":62136,"ancestor_record_offset":56116,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":526,"record_offset":64156,"parent_record_offset":64128,"ancestor_record_offset":56116,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":62228,"parent_record_offset":62200,"ancestor_record_offset":56116,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":526,"record_offset":61976,"parent_record_offset":61948,"ancestor_record_offset":56116,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":526,"record_offset":62044,"parent_record_offset":61948,"ancestor_record_offset":56116,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x795E
// 首条位置: {"module":526,"inlinee":31070}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":526,"record_offset":61976,"parent_record_offset":61948,"ancestor_record_offset":61948,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":526,"record_offset":62044,"parent_record_offset":61948,"ancestor_record_offset":61948,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::upload_plant_sample::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":526,"record_offset":62012,"parent_record_offset":61976,"ancestor_record_offset":61948,"inlinee":30657,"source_line":38400,"archive":"_pdb/C/modules/00526.jsonl.gz"}
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
// 完整结构及原始类型记录: upload_world_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: upload_world_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: upload_world_raster.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture
// [PDB Shader 类型/入口] {"shader_family":"country_core/upload_plant_sample","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::upload_plant_sample::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":62012,"parent_record_offset":61976,"ancestor_record_offset":56116,"inlinee":30657,"source_line":38400,"archive":"_pdb/C/modules/00526.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/upload_plant_sample","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::upload_plant_sample::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":62012,"parent_record_offset":61976,"ancestor_record_offset":61948,"inlinee":30657,"source_line":38400,"archive":"_pdb/C/modules/00526.jsonl.gz"}}
// [Shader 映射] {"shader_id":"3519001e093c0891d56bb812","relative_path":"country-core/compiled/upload_plant_sample.precomp","sha256":"a49ee211d20a65f26d2419eeaf617fc2e1efb92f5ed8a49e6f96845f93f40a49","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture","hint":{"shader_family":"country_core/upload_plant_sample","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::upload_plant_sample::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":62012,"parent_record_offset":61976,"ancestor_record_offset":56116,"inlinee":30657,"source_line":38400,"archive":"_pdb/C/modules/00526.jsonl.gz"}},"shader_uri":"country_core/upload_plant_sample:main"}
// [Shader 映射] {"shader_id":"3519001e093c0891d56bb812","relative_path":"country-core/compiled/upload_plant_sample.precomp","sha256":"a49ee211d20a65f26d2419eeaf617fc2e1efb92f5ed8a49e6f96845f93f40a49","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture::closure$0","hint":{"shader_family":"country_core/upload_plant_sample","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::upload_plant_sample::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":62012,"parent_record_offset":61976,"ancestor_record_offset":61948,"inlinee":30657,"source_line":38400,"archive":"_pdb/C/modules/00526.jsonl.gz"}},"shader_uri":"country_core/upload_plant_sample:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/upload_plant_sample.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[19]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"globalParams","storage_class":"Uniform","type":{"id":7,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"output_tex_width","type_id":6,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"cpu_raster","storage_class":"Uniform","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,45]},"decorations":[],"pointee":{"id":45,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[46]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":46,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":76,"name":"output_tex","storage_class":"UniformConstant","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,73]},"decorations":[],"pointee":{"id":73,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[6,1,2,0,0,2,32]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"unknown","storage_class":"Input","type":{"id":18,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,16]},"decorations":[],"pointee":{"id":16,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679323,"position_among_mapped_render_targets":1,"mapped_render_target_count":74,"previous_callsite":null,"next_callsite":5679337,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679337,"position_among_mapped_render_targets":2,"mapped_render_target_count":74,"previous_callsite":5679323,"next_callsite":5679351,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679351,"position_among_mapped_render_targets":3,"mapped_render_target_count":74,"previous_callsite":5679337,"next_callsite":5679365,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679365,"position_among_mapped_render_targets":4,"mapped_render_target_count":74,"previous_callsite":5679351,"next_callsite":5679379,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679379,"position_among_mapped_render_targets":5,"mapped_render_target_count":74,"previous_callsite":5679365,"next_callsite":5679393,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679393,"position_among_mapped_render_targets":6,"mapped_render_target_count":74,"previous_callsite":5679379,"next_callsite":5679407,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679407,"position_among_mapped_render_targets":7,"mapped_render_target_count":74,"previous_callsite":5679393,"next_callsite":5679418,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679418,"position_among_mapped_render_targets":8,"mapped_render_target_count":74,"previous_callsite":5679407,"next_callsite":5679429,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: upload_world_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture
// 信息 ID: e53dab8823adf0be55cb3374
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a93a -> 0x9f4800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture::closure$0
// 信息 ID: d4f70aa1284d2c2f3472a1b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_plant_sample_to_texture; module=526, inline_record=61948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture::closure$0
// 信息 ID: f730b7c5e580140be8fd69dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>; module=254, inline_record=38180; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>; module=254, inline_record=44488; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>; module=254, inline_record=50796; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>; module=254, inline_record=57112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>; module=254, inline_record=63428; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster>; module=254, inline_record=69744; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>; module=254, inline_record=76060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>
// 信息 ID: bfd86667ed364ccd8eae3353
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::Cobblestone1Raster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a8e9 -> 0x6a0430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>
// 信息 ID: 8578b1f508b50b1751074f73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::Cobblestone2Raster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a8f7 -> 0x6a14e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>
// 信息 ID: 4163ecb9c3136d73e8603ffe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::Cobblestone3Raster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a905 -> 0x6a09c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// 信息 ID: 5cd7ac705a1d3b2963f79689
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::GardenRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a921 -> 0x69f380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// 信息 ID: 69c43691485f31d7b3a782de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::PathRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a8db -> 0x69fea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster>
// 信息 ID: cd2db364df99d5dceab2b409
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::PlantExemplarRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a92f -> 0x6a0f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// 信息 ID: 3841c5cd33912682ea0c88e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::WaterRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a913 -> 0x69f910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
