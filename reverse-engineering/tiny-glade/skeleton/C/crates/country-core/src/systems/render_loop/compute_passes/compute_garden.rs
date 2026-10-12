// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c996f17df0492f4caee6734cd4c611739485bad1cab23ad71c23600e634a4c90
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compute_garden.rs.pdb.jsonl.gz
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
// use typed_index_collections; // namespace candidate
//   package identities: [{"name":"typed-index-collections","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":47112,"rva":10174656,"va":5378883776}
// 编译实例 RVAs: 0x9b40c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4588187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e89b024600","size":5,"soft_fail":false,"target_va":5383472768,"va":5378884576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6419800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8a80a9eff","size":5,"soft_fail":false,"target_va":5372465520,"va":5378885315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176df80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14391691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e88b99db00","size":5,"soft_fail":false,"target_va":5393276800,"va":5378885104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14404081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8f1c9db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378884682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141792c60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14542768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8b0e7dd00","size":5,"soft_fail":false,"target_va":5393427552,"va":5378884779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38172249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e859764602","size":5,"soft_fail":false,"target_va":5417057056,"va":5378884802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38172616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8c8774602","size":5,"soft_fail":false,"target_va":5417057088,"va":5378884467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37487013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8a5013c02","size":5,"soft_fail":false,"target_va":5416370864,"va":5378883846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37736361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8a9cf3f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378885234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t394867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e873060600","size":5,"soft_fail":false,"target_va":5379279280,"va":5378884408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37689683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e853193f02","size":5,"soft_fail":false,"target_va":5416574944,"va":5378885256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5100235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e8cbd24d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378885120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4663610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e83a294700","size":5,"soft_fail":false,"target_va":5383548144,"va":5378884529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4651813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e825fb4600","size":5,"soft_fail":false,"target_va":5383536672,"va":5378884854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4654434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378883776,"raw_hex":"e862054700","size":5,"soft_fail":false,"target_va":5383539152,"va":5378884713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":47292,"parent_record_offset":47112,"ancestor_record_offset":47112,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":47456,"parent_record_offset":47424,"ancestor_record_offset":47112,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":47504,"parent_record_offset":47480,"ancestor_record_offset":47112,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":47480,"parent_record_offset":47456,"ancestor_record_offset":47112,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":47424,"parent_record_offset":47388,"ancestor_record_offset":47112,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":49004,"parent_record_offset":48980,"ancestor_record_offset":47112,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":48980,"parent_record_offset":48940,"ancestor_record_offset":47112,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":47112,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":47388,"parent_record_offset":47364,"ancestor_record_offset":47112,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":48940,"parent_record_offset":48916,"ancestor_record_offset":47112,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":517,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":47112,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":517,"record_offset":51320,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":47732,"parent_record_offset":47704,"ancestor_record_offset":47112,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":49300,"parent_record_offset":49276,"ancestor_record_offset":47112,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":49276,"parent_record_offset":49252,"ancestor_record_offset":47112,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":49224,"parent_record_offset":49192,"ancestor_record_offset":47112,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":49168,"parent_record_offset":48656,"ancestor_record_offset":47112,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":49192,"parent_record_offset":49168,"ancestor_record_offset":47112,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":48688,"parent_record_offset":48656,"ancestor_record_offset":47112,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":49252,"parent_record_offset":49192,"ancestor_record_offset":47112,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":48736,"parent_record_offset":48712,"ancestor_record_offset":47112,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":48712,"parent_record_offset":48688,"ancestor_record_offset":47112,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":48916,"parent_record_offset":48892,"ancestor_record_offset":47112,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":517,"record_offset":49368,"parent_record_offset":49344,"ancestor_record_offset":47112,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":49344,"parent_record_offset":48656,"ancestor_record_offset":47112,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":47704,"parent_record_offset":47676,"ancestor_record_offset":47112,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":48656,"parent_record_offset":47704,"ancestor_record_offset":47112,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":47756,"parent_record_offset":47732,"ancestor_record_offset":47112,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":48404,"parent_record_offset":48380,"ancestor_record_offset":47112,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":48432,"parent_record_offset":48404,"ancestor_record_offset":47112,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":47972,"parent_record_offset":47948,"ancestor_record_offset":47112,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":48056,"parent_record_offset":48032,"ancestor_record_offset":47112,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":48204,"parent_record_offset":48176,"ancestor_record_offset":47112,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":48176,"parent_record_offset":48148,"ancestor_record_offset":47112,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":48828,"parent_record_offset":48764,"ancestor_record_offset":47112,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":48852,"parent_record_offset":48828,"ancestor_record_offset":47112,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":48892,"parent_record_offset":48712,"ancestor_record_offset":47112,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":49464,"parent_record_offset":47676,"ancestor_record_offset":47112,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":47996,"parent_record_offset":47972,"ancestor_record_offset":47112,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":51668,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":48320,"parent_record_offset":48296,"ancestor_record_offset":47112,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":47544,"parent_record_offset":47388,"ancestor_record_offset":47112,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":48764,"parent_record_offset":48736,"ancestor_record_offset":47112,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":48800,"parent_record_offset":48764,"ancestor_record_offset":47112,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":49120,"parent_record_offset":48940,"ancestor_record_offset":47112,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":49392,"parent_record_offset":49368,"ancestor_record_offset":47112,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":51848,"parent_record_offset":51724,"ancestor_record_offset":47112,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":47112,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":49416,"parent_record_offset":49392,"ancestor_record_offset":47112,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::on_texture_processed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":517,"record_offset":51376,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":30218,"source_line":761,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":51512,"parent_record_offset":51476,"ancestor_record_offset":47112,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
//     evidence={"module":517,"record_offset":47584,"parent_record_offset":47292,"ancestor_record_offset":47112,"inlinee":30217,"source_line":28,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":517,"record_offset":51404,"parent_record_offset":51376,"ancestor_record_offset":47112,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":47676,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":48380,"parent_record_offset":48356,"ancestor_record_offset":47112,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":48116,"parent_record_offset":48088,"ancestor_record_offset":47112,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":48148,"parent_record_offset":48116,"ancestor_record_offset":47112,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":48088,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":47948,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":48508,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":48032,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":47792,"parent_record_offset":47756,"ancestor_record_offset":47112,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":48356,"parent_record_offset":48248,"ancestor_record_offset":47112,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48296,"parent_record_offset":48272,"ancestor_record_offset":47112,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48600,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48272,"parent_record_offset":48248,"ancestor_record_offset":47112,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47844,"parent_record_offset":47820,"ancestor_record_offset":47112,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47820,"parent_record_offset":47792,"ancestor_record_offset":47112,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47868,"parent_record_offset":47844,"ancestor_record_offset":47112,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47920,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48248,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":48480,"parent_record_offset":47868,"ancestor_record_offset":47112,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":517,"record_offset":51696,"parent_record_offset":51668,"ancestor_record_offset":47112,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":51760,"parent_record_offset":51724,"ancestor_record_offset":47112,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":51724,"parent_record_offset":51696,"ancestor_record_offset":47112,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":517,"record_offset":51788,"parent_record_offset":51760,"ancestor_record_offset":47112,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":51476,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":51436,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":51544,"parent_record_offset":47584,"ancestor_record_offset":47112,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":517,"record_offset":47340,"parent_record_offset":47292,"ancestor_record_offset":47112,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7609
// 首条位置: {"module":517,"inlinee":30217}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":517,"record_offset":51320,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":51668,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::on_texture_processed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":517,"record_offset":51376,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":30218,"source_line":761,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":47676,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":51476,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":51436,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":51544,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":49028,"parent_record_offset":49004,"ancestor_record_offset":47584,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":49076,"parent_record_offset":49052,"ancestor_record_offset":47584,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":49052,"parent_record_offset":49028,"ancestor_record_offset":47584,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":49004,"parent_record_offset":48980,"ancestor_record_offset":47584,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":48980,"parent_record_offset":48940,"ancestor_record_offset":47584,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":48940,"parent_record_offset":48916,"ancestor_record_offset":47584,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":517,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":47584,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":47732,"parent_record_offset":47704,"ancestor_record_offset":47584,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":49300,"parent_record_offset":49276,"ancestor_record_offset":47584,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":49276,"parent_record_offset":49252,"ancestor_record_offset":47584,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":49224,"parent_record_offset":49192,"ancestor_record_offset":47584,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":49168,"parent_record_offset":48656,"ancestor_record_offset":47584,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":49192,"parent_record_offset":49168,"ancestor_record_offset":47584,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":48688,"parent_record_offset":48656,"ancestor_record_offset":47584,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":49252,"parent_record_offset":49192,"ancestor_record_offset":47584,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":48736,"parent_record_offset":48712,"ancestor_record_offset":47584,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":48712,"parent_record_offset":48688,"ancestor_record_offset":47584,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":48916,"parent_record_offset":48892,"ancestor_record_offset":47584,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":517,"record_offset":49368,"parent_record_offset":49344,"ancestor_record_offset":47584,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":49344,"parent_record_offset":48656,"ancestor_record_offset":47584,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":47704,"parent_record_offset":47676,"ancestor_record_offset":47584,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":48656,"parent_record_offset":47704,"ancestor_record_offset":47584,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":47756,"parent_record_offset":47732,"ancestor_record_offset":47584,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":48404,"parent_record_offset":48380,"ancestor_record_offset":47584,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":48432,"parent_record_offset":48404,"ancestor_record_offset":47584,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":47972,"parent_record_offset":47948,"ancestor_record_offset":47584,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":48056,"parent_record_offset":48032,"ancestor_record_offset":47584,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":48204,"parent_record_offset":48176,"ancestor_record_offset":47584,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":48176,"parent_record_offset":48148,"ancestor_record_offset":47584,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":48828,"parent_record_offset":48764,"ancestor_record_offset":47584,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":48852,"parent_record_offset":48828,"ancestor_record_offset":47584,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":48892,"parent_record_offset":48712,"ancestor_record_offset":47584,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":49464,"parent_record_offset":47676,"ancestor_record_offset":47584,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":47996,"parent_record_offset":47972,"ancestor_record_offset":47584,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":48320,"parent_record_offset":48296,"ancestor_record_offset":47584,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":48764,"parent_record_offset":48736,"ancestor_record_offset":47584,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":48800,"parent_record_offset":48764,"ancestor_record_offset":47584,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":49120,"parent_record_offset":48940,"ancestor_record_offset":47584,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":49392,"parent_record_offset":49368,"ancestor_record_offset":47584,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":51848,"parent_record_offset":51724,"ancestor_record_offset":47584,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":47584,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":49416,"parent_record_offset":49392,"ancestor_record_offset":47584,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":51512,"parent_record_offset":51476,"ancestor_record_offset":47584,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":517,"record_offset":51404,"parent_record_offset":51376,"ancestor_record_offset":47584,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":48380,"parent_record_offset":48356,"ancestor_record_offset":47584,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":48116,"parent_record_offset":48088,"ancestor_record_offset":47584,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":48148,"parent_record_offset":48116,"ancestor_record_offset":47584,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":48088,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":47948,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":48508,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":48032,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":47792,"parent_record_offset":47756,"ancestor_record_offset":47584,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":48356,"parent_record_offset":48248,"ancestor_record_offset":47584,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48296,"parent_record_offset":48272,"ancestor_record_offset":47584,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48600,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48272,"parent_record_offset":48248,"ancestor_record_offset":47584,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47844,"parent_record_offset":47820,"ancestor_record_offset":47584,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47820,"parent_record_offset":47792,"ancestor_record_offset":47584,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47868,"parent_record_offset":47844,"ancestor_record_offset":47584,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":47920,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":48248,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":48480,"parent_record_offset":47868,"ancestor_record_offset":47584,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":517,"record_offset":51696,"parent_record_offset":51668,"ancestor_record_offset":47584,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":51760,"parent_record_offset":51724,"ancestor_record_offset":47584,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":517,"record_offset":51724,"parent_record_offset":51696,"ancestor_record_offset":47584,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":517,"record_offset":51788,"parent_record_offset":51760,"ancestor_record_offset":47584,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":43720,"rva":10173200,"va":5378882320}
// 编译实例 RVAs: 0x9b3b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14153e5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12101879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8f7a8b800","size":5,"soft_fail":false,"target_va":5390984640,"va":5378882756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153ecb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12103722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e82ab0b800","size":5,"soft_fail":false,"target_va":5390986416,"va":5378882689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153f3a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12105482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e80ab7b800","size":5,"soft_fail":false,"target_va":5390988192,"va":5378882705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141540180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12109018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8dac4b800","size":5,"soft_fail":false,"target_va":5390991744,"va":5378882721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141540d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12111962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e85ad0b800","size":5,"soft_fail":false,"target_va":5390994704,"va":5378882737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12146692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e80458b900","size":5,"soft_fail":false,"target_va":5391030352,"va":5378883655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12147310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e86e5ab900","size":5,"soft_fail":false,"target_va":5391030992,"va":5378883677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154a610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12150223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8cf65b900","size":5,"soft_fail":false,"target_va":5391033872,"va":5378883644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154b650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12154361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8f975b900","size":5,"soft_fail":false,"target_va":5391038032,"va":5378883666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154bb50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12155674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e81a7bb900","size":5,"soft_fail":false,"target_va":5391039312,"va":5378883633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14405818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8bad0db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378882945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141779660
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14440393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8c957dc00","size":5,"soft_fail":false,"target_va":5393323616,"va":5378883218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417835d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14481390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8eef7dc00","size":5,"soft_fail":false,"target_va":5393364432,"va":5378883037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141787d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14499284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e8d43ddd00","size":5,"soft_fail":false,"target_va":5393382688,"va":5378883399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37488491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e86b073c02","size":5,"soft_fail":false,"target_va":5416370864,"va":5378882368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37737826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e862d53f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378883769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4653596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e81c024700","size":5,"soft_fail":false,"target_va":5383536672,"va":5378883071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4656172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378882320,"raw_hex":"e82c0c4700","size":5,"soft_fail":false,"target_va":5383539152,"va":5378882975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":43896,"parent_record_offset":43720,"ancestor_record_offset":43720,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":44052,"parent_record_offset":44020,"ancestor_record_offset":43720,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":44100,"parent_record_offset":44076,"ancestor_record_offset":43720,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":44076,"parent_record_offset":44052,"ancestor_record_offset":43720,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":44020,"parent_record_offset":43984,"ancestor_record_offset":43720,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":43720,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":45520,"parent_record_offset":45480,"ancestor_record_offset":43720,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":43960,"parent_record_offset":43936,"ancestor_record_offset":43720,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":43984,"parent_record_offset":43960,"ancestor_record_offset":43720,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":45480,"parent_record_offset":45456,"ancestor_record_offset":43720,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":44272,"parent_record_offset":44244,"ancestor_record_offset":43720,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":45840,"parent_record_offset":45816,"ancestor_record_offset":43720,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":45816,"parent_record_offset":45792,"ancestor_record_offset":43720,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":45764,"parent_record_offset":45732,"ancestor_record_offset":43720,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45708,"parent_record_offset":45196,"ancestor_record_offset":43720,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45732,"parent_record_offset":45708,"ancestor_record_offset":43720,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45228,"parent_record_offset":45196,"ancestor_record_offset":43720,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45792,"parent_record_offset":45732,"ancestor_record_offset":43720,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":43720,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":43720,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":45456,"parent_record_offset":45432,"ancestor_record_offset":43720,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":517,"record_offset":45908,"parent_record_offset":45884,"ancestor_record_offset":43720,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":45884,"parent_record_offset":45196,"ancestor_record_offset":43720,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":44244,"parent_record_offset":44216,"ancestor_record_offset":43720,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":45196,"parent_record_offset":44244,"ancestor_record_offset":43720,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":44296,"parent_record_offset":44272,"ancestor_record_offset":43720,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":44944,"parent_record_offset":44920,"ancestor_record_offset":43720,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":44972,"parent_record_offset":44944,"ancestor_record_offset":43720,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":44512,"parent_record_offset":44488,"ancestor_record_offset":43720,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":44596,"parent_record_offset":44572,"ancestor_record_offset":43720,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":44744,"parent_record_offset":44716,"ancestor_record_offset":43720,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":44716,"parent_record_offset":44688,"ancestor_record_offset":43720,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":45368,"parent_record_offset":45304,"ancestor_record_offset":43720,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":43720,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":45432,"parent_record_offset":45252,"ancestor_record_offset":43720,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":46004,"parent_record_offset":44216,"ancestor_record_offset":43720,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":44536,"parent_record_offset":44512,"ancestor_record_offset":43720,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":43720,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":44140,"parent_record_offset":43984,"ancestor_record_offset":43720,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":45304,"parent_record_offset":45276,"ancestor_record_offset":43720,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":45340,"parent_record_offset":45304,"ancestor_record_offset":43720,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":45660,"parent_record_offset":45480,"ancestor_record_offset":43720,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":45932,"parent_record_offset":45908,"ancestor_record_offset":43720,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":45956,"parent_record_offset":45932,"ancestor_record_offset":43720,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46036,"parent_record_offset":44180,"ancestor_record_offset":43720,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46096,"parent_record_offset":46072,"ancestor_record_offset":43720,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46708,"parent_record_offset":46684,"ancestor_record_offset":43720,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$24::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46072,"parent_record_offset":46036,"ancestor_record_offset":43720,"inlinee":30201,"source_line":593,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$24::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46684,"parent_record_offset":46036,"ancestor_record_offset":43720,"inlinee":30214,"source_line":596,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":43720,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":43720,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":43720,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":43720,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
//     evidence={"module":517,"record_offset":44180,"parent_record_offset":43896,"ancestor_record_offset":43720,"inlinee":30199,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
//     evidence={"module":517,"record_offset":46240,"parent_record_offset":46036,"ancestor_record_offset":43720,"inlinee":30205,"source_line":86,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":44216,"parent_record_offset":44180,"ancestor_record_offset":43720,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":44920,"parent_record_offset":44896,"ancestor_record_offset":43720,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":44656,"parent_record_offset":44628,"ancestor_record_offset":43720,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":44688,"parent_record_offset":44656,"ancestor_record_offset":43720,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":44628,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":44488,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":45048,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":44572,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":44332,"parent_record_offset":44296,"ancestor_record_offset":43720,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":44896,"parent_record_offset":44788,"ancestor_record_offset":43720,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44836,"parent_record_offset":44812,"ancestor_record_offset":43720,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":45140,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":43720,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44384,"parent_record_offset":44360,"ancestor_record_offset":43720,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44360,"parent_record_offset":44332,"ancestor_record_offset":43720,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44408,"parent_record_offset":44384,"ancestor_record_offset":43720,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44460,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44788,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":45020,"parent_record_offset":44408,"ancestor_record_offset":43720,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":46296,"parent_record_offset":46240,"ancestor_record_offset":43720,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":46364,"parent_record_offset":46240,"ancestor_record_offset":43720,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":517,"record_offset":43936,"parent_record_offset":43896,"ancestor_record_offset":43720,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75F7
// 首条位置: {"module":517,"inlinee":30199}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46036,"parent_record_offset":44180,"ancestor_record_offset":44180,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":44216,"parent_record_offset":44180,"ancestor_record_offset":44180,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":45568,"parent_record_offset":45544,"ancestor_record_offset":44180,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":45616,"parent_record_offset":45592,"ancestor_record_offset":44180,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":45592,"parent_record_offset":45568,"ancestor_record_offset":44180,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":44180,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":45520,"parent_record_offset":45480,"ancestor_record_offset":44180,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":45480,"parent_record_offset":45456,"ancestor_record_offset":44180,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":44272,"parent_record_offset":44244,"ancestor_record_offset":44180,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":45840,"parent_record_offset":45816,"ancestor_record_offset":44180,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":45816,"parent_record_offset":45792,"ancestor_record_offset":44180,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":45764,"parent_record_offset":45732,"ancestor_record_offset":44180,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45708,"parent_record_offset":45196,"ancestor_record_offset":44180,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45732,"parent_record_offset":45708,"ancestor_record_offset":44180,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45228,"parent_record_offset":45196,"ancestor_record_offset":44180,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":45792,"parent_record_offset":45732,"ancestor_record_offset":44180,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":44180,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":44180,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":45456,"parent_record_offset":45432,"ancestor_record_offset":44180,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":517,"record_offset":45908,"parent_record_offset":45884,"ancestor_record_offset":44180,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":45884,"parent_record_offset":45196,"ancestor_record_offset":44180,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":44244,"parent_record_offset":44216,"ancestor_record_offset":44180,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":45196,"parent_record_offset":44244,"ancestor_record_offset":44180,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":44296,"parent_record_offset":44272,"ancestor_record_offset":44180,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":44944,"parent_record_offset":44920,"ancestor_record_offset":44180,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":44972,"parent_record_offset":44944,"ancestor_record_offset":44180,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":44512,"parent_record_offset":44488,"ancestor_record_offset":44180,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":44596,"parent_record_offset":44572,"ancestor_record_offset":44180,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":44744,"parent_record_offset":44716,"ancestor_record_offset":44180,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":44716,"parent_record_offset":44688,"ancestor_record_offset":44180,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":45368,"parent_record_offset":45304,"ancestor_record_offset":44180,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":44180,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":45432,"parent_record_offset":45252,"ancestor_record_offset":44180,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":46004,"parent_record_offset":44216,"ancestor_record_offset":44180,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":44536,"parent_record_offset":44512,"ancestor_record_offset":44180,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":44180,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":45304,"parent_record_offset":45276,"ancestor_record_offset":44180,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":45340,"parent_record_offset":45304,"ancestor_record_offset":44180,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":45660,"parent_record_offset":45480,"ancestor_record_offset":44180,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":45932,"parent_record_offset":45908,"ancestor_record_offset":44180,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":45956,"parent_record_offset":45932,"ancestor_record_offset":44180,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46096,"parent_record_offset":46072,"ancestor_record_offset":44180,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46708,"parent_record_offset":46684,"ancestor_record_offset":44180,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$24::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46072,"parent_record_offset":46036,"ancestor_record_offset":44180,"inlinee":30201,"source_line":593,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$24::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":46684,"parent_record_offset":46036,"ancestor_record_offset":44180,"inlinee":30214,"source_line":596,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":44180,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":44180,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":44180,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":44180,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
//     evidence={"module":517,"record_offset":46240,"parent_record_offset":46036,"ancestor_record_offset":44180,"inlinee":30205,"source_line":86,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":44920,"parent_record_offset":44896,"ancestor_record_offset":44180,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":44656,"parent_record_offset":44628,"ancestor_record_offset":44180,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":44688,"parent_record_offset":44656,"ancestor_record_offset":44180,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":44628,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":44488,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":45048,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":44572,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":44332,"parent_record_offset":44296,"ancestor_record_offset":44180,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":44896,"parent_record_offset":44788,"ancestor_record_offset":44180,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44836,"parent_record_offset":44812,"ancestor_record_offset":44180,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":45140,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":44180,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44384,"parent_record_offset":44360,"ancestor_record_offset":44180,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44360,"parent_record_offset":44332,"ancestor_record_offset":44180,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44408,"parent_record_offset":44384,"ancestor_record_offset":44180,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44460,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":44788,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":45020,"parent_record_offset":44408,"ancestor_record_offset":44180,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":46296,"parent_record_offset":46240,"ancestor_record_offset":44180,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":46364,"parent_record_offset":46240,"ancestor_record_offset":44180,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75FD
// 首条位置: {"module":517,"inlinee":30205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":46296,"parent_record_offset":46240,"ancestor_record_offset":46240,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":46364,"parent_record_offset":46240,"ancestor_record_offset":46240,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":46240,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":46240,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":46240,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":46240,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}
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
// 完整结构及原始类型记录: compute_garden.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>,tuple$<ref$<country_core::asset_libraries::ssbo_library::AssetSsboLibrary> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "tuple$<ref$<country_core::asset_libraries::ssbo_library::AssetSsboLibrary> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>,tuple$<ref$<country_core::asset_libraries::ssbo_library::AssetSsboLibrary> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure$0::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "tuple$<ref$<country_core::asset_libraries::ssbo_library::AssetSsboLibrary> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$5::gpu_world_subset_desc::external::closure_env$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compute_garden.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: compute_garden.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":51512,"parent_record_offset":51476,"ancestor_record_offset":47112,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/gauss_blur_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":51512,"parent_record_offset":51476,"ancestor_record_offset":47584,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_bushes_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":43720,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":43720,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/marching_squares","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":43720,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_small_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":43720,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_bushes_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":44180,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":44180,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/marching_squares","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":44180,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_small_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":44180,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_bushes_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":46240,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":46240,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/marching_squares","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":46240,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_small_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":46240,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [Shader 映射] {"shader_id":"c72bf15bc4cfc4fa8e7fce41","relative_path":"country-core/compiled/gauss_blur_mask.precomp","sha256":"aa16834285ace72b80dc53dd77ac8bd52c513192cde2a17739b96ca23b82e6e3","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu","hint":{"shader_family":"country_core/gauss_blur_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":51512,"parent_record_offset":51476,"ancestor_record_offset":47112,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_mask:main"}
// [Shader 映射] {"shader_id":"c72bf15bc4cfc4fa8e7fce41","relative_path":"country-core/compiled/gauss_blur_mask.precomp","sha256":"aa16834285ace72b80dc53dd77ac8bd52c513192cde2a17739b96ca23b82e6e3","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu::closure$0","hint":{"shader_family":"country_core/gauss_blur_mask","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::gauss_blur_mask::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":51512,"parent_record_offset":51476,"ancestor_record_offset":47584,"inlinee":30221,"source_line":8744,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/gauss_blur_mask:main"}
// [Shader 映射] {"shader_id":"037466a1f9e26746d9da86be","relative_path":"country-core/compiled/garden/spawn_bushes_garden.precomp","sha256":"4ea1c8796965a1d2767f3edf2e5771d59f2230e6d2f2cd4b5e00159af4116bef","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation","hint":{"shader_family":"country_core/garden/spawn_bushes_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":43720,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_bushes_garden:main"}
// [Shader 映射] {"shader_id":"187067b9964fac1b49b4231e","relative_path":"country-core/compiled/garden/spawn_flowers_garden.precomp","sha256":"bbb23bb8d036df2d2426dc9271f4bfa98a80d6d5750ee2efdc1a70625d3941ba","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation","hint":{"shader_family":"country_core/garden/spawn_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":43720,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_flowers_garden:main"}
// [Shader 映射] {"shader_id":"c3ee74eedcba5e0b53216157","relative_path":"country-core/compiled/garden/marching_squares.precomp","sha256":"3d9003b53a095c91ba9fb19a00edac4cdb539e1a1fa83c32fbe7ef6a41bba657","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation","hint":{"shader_family":"country_core/garden/marching_squares","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":43720,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/marching_squares:main"}
// [Shader 映射] {"shader_id":"73a725246b70b1b69c8f7ca8","relative_path":"country-core/compiled/garden/spawn_small_flowers_garden.precomp","sha256":"2628caf7bff57e31c97e28bbf3ea1344b7124f4746d984ccb0662250bee3026f","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation","hint":{"shader_family":"country_core/garden/spawn_small_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":43720,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_small_flowers_garden:main"}
// [Shader 映射] {"shader_id":"037466a1f9e26746d9da86be","relative_path":"country-core/compiled/garden/spawn_bushes_garden.precomp","sha256":"4ea1c8796965a1d2767f3edf2e5771d59f2230e6d2f2cd4b5e00159af4116bef","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0","hint":{"shader_family":"country_core/garden/spawn_bushes_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":44180,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_bushes_garden:main"}
// [Shader 映射] {"shader_id":"187067b9964fac1b49b4231e","relative_path":"country-core/compiled/garden/spawn_flowers_garden.precomp","sha256":"bbb23bb8d036df2d2426dc9271f4bfa98a80d6d5750ee2efdc1a70625d3941ba","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0","hint":{"shader_family":"country_core/garden/spawn_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":44180,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_flowers_garden:main"}
// [Shader 映射] {"shader_id":"c3ee74eedcba5e0b53216157","relative_path":"country-core/compiled/garden/marching_squares.precomp","sha256":"3d9003b53a095c91ba9fb19a00edac4cdb539e1a1fa83c32fbe7ef6a41bba657","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0","hint":{"shader_family":"country_core/garden/marching_squares","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":44180,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/marching_squares:main"}
// [Shader 映射] {"shader_id":"73a725246b70b1b69c8f7ca8","relative_path":"country-core/compiled/garden/spawn_small_flowers_garden.precomp","sha256":"2628caf7bff57e31c97e28bbf3ea1344b7124f4746d984ccb0662250bee3026f","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0","hint":{"shader_family":"country_core/garden/spawn_small_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":44180,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_small_flowers_garden:main"}
// [Shader 映射] {"shader_id":"037466a1f9e26746d9da86be","relative_path":"country-core/compiled/garden/spawn_bushes_garden.precomp","sha256":"4ea1c8796965a1d2767f3edf2e5771d59f2230e6d2f2cd4b5e00159af4116bef","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0","hint":{"shader_family":"country_core/garden/spawn_bushes_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":46240,"inlinee":30207,"source_line":8218,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_bushes_garden:main"}
// [Shader 映射] {"shader_id":"187067b9964fac1b49b4231e","relative_path":"country-core/compiled/garden/spawn_flowers_garden.precomp","sha256":"bbb23bb8d036df2d2426dc9271f4bfa98a80d6d5750ee2efdc1a70625d3941ba","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0","hint":{"shader_family":"country_core/garden/spawn_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46428,"parent_record_offset":46392,"ancestor_record_offset":46240,"inlinee":30209,"source_line":8318,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_flowers_garden:main"}
// [Shader 映射] {"shader_id":"c3ee74eedcba5e0b53216157","relative_path":"country-core/compiled/garden/marching_squares.precomp","sha256":"3d9003b53a095c91ba9fb19a00edac4cdb539e1a1fa83c32fbe7ef6a41bba657","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0","hint":{"shader_family":"country_core/garden/marching_squares","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::marching_squares::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46524,"parent_record_offset":46488,"ancestor_record_offset":46240,"inlinee":30211,"source_line":7981,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/marching_squares:main"}
// [Shader 映射] {"shader_id":"73a725246b70b1b69c8f7ca8","relative_path":"country-core/compiled/garden/spawn_small_flowers_garden.precomp","sha256":"2628caf7bff57e31c97e28bbf3ea1344b7124f4746d984ccb0662250bee3026f","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0","hint":{"shader_family":"country_core/garden/spawn_small_flowers_garden","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_small_flowers_garden::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":46620,"parent_record_offset":46584,"ancestor_record_offset":46240,"inlinee":30213,"source_line":8368,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_small_flowers_garden:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/spawn_bushes_garden.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[56]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":47,"name":"globalParams","storage_class":"Uniform","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[9,45]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":9,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grid_dims","type_id":45,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":156,"name":"path_mask","storage_class":"Uniform","type":{"id":155,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,154]},"decorations":[],"pointee":{"id":154,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[18]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":18,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":178,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":183,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":9,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":188,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":193,"name":"foliage_exclusion_mask.tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":12,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":197,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,194]},"decorations":[],"pointee":{"id":194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":207,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":206,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,204]},"decorations":[],"pointee":{"id":204,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[18,205,205,20,21]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":18,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":205,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":205,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":20,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":21,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":17,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[17]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":246,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,194]},"decorations":[],"pointee":{"id":194,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":248,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":17,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":315,"name":"building_mask.non_blurred_tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":325,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,175]},"decorations":[],"pointee":{"id":175,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":13,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":593,"name":"draw_commands","storage_class":"Uniform","type":{"id":592,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,590]},"decorations":[],"pointee":{"id":590,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[591]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":591,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":619,"name":"bush_buffer","storage_class":"Uniform","type":{"id":618,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,616]},"decorations":[],"pointee":{"id":616,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[617]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":617,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":728,"name":"bush_flowers","storage_class":"Uniform","type":{"id":727,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,725]},"decorations":[],"pointee":{"id":725,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[726]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":726,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":56,"name":"unknown","storage_class":"Input","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[17,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/spawn_flowers_garden.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[21]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"globalParams","storage_class":"Uniform","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[6,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":6,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grid_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":179,"name":"path_mask","storage_class":"Uniform","type":{"id":178,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,177]},"decorations":[],"pointee":{"id":177,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":201,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":206,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":9,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":211,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":216,"name":"foliage_exclusion_mask.tex","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":12,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":220,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":219,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,217]},"decorations":[],"pointee":{"id":217,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":230,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":229,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,227]},"decorations":[],"pointee":{"id":227,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[22,228,228,15,134]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":228,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":228,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":15,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":134,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":17,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[17]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":269,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":219,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,217]},"decorations":[],"pointee":{"id":217,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":271,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":17,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":338,"name":"building_mask.non_blurred_tex","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":348,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":200,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,198]},"decorations":[],"pointee":{"id":198,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":13,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":923,"name":"draw_commands","storage_class":"Uniform","type":{"id":922,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,920]},"decorations":[],"pointee":{"id":920,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[921]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":921,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":945,"name":"flower_buffer","storage_class":"Uniform","type":{"id":944,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,942]},"decorations":[],"pointee":{"id":942,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[943]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":943,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"unknown","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,18]},"decorations":[],"pointee":{"id":18,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[17,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/spawn_small_flowers_garden.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[39]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"globalParams","storage_class":"Uniform","type":{"id":27,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,24]},"decorations":[],"pointee":{"id":24,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[25,26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":25,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grid_dims","type_id":26,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":146,"name":"path_mask","storage_class":"Uniform","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,144]},"decorations":[],"pointee":{"id":144,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[40]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":8,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[8]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":168,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,165]},"decorations":[],"pointee":{"id":165,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":8,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[8]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":173,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,165]},"decorations":[],"pointee":{"id":165,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":178,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,165]},"decorations":[],"pointee":{"id":165,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":12,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":183,"name":"foliage_exclusion_mask.tex","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,165]},"decorations":[],"pointee":{"id":165,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":14,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":187,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":186,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,184]},"decorations":[],"pointee":{"id":184,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":197,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":196,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,194]},"decorations":[],"pointee":{"id":194,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[40,195,195,33,10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":195,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":195,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":33,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":10,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":5,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[5]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":236,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":186,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,184]},"decorations":[],"pointee":{"id":184,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":238,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,165]},"decorations":[],"pointee":{"id":165,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":5,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":305,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,165]},"decorations":[],"pointee":{"id":165,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":15,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":353,"name":"draw_commands","storage_class":"Uniform","type":{"id":352,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,350]},"decorations":[],"pointee":{"id":350,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[351]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":351,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":374,"name":"flower03_buffer","storage_class":"Uniform","type":{"id":373,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,371]},"decorations":[],"pointee":{"id":371,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[372]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":372,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"unknown","storage_class":"Input","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[35,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/marching_squares.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[394,126,143,112]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":104,"name":"globalParams","storage_class":"Uniform","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,101]},"decorations":[],"pointee":{"id":101,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[12,102]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":12,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"contouring_grid_dims","type_id":102,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":219,"name":"garden_mask","storage_class":"Uniform","type":{"id":218,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,217]},"decorations":[],"pointee":{"id":217,"name":"GardenMask_natural","record":{"opcode":"OpTypeStruct","operands":[102]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":102,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":13,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[13]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":243,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":13,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":318,"name":"foliage_exclusion_mask.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":12,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":322,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":321,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,319]},"decorations":[],"pointee":{"id":319,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":335,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":351,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":9,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":361,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":742,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":321,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,319]},"decorations":[],"pointee":{"id":319,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":836,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":835,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,833]},"decorations":[],"pointee":{"id":833,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[102,834,834,11,244]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":102,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":834,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":834,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":11,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":244,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":17,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[17]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":868,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":17,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1084,"name":"draw_commands","storage_class":"Uniform","type":{"id":1083,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1081]},"decorations":[],"pointee":{"id":1081,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1082]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1082,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1124,"name":"instance_buffer","storage_class":"Uniform","type":{"id":1123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1121]},"decorations":[],"pointee":{"id":1121,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1122]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1122,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":112,"name":"unknown","storage_class":"Input","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,5]},"decorations":[],"pointee":{"id":5,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":126,"name":"unknown","storage_class":"Input","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,123]},"decorations":[],"pointee":{"id":123,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":143,"name":"unknown","storage_class":"Input","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,123]},"decorations":[],"pointee":{"id":123,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":394,"name":"unknown","storage_class":"Input","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,123]},"decorations":[],"pointee":{"id":123,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/gauss_blur_mask.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[68]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"globalParams","storage_class":"Uniform","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[41,41,5,8,5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"water_mask_dims","type_id":41,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"out_water_mask_dims","type_id":41,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"is_horizontal","type_id":5,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"sigma","type_id":8,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"output_channel","type_id":5,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":122,"name":"water_mask","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,119]},"decorations":[],"pointee":{"id":119,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":171,"name":"out_water_mask","storage_class":"UniformConstant","type":{"id":170,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,168]},"decorations":[],"pointee":{"id":168,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":68,"name":"unknown","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,65]},"decorations":[],"pointee":{"id":65,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679539,"position_among_mapped_render_targets":19,"mapped_render_target_count":74,"previous_callsite":5679528,"next_callsite":5679550,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass2.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679717,"position_among_mapped_render_targets":24,"mapped_render_target_count":74,"previous_callsite":5679706,"next_callsite":5679728,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compute_garden.rs.enriched.json.gz
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
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
// 信息 ID: 45dfae8da805424e53ba6df9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a9b3 -> 0x9b40c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu::closure$0
// 信息 ID: e20471ebb676fd963e8e7468
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=517, inline_record=47584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
// 信息 ID: 50b2460e33eb7e4b5601f0e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56aa65 -> 0x9b3b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0
// 信息 ID: fe1c3cd0277acabb1acbef7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation; module=517, inline_record=44180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation::closure$0::closure$0
// 信息 ID: fdbc7ad921ca6778f0586975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation; module=517, inline_record=46240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
