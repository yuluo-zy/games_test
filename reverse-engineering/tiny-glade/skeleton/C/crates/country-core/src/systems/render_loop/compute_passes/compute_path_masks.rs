// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cc47160559504d7f18e133c6f87907033db88f30f8e2cf37da0179133a19b775
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compute_path_masks.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 517
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
// 函数符号: country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":69032,"rva":10180848,"va":5378889968}
// 编译实例 RVAs: 0x9b58f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4580229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e885e34500","size":5,"soft_fail":false,"target_va":5383472768,"va":5378892534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395120
//     owner=unresolved_namespace; count=7
//     target_aliases=18 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6429704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8f8e39dff","size":5,"soft_fail":false,"target_va":5372465440,"va":5378895139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=4
//     target_aliases=49 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6430008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8c8e29dff","size":5,"soft_fail":false,"target_va":5372465520,"va":5378895523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176c700
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14376306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8725ddb00","size":5,"soft_fail":false,"target_va":5393270528,"va":5378894217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176df80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14382628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e82476db00","size":5,"soft_fail":false,"target_va":5393276800,"va":5378894167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14396567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e897acdb00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378892196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177e9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14450934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8f680dc00","size":5,"soft_fail":false,"target_va":5393344992,"va":5378894053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141785dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14480869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8e5f5dc00","size":5,"soft_fail":false,"target_va":5393374656,"va":5378893782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14178acb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14502552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8984add00","size":5,"soft_fail":false,"target_va":5393394864,"va":5378892307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141792670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14533094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8e6c1dd00","size":5,"soft_fail":false,"target_va":5393426032,"va":5378892933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141798890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14557974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e81623de00","size":5,"soft_fail":false,"target_va":5393451152,"va":5378893173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38164718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8ee584602","size":5,"soft_fail":false,"target_va":5417057056,"va":5378892333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38165082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e85a5a4602","size":5,"soft_fail":false,"target_va":5417057088,"va":5378892001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t37730620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e83cb93f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378890975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t386006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8d6e30500","size":5,"soft_fail":false,"target_va":5379276288,"va":5378890277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t388863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8ffee0500","size":5,"soft_fail":false,"target_va":5379279280,"va":5378890412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e806efffff","size":5,"soft_fail":false,"target_va":5378889152,"va":5378893493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37679999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e87ff33e02","size":5,"soft_fail":false,"target_va":5416574944,"va":5378894940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37677137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e851e83e02","size":5,"soft_fail":false,"target_va":5416572144,"va":5378895002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=22
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5091482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e89ab04d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378893873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4655670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8360a4700","size":5,"soft_fail":false,"target_va":5383548144,"va":5378892469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4644282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e8badd4600","size":5,"soft_fail":false,"target_va":5383536672,"va":5378892385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4646913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889968,"raw_hex":"e801e84600","size":5,"soft_fail":false,"target_va":5383539152,"va":5378892234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":517,"record_offset":80024,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":81116,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_texture_processed
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":517,"record_offset":80080,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":30218,"source_line":761,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::compute_path_masks::blur_exclusion_mask
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs
//     evidence={"module":517,"record_offset":80584,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":30250,"source_line":176,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":78388,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":69236,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":80488,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":80556,"parent_record_offset":69032,"ancestor_record_offset":69032,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":70588,"parent_record_offset":70564,"ancestor_record_offset":69032,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":70636,"parent_record_offset":70612,"ancestor_record_offset":69032,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":70612,"parent_record_offset":70588,"ancestor_record_offset":69032,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":70564,"parent_record_offset":70540,"ancestor_record_offset":69032,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":70540,"parent_record_offset":70500,"ancestor_record_offset":69032,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":70500,"parent_record_offset":70476,"ancestor_record_offset":69032,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":517,"record_offset":80048,"parent_record_offset":80024,"ancestor_record_offset":69032,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":69292,"parent_record_offset":69264,"ancestor_record_offset":69032,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":70860,"parent_record_offset":70836,"ancestor_record_offset":69032,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":70836,"parent_record_offset":70812,"ancestor_record_offset":69032,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":70784,"parent_record_offset":70752,"ancestor_record_offset":69032,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":79892,"parent_record_offset":79384,"ancestor_record_offset":69032,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":70728,"parent_record_offset":70216,"ancestor_record_offset":69032,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":70752,"parent_record_offset":70728,"ancestor_record_offset":69032,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":70248,"parent_record_offset":70216,"ancestor_record_offset":69032,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":70812,"parent_record_offset":70752,"ancestor_record_offset":69032,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":69032,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":69032,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":70476,"parent_record_offset":70452,"ancestor_record_offset":69032,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":517,"record_offset":70928,"parent_record_offset":70904,"ancestor_record_offset":69032,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":70904,"parent_record_offset":70216,"ancestor_record_offset":69032,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":78424,"parent_record_offset":78388,"ancestor_record_offset":69032,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":79384,"parent_record_offset":78424,"ancestor_record_offset":69032,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":69264,"parent_record_offset":69236,"ancestor_record_offset":69032,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":70216,"parent_record_offset":69264,"ancestor_record_offset":69032,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":69316,"parent_record_offset":69292,"ancestor_record_offset":69032,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":69964,"parent_record_offset":69940,"ancestor_record_offset":69032,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":69992,"parent_record_offset":69964,"ancestor_record_offset":69032,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":69532,"parent_record_offset":69508,"ancestor_record_offset":69032,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":69616,"parent_record_offset":69592,"ancestor_record_offset":69032,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":69764,"parent_record_offset":69736,"ancestor_record_offset":69032,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":69736,"parent_record_offset":69708,"ancestor_record_offset":69032,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":70388,"parent_record_offset":70324,"ancestor_record_offset":69032,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":70412,"parent_record_offset":70388,"ancestor_record_offset":69032,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":70452,"parent_record_offset":70272,"ancestor_record_offset":69032,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":71024,"parent_record_offset":69236,"ancestor_record_offset":69032,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":69556,"parent_record_offset":69532,"ancestor_record_offset":69032,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":69880,"parent_record_offset":69856,"ancestor_record_offset":69032,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":70324,"parent_record_offset":70296,"ancestor_record_offset":69032,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":70360,"parent_record_offset":70324,"ancestor_record_offset":69032,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":70680,"parent_record_offset":70500,"ancestor_record_offset":69032,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":70952,"parent_record_offset":70928,"ancestor_record_offset":69032,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":81640,"parent_record_offset":81516,"ancestor_record_offset":69032,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":81296,"parent_record_offset":81172,"ancestor_record_offset":69032,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":81664,"parent_record_offset":81640,"ancestor_record_offset":69032,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":70976,"parent_record_offset":70952,"ancestor_record_offset":69032,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":81320,"parent_record_offset":81296,"ancestor_record_offset":69032,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":80860,"parent_record_offset":80824,"ancestor_record_offset":69032,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::paths::combine_path_exclusions::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":80524,"parent_record_offset":80488,"ancestor_record_offset":69032,"inlinee":30248,"source_line":30821,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::paths::directional_contract::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":80764,"parent_record_offset":80728,"ancestor_record_offset":69032,"inlinee":30252,"source_line":30883,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::paths::extend_path_mask_margin::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":81400,"parent_record_offset":81364,"ancestor_record_offset":69032,"inlinee":30256,"source_line":30933,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::paths::path_mask_visual::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":81056,"parent_record_offset":81020,"ancestor_record_offset":69032,"inlinee":30254,"source_line":31110,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":517,"record_offset":80108,"parent_record_offset":80080,"ancestor_record_offset":69032,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":69940,"parent_record_offset":69916,"ancestor_record_offset":69032,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":69676,"parent_record_offset":69648,"ancestor_record_offset":69032,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":69708,"parent_record_offset":69676,"ancestor_record_offset":69032,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":69648,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":69508,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":70068,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":69592,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":69352,"parent_record_offset":69316,"ancestor_record_offset":69032,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":69916,"parent_record_offset":69808,"ancestor_record_offset":69032,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69856,"parent_record_offset":69832,"ancestor_record_offset":69032,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":70160,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69832,"parent_record_offset":69808,"ancestor_record_offset":69032,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69404,"parent_record_offset":69380,"ancestor_record_offset":69032,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69380,"parent_record_offset":69352,"ancestor_record_offset":69032,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69428,"parent_record_offset":69404,"ancestor_record_offset":69032,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69480,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":69808,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":70040,"parent_record_offset":69428,"ancestor_record_offset":69032,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":517,"record_offset":81144,"parent_record_offset":81116,"ancestor_record_offset":69032,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":517,"record_offset":81488,"parent_record_offset":81460,"ancestor_record_offset":69032,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":81208,"parent_record_offset":81172,"ancestor_record_offset":69032,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":81516,"parent_record_offset":81488,"ancestor_record_offset":69032,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":81172,"parent_record_offset":81144,"ancestor_record_offset":69032,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":517,"record_offset":81236,"parent_record_offset":81208,"ancestor_record_offset":69032,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":80728,"parent_record_offset":80584,"ancestor_record_offset":69032,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":80640,"parent_record_offset":80584,"ancestor_record_offset":69032,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":80796,"parent_record_offset":80584,"ancestor_record_offset":69032,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":68576,"rva":10180032,"va":5378889152}
// 编译实例 RVAs: 0x9b55c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4583449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889152,"raw_hex":"e819f04500","size":5,"soft_fail":false,"target_va":5383472768,"va":5378889314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14399161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889152,"raw_hex":"e8b9b6db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378889602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141798890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_path_masks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14561460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889152,"raw_hex":"e8b430de00","size":5,"soft_fail":false,"target_va":5393451152,"va":5378889687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4658886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889152,"raw_hex":"e8c6164700","size":5,"soft_fail":false,"target_va":5383548144,"va":5378889253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4646924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889152,"raw_hex":"e80ce84600","size":5,"soft_fail":false,"target_va":5383536672,"va":5378889743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4649512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378889152,"raw_hex":"e828f24600","size":5,"soft_fail":false,"target_va":5383539152,"va":5378889635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":68836,"parent_record_offset":68576,"ancestor_record_offset":68576,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":68756,"parent_record_offset":68576,"ancestor_record_offset":68576,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":68904,"parent_record_offset":68576,"ancestor_record_offset":68576,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":68872,"parent_record_offset":68836,"ancestor_record_offset":68576,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_path_masks::blur_exclusion_mask
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x762A
// 首条位置: {"module":517,"inlinee":30250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":80728,"parent_record_offset":80584,"ancestor_record_offset":80584,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":80640,"parent_record_offset":80584,"ancestor_record_offset":80584,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":80796,"parent_record_offset":80584,"ancestor_record_offset":80584,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":80860,"parent_record_offset":80824,"ancestor_record_offset":80584,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::paths::directional_contract::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":80764,"parent_record_offset":80728,"ancestor_record_offset":80584,"inlinee":30252,"source_line":30883,"archive":"_pdb/C/modules/00517.jsonl.gz"}
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
// 完整结构及原始类型记录: compute_path_masks.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compute_path_masks.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: compute_path_masks.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks
// [PDB Shader 类型/入口] {"shader_family":"country_core/paths/combine_path_exclusions","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::combine_path_exclusions::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80524,"parent_record_offset":80488,"ancestor_record_offset":69032,"inlinee":30248,"source_line":30821,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/paths/directional_contract","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::directional_contract::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80764,"parent_record_offset":80728,"ancestor_record_offset":69032,"inlinee":30252,"source_line":30883,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_r16f","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80860,"parent_record_offset":80824,"ancestor_record_offset":69032,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/paths/path_mask_visual","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::path_mask_visual::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":81056,"parent_record_offset":81020,"ancestor_record_offset":69032,"inlinee":30254,"source_line":31110,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/paths/extend_path_mask_margin","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::extend_path_mask_margin::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":81400,"parent_record_offset":81364,"ancestor_record_offset":69032,"inlinee":30256,"source_line":30933,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_r16f","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":68872,"parent_record_offset":68836,"ancestor_record_offset":68576,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_path_masks::blur_exclusion_mask
// [PDB Shader 类型/入口] {"shader_family":"country_core/paths/directional_contract","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::directional_contract::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80764,"parent_record_offset":80728,"ancestor_record_offset":80584,"inlinee":30252,"source_line":30883,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_r16f","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80860,"parent_record_offset":80824,"ancestor_record_offset":80584,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [Shader 映射] {"shader_id":"6930cb3a2a2cd92e8feae308","relative_path":"country-core/compiled/paths/combine_path_exclusions.precomp","sha256":"b29eaea1a090dbe15b2b79ddc46fc481baebadc67deba7aa3e9d6aed245d76c3","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks","hint":{"shader_family":"country_core/paths/combine_path_exclusions","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::combine_path_exclusions::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80524,"parent_record_offset":80488,"ancestor_record_offset":69032,"inlinee":30248,"source_line":30821,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/paths/combine_path_exclusions:main"}
// [Shader 映射] {"shader_id":"21b9a5b3ec2c0d4638d3532e","relative_path":"country-core/compiled/paths/directional_contract.precomp","sha256":"d610aefdc63dc9373b23baac19b2052227823c3a0c21873d5b89653fc5e4eadf","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks","hint":{"shader_family":"country_core/paths/directional_contract","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::directional_contract::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80764,"parent_record_offset":80728,"ancestor_record_offset":69032,"inlinee":30252,"source_line":30883,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/paths/directional_contract:main"}
// [Shader 映射] {"shader_id":"326076139cebd1c0d704f846","relative_path":"country-core/compiled/gauss_blur_r16f.precomp","sha256":"a4aeb6b5bafa106a6888163b47a339146849f0072c64d74c168a8f75a5b9eb80","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks","hint":{"shader_family":"country_core/gauss_blur_r16f","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80860,"parent_record_offset":80824,"ancestor_record_offset":69032,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_r16f:main"}
// [Shader 映射] {"shader_id":"6286a713e2a33d9e6908349c","relative_path":"country-core/compiled/paths/path_mask_visual.precomp","sha256":"c719b6af6af47175510fc8a71570f9a921e326c7d02bc2e6288c07a844172b3e","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks","hint":{"shader_family":"country_core/paths/path_mask_visual","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::path_mask_visual::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":81056,"parent_record_offset":81020,"ancestor_record_offset":69032,"inlinee":30254,"source_line":31110,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/paths/path_mask_visual:main"}
// [Shader 映射] {"shader_id":"a744d26bb17e21cbc76c7d2f","relative_path":"country-core/compiled/paths/extend_path_mask_margin.precomp","sha256":"bceee44da3b9d43b9b0b631c34cbbc21d61fdcb72e9a1eb2f1cb6bf8e2832ea2","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks","hint":{"shader_family":"country_core/paths/extend_path_mask_margin","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::extend_path_mask_margin::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":81400,"parent_record_offset":81364,"ancestor_record_offset":69032,"inlinee":30256,"source_line":30933,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/paths/extend_path_mask_margin:main"}
// [Shader 映射] {"shader_id":"326076139cebd1c0d704f846","relative_path":"country-core/compiled/gauss_blur_r16f.precomp","sha256":"a4aeb6b5bafa106a6888163b47a339146849f0072c64d74c168a8f75a5b9eb80","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask","hint":{"shader_family":"country_core/gauss_blur_r16f","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":68872,"parent_record_offset":68836,"ancestor_record_offset":68576,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_r16f:main"}
// [Shader 映射] {"shader_id":"21b9a5b3ec2c0d4638d3532e","relative_path":"country-core/compiled/paths/directional_contract.precomp","sha256":"d610aefdc63dc9373b23baac19b2052227823c3a0c21873d5b89653fc5e4eadf","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::blur_exclusion_mask","hint":{"shader_family":"country_core/paths/directional_contract","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::paths::directional_contract::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80764,"parent_record_offset":80728,"ancestor_record_offset":80584,"inlinee":30252,"source_line":30883,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/paths/directional_contract:main"}
// [Shader 映射] {"shader_id":"326076139cebd1c0d704f846","relative_path":"country-core/compiled/gauss_blur_r16f.precomp","sha256":"a4aeb6b5bafa106a6888163b47a339146849f0072c64d74c168a8f75a5b9eb80","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_path_masks::blur_exclusion_mask","hint":{"shader_family":"country_core/gauss_blur_r16f","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_r16f::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":80860,"parent_record_offset":80824,"ancestor_record_offset":80584,"inlinee":28105,"source_line":8806,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_r16f:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/paths/directional_contract.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[35]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"terrain_sdg","storage_class":"UniformConstant","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"path_mask","storage_class":"UniformConstant","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":73,"name":"img_out","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":79,"name":"globalParams","storage_class":"Uniform","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,77]},"decorations":[],"pointee":{"id":77,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[7,7,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"path_mask_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"img_out_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"terrain_sdg_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":123,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,120]},"decorations":[],"pointee":{"id":120,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[7,121,121,10,11]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":121,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":121,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":10,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":11,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[5]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":187,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":5,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":191,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":190,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,188]},"decorations":[],"pointee":{"id":188,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":262,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":190,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,188]},"decorations":[],"pointee":{"id":188,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"unknown","storage_class":"Input","type":{"id":34,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,32]},"decorations":[],"pointee":{"id":32,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/gauss_blur_r16f.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[64]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"globalParams","storage_class":"Uniform","type":{"id":31,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,28]},"decorations":[],"pointee":{"id":28,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[30,30,5,8]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"img_in_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"img_out_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"is_horizontal","type_id":5,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"sigma","type_id":8,"decorations":[{"decoration":"Offset","operands":[20]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":111,"name":"img_in","storage_class":"UniformConstant","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,108]},"decorations":[],"pointee":{"id":108,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":140,"name":"img_out","storage_class":"UniformConstant","type":{"id":139,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,137]},"decorations":[],"pointee":{"id":137,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":64,"name":"unknown","storage_class":"Input","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[29,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/paths/path_mask_visual.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[28]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"globalParams","storage_class":"Uniform","type":{"id":16,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[7,7,7,7,7,7,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"paths_visual_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"dirt_mask_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"path_mask_blurred_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"cobblestone1_mask_blurred_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"cobblestone2_mask_blurred_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"cobblestone3_mask_blurred_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"rocky_terrain_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":63,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,60]},"decorations":[],"pointee":{"id":60,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[7,61,61,10,11]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":61,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":61,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":10,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":11,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[5]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":129,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,126]},"decorations":[],"pointee":{"id":126,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":133,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":5,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":470,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,126]},"decorations":[],"pointee":{"id":126,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":482,"name":"path_mask_blurred","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":516,"name":"cobblestone1_mask_blurred","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":550,"name":"cobblestone2_mask_blurred","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":6,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[6]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":584,"name":"cobblestone3_mask_blurred","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":7,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[7]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":669,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":688,"name":"paths_visual","storage_class":"UniformConstant","type":{"id":687,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,685]},"decorations":[],"pointee":{"id":685,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":739,"name":"dirt_mask","storage_class":"UniformConstant","type":{"id":738,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,736]},"decorations":[],"pointee":{"id":736,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"unknown","storage_class":"Input","type":{"id":27,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/paths/combine_path_exclusions.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[9]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"path_mask","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"cobblestone1_mask","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"cobblestone2_mask","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":51,"name":"cobblestone3_mask","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"paths_exclusion","storage_class":"UniformConstant","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,59]},"decorations":[],"pointee":{"id":59,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/paths/extend_path_mask_margin.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[20]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"globalParams","storage_class":"Uniform","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[7,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"paths_visual_src_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"paths_visual_margin_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":129,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":128,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,126]},"decorations":[],"pointee":{"id":126,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":133,"name":"paths_visual_src","storage_class":"UniformConstant","type":{"id":132,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,130]},"decorations":[],"pointee":{"id":130,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":144,"name":"paths_visual_margin","storage_class":"UniformConstant","type":{"id":143,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,141]},"decorations":[],"pointee":{"id":141,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,2,4]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":20,"name":"unknown","storage_class":"Input","type":{"id":19,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,17]},"decorations":[],"pointee":{"id":17,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679451,"position_among_mapped_render_targets":11,"mapped_render_target_count":74,"previous_callsite":5679440,"next_callsite":5679462,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10180848,"caller_aliases":{"alias_group_rva":10180848,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10184373,"position_among_mapped_render_targets":1,"mapped_render_target_count":4,"previous_callsite":null,"next_callsite":10184392,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10180848,"caller_aliases":{"alias_group_rva":10180848,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10184392,"position_among_mapped_render_targets":2,"mapped_render_target_count":4,"previous_callsite":10184373,"next_callsite":10184414,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10180848,"caller_aliases":{"alias_group_rva":10180848,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10184414,"position_among_mapped_render_targets":3,"mapped_render_target_count":4,"previous_callsite":10184392,"next_callsite":10184440,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10180848,"caller_aliases":{"alias_group_rva":10180848,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10184440,"position_among_mapped_render_targets":4,"mapped_render_target_count":4,"previous_callsite":10184414,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compute_path_masks.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_path_masks::blur_exclusion_mask
// 信息 ID: 4cb6b1057f2ec8701f524209
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_path_masks::blur_path_mask
// 信息 ID: f46295d152b9b97c53e2a34f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks @ 0x9b66b5 -> 0x9b55c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks @ 0x9b66c8 -> 0x9b55c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks @ 0x9b66de -> 0x9b55c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks @ 0x9b66f8 -> 0x9b55c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks
// 信息 ID: f87756fb5e0667f0f70d5aba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a95b -> 0x9b58f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
