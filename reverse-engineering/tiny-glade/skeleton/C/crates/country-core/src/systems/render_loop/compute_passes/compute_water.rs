// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9fb386051f1ed17ceb2476f84f4eeaf97630f6cf83cd800ecd858f9023bbfc03
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compute_water.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 560
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
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
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
// 函数符号: country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":560,"record_offset":4920,"rva":11575840,"va":5380284960}
// 编译实例 RVAs: 0xb0a220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3186649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8d99f3000","size":5,"soft_fail":false,"target_va":5383472768,"va":5380286114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395120
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7821880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8c8a588ff","size":5,"soft_fail":false,"target_va":5372465440,"va":5380287315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7821736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e858a688ff","size":5,"soft_fail":false,"target_va":5372465520,"va":5380287251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141536440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10665659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8bbbea200","size":5,"soft_fail":false,"target_va":5390951488,"va":5380285824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141546c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10733094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e826c6a300","size":5,"soft_fail":false,"target_va":5391019056,"va":5380285957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176c700
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12983437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e88d1cc600","size":5,"soft_fail":false,"target_va":5393270528,"va":5380287086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176d0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12987129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8f92ac600","size":5,"soft_fail":false,"target_va":5393273024,"va":5380285890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13002555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e83b67c600","size":5,"soft_fail":false,"target_va":5393288768,"va":5380286208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141792c60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13141238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8f684c800","size":5,"soft_fail":false,"target_va":5393427552,"va":5380286309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36770719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e89f133102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380286332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36771078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e806153102","size":5,"soft_fail":false,"target_va":5417057088,"va":5380286005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36335900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e81c712a02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380285695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1009552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e87098f0ff","size":5,"soft_fail":false,"target_va":5379276288,"va":5380285835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1006517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e84ba4f0ff","size":5,"soft_fail":false,"target_va":5379279280,"va":5380285792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36287742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8feb42902","size":5,"soft_fail":false,"target_va":5416574944,"va":5380287197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36284958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e81eaa2902","size":5,"soft_fail":false,"target_va":5416572144,"va":5380287181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3699449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8f9723800","size":5,"soft_fail":false,"target_va":5383985360,"va":5380285906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3262072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e878c63100","size":5,"soft_fail":false,"target_va":5383548144,"va":5380286067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3250283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e86b983100","size":5,"soft_fail":false,"target_va":5383536672,"va":5380286384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3252908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380284960,"raw_hex":"e8aca23100","size":5,"soft_fail":false,"target_va":5383539152,"va":5380286239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":560,"record_offset":11000,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":11540,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":10604,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_texture_processed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":560,"record_offset":11056,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":30218,"source_line":761,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":560,"record_offset":5124,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":11156,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":560,"record_offset":11116,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":11224,"parent_record_offset":4920,"ancestor_record_offset":4920,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":560,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":4920,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":560,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":4920,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":560,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":4920,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":560,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":4920,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":560,"record_offset":6428,"parent_record_offset":6388,"ancestor_record_offset":4920,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":560,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":4920,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":560,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":4920,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":560,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":4920,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":560,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":4920,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":560,"record_offset":6724,"parent_record_offset":6700,"ancestor_record_offset":4920,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":560,"record_offset":6672,"parent_record_offset":6640,"ancestor_record_offset":4920,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":6616,"parent_record_offset":6104,"ancestor_record_offset":4920,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":4920,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":6136,"parent_record_offset":6104,"ancestor_record_offset":4920,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":6700,"parent_record_offset":6640,"ancestor_record_offset":4920,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":560,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":4920,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":560,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":4920,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":560,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":4920,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":560,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":4920,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":560,"record_offset":6792,"parent_record_offset":6104,"ancestor_record_offset":4920,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":560,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":4920,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":560,"record_offset":6104,"parent_record_offset":5152,"ancestor_record_offset":4920,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":560,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4920,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":560,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":4920,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":560,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":4920,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":560,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":4920,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":560,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":4920,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":560,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":4920,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":560,"record_offset":5624,"parent_record_offset":5596,"ancestor_record_offset":4920,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":560,"record_offset":6276,"parent_record_offset":6212,"ancestor_record_offset":4920,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":560,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":4920,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":560,"record_offset":6340,"parent_record_offset":6160,"ancestor_record_offset":4920,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":560,"record_offset":6912,"parent_record_offset":5124,"ancestor_record_offset":4920,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":4920,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":10716,"parent_record_offset":10676,"ancestor_record_offset":4920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":560,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":4920,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":560,"record_offset":6212,"parent_record_offset":6184,"ancestor_record_offset":4920,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":560,"record_offset":6248,"parent_record_offset":6212,"ancestor_record_offset":4920,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":560,"record_offset":6568,"parent_record_offset":6388,"ancestor_record_offset":4920,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":4920,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":11720,"parent_record_offset":11596,"ancestor_record_offset":4920,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":10896,"parent_record_offset":10772,"ancestor_record_offset":4920,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":4920,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":4920,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":4920,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":10648,"parent_record_offset":10604,"ancestor_record_offset":4920,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":10968,"parent_record_offset":10604,"ancestor_record_offset":4920,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":11192,"parent_record_offset":11156,"ancestor_record_offset":4920,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs
//     evidence={"module":560,"record_offset":10676,"parent_record_offset":10604,"ancestor_record_offset":4920,"inlinee":35156,"source_line":35,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":560,"record_offset":11084,"parent_record_offset":11056,"ancestor_record_offset":4920,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":560,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":4920,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":560,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":4920,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":560,"record_offset":5596,"parent_record_offset":5564,"ancestor_record_offset":4920,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":560,"record_offset":5536,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":560,"record_offset":5396,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":560,"record_offset":5956,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":560,"record_offset":5480,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":560,"record_offset":5240,"parent_record_offset":5204,"ancestor_record_offset":4920,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":560,"record_offset":5804,"parent_record_offset":5696,"ancestor_record_offset":4920,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":4920,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":6048,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":4920,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":4920,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5268,"parent_record_offset":5240,"ancestor_record_offset":4920,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":4920,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5368,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":5696,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":560,"record_offset":5928,"parent_record_offset":5316,"ancestor_record_offset":4920,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":10744,"parent_record_offset":10716,"ancestor_record_offset":4920,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":11568,"parent_record_offset":11540,"ancestor_record_offset":4920,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":10808,"parent_record_offset":10772,"ancestor_record_offset":4920,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":4920,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":10772,"parent_record_offset":10744,"ancestor_record_offset":4920,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":560,"record_offset":10836,"parent_record_offset":10808,"ancestor_record_offset":4920,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8954
// 首条位置: {"module":560,"inlinee":35156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":10716,"parent_record_offset":10676,"ancestor_record_offset":10676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":10896,"parent_record_offset":10772,"ancestor_record_offset":10676,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":10676,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":10744,"parent_record_offset":10716,"ancestor_record_offset":10676,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":10808,"parent_record_offset":10772,"ancestor_record_offset":10676,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":10772,"parent_record_offset":10744,"ancestor_record_offset":10676,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":560,"record_offset":10836,"parent_record_offset":10808,"ancestor_record_offset":10676,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":560,"record_offset":268,"rva":11574448,"va":5380283568}
// 编译实例 RVAs: 0xb09cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7819412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e86caf88ff","size":5,"soft_fail":false,"target_va":5372465520,"va":5380284927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10694539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e88b2fa300","size":5,"soft_fail":false,"target_va":5390978720,"va":5380284176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10695179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e80b32a300","size":5,"soft_fail":false,"target_va":5390979312,"va":5380284128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153e5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10700474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8ba46a300","size":5,"soft_fail":false,"target_va":5390984640,"va":5380284161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153ea60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10701673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8694ba300","size":5,"soft_fail":false,"target_va":5390985824,"va":5380284146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415490d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10743627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e84befa300","size":5,"soft_fail":false,"target_va":5391028432,"va":5380284800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10745856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e800f8a300","size":5,"soft_fail":false,"target_va":5391030672,"va":5380284811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10746165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e835f9a300","size":5,"soft_fail":false,"target_va":5391030992,"va":5380284822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154b290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10752234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8ea10a400","size":5,"soft_fail":false,"target_va":5391037072,"va":5380284833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13004429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e88d6ec600","size":5,"soft_fail":false,"target_va":5393288768,"va":5380284334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141779660
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13038925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e84df5c600","size":5,"soft_fail":false,"target_va":5393323616,"va":5380284686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417963d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13157284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8a4c3c800","size":5,"soft_fail":false,"target_va":5393441744,"va":5380284455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36772573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8dd1a3102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380284478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36336733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e85d742a02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380284862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1004984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e848aaf0ff","size":5,"soft_fail":false,"target_va":5379279280,"va":5380284259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36290055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e807be2902","size":5,"soft_fail":false,"target_va":5416574944,"va":5380284884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3700592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e870773800","size":5,"soft_fail":false,"target_va":5383985360,"va":5380284763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3252147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8b39f3100","size":5,"soft_fail":false,"target_va":5383536672,"va":5380284520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3254782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380283568,"raw_hex":"e8fea93100","size":5,"soft_fail":false,"target_va":5383539152,"va":5380284365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":4092,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":560,"record_offset":448,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":560,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":560,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":560,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":560,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":560,"record_offset":1752,"parent_record_offset":1712,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":560,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":560,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":268,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":560,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":268,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":560,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":268,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":560,"record_offset":1996,"parent_record_offset":1964,"ancestor_record_offset":268,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":1940,"parent_record_offset":1428,"ancestor_record_offset":268,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":268,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":1460,"parent_record_offset":1428,"ancestor_record_offset":268,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":560,"record_offset":2024,"parent_record_offset":1964,"ancestor_record_offset":268,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":560,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":560,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":268,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":560,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":268,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":560,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":268,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":560,"record_offset":2116,"parent_record_offset":1428,"ancestor_record_offset":268,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":560,"record_offset":476,"parent_record_offset":448,"ancestor_record_offset":268,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":560,"record_offset":1428,"parent_record_offset":476,"ancestor_record_offset":268,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":560,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":268,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":560,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":268,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":560,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":268,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":560,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":560,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":560,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":560,"record_offset":948,"parent_record_offset":920,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":560,"record_offset":1600,"parent_record_offset":1536,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":560,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":560,"record_offset":1664,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":560,"record_offset":2236,"parent_record_offset":448,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":4520,"parent_record_offset":4272,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":560,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":560,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":560,"record_offset":1572,"parent_record_offset":1536,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":560,"record_offset":1892,"parent_record_offset":1712,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":268,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":4700,"parent_record_offset":4576,"ancestor_record_offset":268,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":268,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":268,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":268,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":268,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$23::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":4132,"parent_record_offset":4092,"ancestor_record_offset":268,"inlinee":30363,"source_line":593,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$23::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":560,"record_offset":4772,"parent_record_offset":4092,"ancestor_record_offset":268,"inlinee":30374,"source_line":596,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::spawn_lilypads::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":4364,"parent_record_offset":4328,"ancestor_record_offset":268,"inlinee":35152,"source_line":42101,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::spawn_reeds::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":268,"inlinee":35154,"source_line":42186,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs
//     evidence={"module":560,"record_offset":4272,"parent_record_offset":4092,"ancestor_record_offset":268,"inlinee":35150,"source_line":124,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":560,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":268,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":560,"record_offset":888,"parent_record_offset":860,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":560,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":560,"record_offset":860,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":560,"record_offset":720,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":560,"record_offset":1280,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":560,"record_offset":804,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":560,"record_offset":564,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":560,"record_offset":1128,"parent_record_offset":1020,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":1372,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":592,"parent_record_offset":564,"ancestor_record_offset":268,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":692,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":560,"record_offset":1020,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":560,"record_offset":1252,"parent_record_offset":640,"ancestor_record_offset":268,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":4548,"parent_record_offset":4520,"ancestor_record_offset":268,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":4612,"parent_record_offset":4576,"ancestor_record_offset":268,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":4576,"parent_record_offset":4548,"ancestor_record_offset":268,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":560,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":268,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":4328,"parent_record_offset":4272,"ancestor_record_offset":268,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":4396,"parent_record_offset":4272,"ancestor_record_offset":268,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x894E
// 首条位置: {"module":560,"inlinee":35150}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":4520,"parent_record_offset":4272,"ancestor_record_offset":4272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":4328,"parent_record_offset":4272,"ancestor_record_offset":4272,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":4396,"parent_record_offset":4272,"ancestor_record_offset":4272,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":4700,"parent_record_offset":4576,"ancestor_record_offset":4272,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":4272,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::spawn_lilypads::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":4364,"parent_record_offset":4328,"ancestor_record_offset":4272,"inlinee":35152,"source_line":42101,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::water::spawn_reeds::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":4272,"inlinee":35154,"source_line":42186,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":4548,"parent_record_offset":4520,"ancestor_record_offset":4272,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":4612,"parent_record_offset":4576,"ancestor_record_offset":4272,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":4576,"parent_record_offset":4548,"ancestor_record_offset":4272,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":560,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":4272,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00560.jsonl.gz"}
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
// 完整结构及原始类型记录: compute_water.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compute_water.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: compute_water.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":11192,"parent_record_offset":11156,"ancestor_record_offset":4920,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00560.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/spawn_lilypads","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_lilypads::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4364,"parent_record_offset":4328,"ancestor_record_offset":268,"inlinee":35152,"source_line":42101,"archive":"_pdb/C/modules/00560.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/spawn_reeds","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_reeds::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":268,"inlinee":35154,"source_line":42186,"archive":"_pdb/C/modules/00560.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/spawn_lilypads","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_lilypads::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4364,"parent_record_offset":4328,"ancestor_record_offset":4272,"inlinee":35152,"source_line":42101,"archive":"_pdb/C/modules/00560.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/water/spawn_reeds","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_reeds::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":4272,"inlinee":35154,"source_line":42186,"archive":"_pdb/C/modules/00560.jsonl.gz"}}
// [Shader 映射] {"shader_id":"c72bf15bc4cfc4fa8e7fce41","relative_path":"country-core/compiled/gauss_blur_mask.precomp","sha256":"aa16834285ace72b80dc53dd77ac8bd52c513192cde2a17739b96ca23b82e6e3","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu","hint":{"shader_family":"country_core/gauss_blur_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":11192,"parent_record_offset":11156,"ancestor_record_offset":4920,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00560.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_mask:main"}
// [Shader 映射] {"shader_id":"588ba7e6598445d1cfe9d206","relative_path":"country-core/compiled/water/spawn_lilypads.precomp","sha256":"6523e6dc09fb90dc3ea225e13f2100b16696aeb80439a233a947ea2848b7e213","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation","hint":{"shader_family":"country_core/water/spawn_lilypads","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_lilypads::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4364,"parent_record_offset":4328,"ancestor_record_offset":268,"inlinee":35152,"source_line":42101,"archive":"_pdb/C/modules/00560.jsonl.gz"}},"shader_uri":"country_core/water/spawn_lilypads:main"}
// [Shader 映射] {"shader_id":"c77e83f762ff7cf99a601cbd","relative_path":"country-core/compiled/water/spawn_reeds.precomp","sha256":"98ff82c610f7a03bd3b713c68c6b70062476be4138a6b40e809360938812a870","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation","hint":{"shader_family":"country_core/water/spawn_reeds","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_reeds::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":268,"inlinee":35154,"source_line":42186,"archive":"_pdb/C/modules/00560.jsonl.gz"}},"shader_uri":"country_core/water/spawn_reeds:main"}
// [Shader 映射] {"shader_id":"588ba7e6598445d1cfe9d206","relative_path":"country-core/compiled/water/spawn_lilypads.precomp","sha256":"6523e6dc09fb90dc3ea225e13f2100b16696aeb80439a233a947ea2848b7e213","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation::closure$0","hint":{"shader_family":"country_core/water/spawn_lilypads","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_lilypads::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4364,"parent_record_offset":4328,"ancestor_record_offset":4272,"inlinee":35152,"source_line":42101,"archive":"_pdb/C/modules/00560.jsonl.gz"}},"shader_uri":"country_core/water/spawn_lilypads:main"}
// [Shader 映射] {"shader_id":"c77e83f762ff7cf99a601cbd","relative_path":"country-core/compiled/water/spawn_reeds.precomp","sha256":"98ff82c610f7a03bd3b713c68c6b70062476be4138a6b40e809360938812a870","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation::closure$0","hint":{"shader_family":"country_core/water/spawn_reeds","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::water::spawn_reeds::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":560,"record_offset":4460,"parent_record_offset":4424,"ancestor_record_offset":4272,"inlinee":35154,"source_line":42186,"archive":"_pdb/C/modules/00560.jsonl.gz"}},"shader_uri":"country_core/water/spawn_reeds:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/spawn_lilypads.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[63]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":68,"name":"globalParams","storage_class":"Uniform","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,65]},"decorations":[],"pointee":{"id":65,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[20,7,66]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"animation_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"grid_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[12]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":211,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":210,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,208]},"decorations":[],"pointee":{"id":208,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":215,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":214,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,212]},"decorations":[],"pointee":{"id":212,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":227,"name":"path_mask","storage_class":"Uniform","type":{"id":226,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,225]},"decorations":[],"pointee":{"id":225,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":240,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":214,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,212]},"decorations":[],"pointee":{"id":212,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":252,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":210,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,208]},"decorations":[],"pointee":{"id":208,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":265,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":210,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,208]},"decorations":[],"pointee":{"id":208,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":13,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":276,"name":"frame_constants","storage_class":"Uniform","type":{"id":275,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,274]},"decorations":[],"pointee":{"id":274,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[219,219,219,219,219,219,219,219,10,10,10,6,7,6,10,25,10,25,6,25,6,25,10,219,10,6,7,10,6,6,6,25,10,6,6,6,6,6,10,10,10,219,25,10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":219,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":219,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":219,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":219,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":219,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":219,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":219,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":219,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":10,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":10,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":10,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":6,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":7,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":6,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":10,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":25,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":10,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":25,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":6,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":25,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":6,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":25,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":10,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":219,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":10,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":6,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":10,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":6,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":6,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":6,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":25,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":10,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":6,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":6,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":6,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":6,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":6,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":10,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":10,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":10,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":219,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":25,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":10,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":297,"name":"animation","storage_class":"UniformConstant","type":{"id":296,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,294]},"decorations":[],"pointee":{"id":294,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":791,"name":"draw_commands","storage_class":"Uniform","type":{"id":790,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,788]},"decorations":[],"pointee":{"id":788,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[789]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":789,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":815,"name":"lilypad_buffer","storage_class":"Uniform","type":{"id":814,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,812]},"decorations":[],"pointee":{"id":812,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[813]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":813,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1002,"name":"lilypad_flower_buffer","storage_class":"Uniform","type":{"id":814,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,812]},"decorations":[],"pointee":{"id":812,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[813]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":813,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":63,"name":"unknown","storage_class":"Input","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/gauss_blur_mask.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[68]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"globalParams","storage_class":"Uniform","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[41,41,5,8,5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"water_mask_dims","type_id":41,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"out_water_mask_dims","type_id":41,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"is_horizontal","type_id":5,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"sigma","type_id":8,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"output_channel","type_id":5,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":122,"name":"water_mask","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,119]},"decorations":[],"pointee":{"id":119,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":171,"name":"out_water_mask","storage_class":"UniformConstant","type":{"id":170,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,168]},"decorations":[],"pointee":{"id":168,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":68,"name":"unknown","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,65]},"decorations":[],"pointee":{"id":65,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/water/spawn_reeds.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[34]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"globalParams","storage_class":"Uniform","type":{"id":43,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[17,42]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":17,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grid_dims","type_id":42,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":171,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":170,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,168]},"decorations":[],"pointee":{"id":168,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":175,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":174,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,172]},"decorations":[],"pointee":{"id":172,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":187,"name":"path_mask","storage_class":"Uniform","type":{"id":186,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,185]},"decorations":[],"pointee":{"id":185,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":201,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":174,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,172]},"decorations":[],"pointee":{"id":172,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":213,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":170,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,168]},"decorations":[],"pointee":{"id":168,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":260,"name":"building_mask.non_blurred_tex","storage_class":"UniformConstant","type":{"id":170,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,168]},"decorations":[],"pointee":{"id":168,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":656,"name":"draw_commands","storage_class":"Uniform","type":{"id":655,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,653]},"decorations":[],"pointee":{"id":653,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[654]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":654,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":681,"name":"instance_buffer","storage_class":"Uniform","type":{"id":680,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,678]},"decorations":[],"pointee":{"id":678,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[679]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":679,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"unknown","storage_class":"Input","type":{"id":33,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,31]},"decorations":[],"pointee":{"id":31,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679429,"position_among_mapped_render_targets":9,"mapped_render_target_count":74,"previous_callsite":5679418,"next_callsite":5679440,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679684,"position_among_mapped_render_targets":21,"mapped_render_target_count":74,"previous_callsite":5679550,"next_callsite":5679695,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass2.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compute_water.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<array$<f32,2>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// 候选参数类型: ["ref$<enum2$<serde_json::value::Value> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<array$<f32,3>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// 候选参数类型: ["ref$<enum2$<serde_json::value::Value> >"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
// 信息 ID: cdaf74eb0276c26fbfae6d45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a945 -> 0xb0a220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu::closure$0
// 信息 ID: 1e5941d1bdcc863e6c1753d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=560, inline_record=10676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
// 信息 ID: d1a139e45de1ed0259dac16f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56aa44 -> 0xb09cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation::closure$0
// 信息 ID: 8e74a7b5b2e8007ba6528aad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation; module=560, inline_record=4272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
