// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8041117007c2b5a326ec15ab78ab7cf4a18f12d6f895c7fad08ce82bee5a226b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reset_indirect_args.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 526, 1679
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
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
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":52040,"rva":10436736,"va":5379145856}
// 编译实例 RVAs: 0x9f4080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9842178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8fed169ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5379147757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ab7f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9735854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e852716bff","size":5,"soft_fail":false,"target_va":5369411568,"va":5379147417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098ee20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-415695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e831a8f9ff","size":5,"soft_fail":false,"target_va":5378731552,"va":5379147242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153e5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11838454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8f6a3b400","size":5,"soft_fail":false,"target_va":5390984640,"va":5379146181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153f5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11842616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e838b4b400","size":5,"soft_fail":false,"target_va":5390988784,"va":5379146163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11883956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8b455b500","size":5,"soft_fail":false,"target_va":5391030992,"va":5379147031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154b790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11891327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e87f72b500","size":5,"soft_fail":false,"target_va":5391038352,"va":5379147020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176b0c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14118079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8bf6cd700","size":5,"soft_fail":false,"target_va":5393264832,"va":5379146748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141804de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14747881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8e908e100","size":5,"soft_fail":false,"target_va":5393894880,"va":5379146994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37910553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e819784202","size":5,"soft_fail":false,"target_va":5417057056,"va":5379146498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37474531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8e3d03b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379147064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37475651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e843d53b02","size":5,"soft_fail":false,"target_va":5416622768,"va":5379147112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37429737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8e9213b02","size":5,"soft_fail":false,"target_va":5416576832,"va":5379147090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37428983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8f71e3b02","size":5,"soft_fail":false,"target_va":5416576144,"va":5379147156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4838592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8c0d44900","size":5,"soft_fail":false,"target_va":5383984848,"va":5379146251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4838801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e891d54900","size":5,"soft_fail":false,"target_va":5383985360,"va":5379146554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4379341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e8cdd24200","size":5,"soft_fail":false,"target_va":5383525648,"va":5379146302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4394395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379145856,"raw_hex":"e89b0d4300","size":5,"soft_fail":false,"target_va":5383541008,"va":5379146608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":52224,"parent_record_offset":52040,"ancestor_record_offset":52040,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":56084,"parent_record_offset":52040,"ancestor_record_offset":52040,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":53868,"parent_record_offset":52040,"ancestor_record_offset":52040,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":53544,"parent_record_offset":53520,"ancestor_record_offset":52040,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":53592,"parent_record_offset":53568,"ancestor_record_offset":52040,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":53568,"parent_record_offset":53544,"ancestor_record_offset":52040,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55368,"parent_record_offset":55344,"ancestor_record_offset":52040,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55344,"parent_record_offset":55320,"ancestor_record_offset":52040,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":53520,"parent_record_offset":53496,"ancestor_record_offset":52040,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":53496,"parent_record_offset":53456,"ancestor_record_offset":52040,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55320,"parent_record_offset":55296,"ancestor_record_offset":52040,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":53456,"parent_record_offset":53432,"ancestor_record_offset":52040,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":526,"record_offset":52252,"parent_record_offset":52224,"ancestor_record_offset":52040,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":53816,"parent_record_offset":53792,"ancestor_record_offset":52040,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":53792,"parent_record_offset":53768,"ancestor_record_offset":52040,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":526,"record_offset":53740,"parent_record_offset":53708,"ancestor_record_offset":52040,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":53684,"parent_record_offset":53176,"ancestor_record_offset":52040,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":53708,"parent_record_offset":53684,"ancestor_record_offset":52040,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":53204,"parent_record_offset":53176,"ancestor_record_offset":52040,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":526,"record_offset":53768,"parent_record_offset":53708,"ancestor_record_offset":52040,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":53252,"parent_record_offset":53228,"ancestor_record_offset":52040,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":53228,"parent_record_offset":53204,"ancestor_record_offset":52040,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":526,"record_offset":53432,"parent_record_offset":53408,"ancestor_record_offset":52040,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":526,"record_offset":53176,"parent_record_offset":52224,"ancestor_record_offset":52040,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":526,"record_offset":52276,"parent_record_offset":52252,"ancestor_record_offset":52040,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":526,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":52040,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":526,"record_offset":52952,"parent_record_offset":52924,"ancestor_record_offset":52040,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":526,"record_offset":54332,"parent_record_offset":54308,"ancestor_record_offset":52040,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":52492,"parent_record_offset":52468,"ancestor_record_offset":52040,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":526,"record_offset":52576,"parent_record_offset":52552,"ancestor_record_offset":52040,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":526,"record_offset":54728,"parent_record_offset":53992,"ancestor_record_offset":52040,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":52724,"parent_record_offset":52696,"ancestor_record_offset":52040,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":526,"record_offset":52696,"parent_record_offset":52668,"ancestor_record_offset":52040,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":526,"record_offset":53344,"parent_record_offset":53280,"ancestor_record_offset":52040,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":53368,"parent_record_offset":53344,"ancestor_record_offset":52040,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":53408,"parent_record_offset":53228,"ancestor_record_offset":52040,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":52516,"parent_record_offset":52492,"ancestor_record_offset":52040,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":54512,"parent_record_offset":54364,"ancestor_record_offset":52040,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":526,"record_offset":52840,"parent_record_offset":52816,"ancestor_record_offset":52040,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":54440,"parent_record_offset":54408,"ancestor_record_offset":52040,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":54408,"parent_record_offset":54364,"ancestor_record_offset":52040,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":53280,"parent_record_offset":53252,"ancestor_record_offset":52040,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":53316,"parent_record_offset":53280,"ancestor_record_offset":52040,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":526,"record_offset":53636,"parent_record_offset":53456,"ancestor_record_offset":52040,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54876,"parent_record_offset":54812,"ancestor_record_offset":52040,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54212,"parent_record_offset":54176,"ancestor_record_offset":52040,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54900,"parent_record_offset":54876,"ancestor_record_offset":52040,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54236,"parent_record_offset":54212,"ancestor_record_offset":52040,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":53932,"parent_record_offset":53908,"ancestor_record_offset":52040,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":56020,"parent_record_offset":55996,"ancestor_record_offset":52040,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$21::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":53908,"parent_record_offset":53868,"ancestor_record_offset":52040,"inlinee":30229,"source_line":593,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$21::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":55996,"parent_record_offset":53868,"ancestor_record_offset":52040,"inlinee":30234,"source_line":596,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs
//     evidence={"module":526,"record_offset":53992,"parent_record_offset":53868,"ancestor_record_offset":52040,"inlinee":31067,"source_line":19,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":526,"record_offset":52900,"parent_record_offset":52876,"ancestor_record_offset":52040,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":52636,"parent_record_offset":52608,"ancestor_record_offset":52040,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":52668,"parent_record_offset":52636,"ancestor_record_offset":52040,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":526,"record_offset":52608,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":52468,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":53028,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":526,"record_offset":52552,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":52312,"parent_record_offset":52276,"ancestor_record_offset":52040,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":526,"record_offset":52876,"parent_record_offset":52768,"ancestor_record_offset":52040,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":52040,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":53120,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52792,"parent_record_offset":52768,"ancestor_record_offset":52040,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52364,"parent_record_offset":52340,"ancestor_record_offset":52040,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52340,"parent_record_offset":52312,"ancestor_record_offset":52040,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52388,"parent_record_offset":52364,"ancestor_record_offset":52040,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52440,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":526,"record_offset":52768,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":526,"record_offset":53000,"parent_record_offset":52388,"ancestor_record_offset":52040,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":526,"record_offset":54308,"parent_record_offset":53992,"ancestor_record_offset":52040,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":526,"record_offset":54784,"parent_record_offset":54756,"ancestor_record_offset":52040,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":526,"record_offset":54176,"parent_record_offset":54124,"ancestor_record_offset":52040,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":526,"record_offset":54812,"parent_record_offset":54784,"ancestor_record_offset":52040,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":54268,"parent_record_offset":54176,"ancestor_record_offset":52040,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":54848,"parent_record_offset":54812,"ancestor_record_offset":52040,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::dynamic_constants
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":526,"record_offset":54096,"parent_record_offset":53992,"ancestor_record_offset":52040,"inlinee":16914,"source_line":919,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/dynamic_constants.rs
//     evidence={"module":526,"record_offset":54364,"parent_record_offset":53992,"ancestor_record_offset":52040,"inlinee":13992,"source_line":113,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::NodeBuilder::access
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
//     evidence={"module":526,"record_offset":55012,"parent_record_offset":54980,"ancestor_record_offset":52040,"inlinee":4477,"source_line":89,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::NodeBuilder::write
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
//     evidence={"module":526,"record_offset":54948,"parent_record_offset":53992,"ancestor_record_offset":52040,"inlinee":4496,"source_line":107,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::NodeBuilder::write_impl
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
//     evidence={"module":526,"record_offset":54980,"parent_record_offset":54948,"ancestor_record_offset":52040,"inlinee":4497,"source_line":64,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::impl$14::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":526,"record_offset":55100,"parent_record_offset":55012,"ancestor_record_offset":52040,"inlinee":4479,"source_line":972,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::impl$15::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":526,"record_offset":55296,"parent_record_offset":55012,"ancestor_record_offset":52040,"inlinee":4481,"source_line":979,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x795B
// 首条位置: {"module":526,"inlinee":31067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":526,"record_offset":54728,"parent_record_offset":53992,"ancestor_record_offset":53992,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":526,"record_offset":54308,"parent_record_offset":53992,"ancestor_record_offset":53992,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::dynamic_constants
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":526,"record_offset":54096,"parent_record_offset":53992,"ancestor_record_offset":53992,"inlinee":16914,"source_line":919,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/dynamic_constants.rs
//     evidence={"module":526,"record_offset":54364,"parent_record_offset":53992,"ancestor_record_offset":53992,"inlinee":13992,"source_line":113,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::NodeBuilder::write
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
//     evidence={"module":526,"record_offset":54948,"parent_record_offset":53992,"ancestor_record_offset":53992,"inlinee":4496,"source_line":107,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":55196,"parent_record_offset":55172,"ancestor_record_offset":53992,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":55244,"parent_record_offset":55220,"ancestor_record_offset":53992,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":526,"record_offset":55220,"parent_record_offset":55196,"ancestor_record_offset":53992,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55368,"parent_record_offset":55344,"ancestor_record_offset":53992,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55344,"parent_record_offset":55320,"ancestor_record_offset":53992,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55172,"parent_record_offset":55148,"ancestor_record_offset":53992,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55148,"parent_record_offset":55124,"ancestor_record_offset":53992,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55320,"parent_record_offset":55296,"ancestor_record_offset":53992,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":526,"record_offset":55124,"parent_record_offset":55100,"ancestor_record_offset":53992,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":526,"record_offset":54332,"parent_record_offset":54308,"ancestor_record_offset":53992,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":54124,"parent_record_offset":54096,"ancestor_record_offset":53992,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":54476,"parent_record_offset":54440,"ancestor_record_offset":53992,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":54512,"parent_record_offset":54364,"ancestor_record_offset":53992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":54440,"parent_record_offset":54408,"ancestor_record_offset":53992,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":526,"record_offset":54408,"parent_record_offset":54364,"ancestor_record_offset":53992,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54876,"parent_record_offset":54812,"ancestor_record_offset":53992,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54212,"parent_record_offset":54176,"ancestor_record_offset":53992,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54900,"parent_record_offset":54876,"ancestor_record_offset":53992,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":526,"record_offset":54236,"parent_record_offset":54212,"ancestor_record_offset":53992,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":526,"record_offset":54784,"parent_record_offset":54756,"ancestor_record_offset":53992,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":526,"record_offset":54176,"parent_record_offset":54124,"ancestor_record_offset":53992,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":526,"record_offset":54812,"parent_record_offset":54784,"ancestor_record_offset":53992,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":54268,"parent_record_offset":54176,"ancestor_record_offset":53992,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":526,"record_offset":54848,"parent_record_offset":54812,"ancestor_record_offset":53992,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::NodeBuilder::access
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
//     evidence={"module":526,"record_offset":55012,"parent_record_offset":54980,"ancestor_record_offset":53992,"inlinee":4477,"source_line":89,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::NodeBuilder::write_impl
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
//     evidence={"module":526,"record_offset":54980,"parent_record_offset":54948,"ancestor_record_offset":53992,"inlinee":4497,"source_line":64,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::impl$14::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":526,"record_offset":55100,"parent_record_offset":55012,"ancestor_record_offset":53992,"inlinee":4479,"source_line":972,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::impl$15::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":526,"record_offset":55296,"parent_record_offset":55012,"ancestor_record_offset":53992,"inlinee":4481,"source_line":979,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1430F
// 首条位置: {"module":1679,"inlinee":82703}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1679,"record_offset":15064,"parent_record_offset":14312,"ancestor_record_offset":14312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::render_api::RenderApi::copy_buffer
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/render_api.rs
//     evidence={"module":1679,"record_offset":14384,"parent_record_offset":14312,"ancestor_record_offset":14312,"inlinee":82704,"source_line":501,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1679,"record_offset":14692,"parent_record_offset":14384,"ancestor_record_offset":14312,"inlinee":51641,"source_line":1466,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1679,"record_offset":14584,"parent_record_offset":14556,"ancestor_record_offset":14312,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1679,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14312,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1679,"record_offset":14612,"parent_record_offset":14556,"ancestor_record_offset":14312,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1679,"record_offset":14636,"parent_record_offset":14612,"ancestor_record_offset":14312,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1679,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":14312,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1679,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":14312,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::HandleBorrow::hot
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1679,"record_offset":14460,"parent_record_offset":14436,"ancestor_record_offset":14312,"inlinee":10377,"source_line":999,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1679,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":14312,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$28::hot
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1679,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":14312,"inlinee":33563,"source_line":934,"archive":"_pdb/C/modules/01679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$34::hot
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1679,"record_offset":14436,"parent_record_offset":14384,"ancestor_record_offset":14312,"inlinee":37432,"source_line":1033,"archive":"_pdb/C/modules/01679.jsonl.gz"}
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
// 完整结构及原始类型记录: reset_indirect_args.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reset_indirect_args.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: reset_indirect_args.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::node_builder::NodeBuilder::write","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":54948,"parent_record_offset":53992,"ancestor_record_offset":52040,"inlinee":4496,"source_line":107,"archive":"_pdb/C/modules/00526.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_access_api","symbol":"rhapsody::rg::node_builder::NodeBuilder::access","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":55012,"parent_record_offset":54980,"ancestor_record_offset":52040,"inlinee":4477,"source_line":89,"archive":"_pdb/C/modules/00526.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::node_builder::NodeBuilder::write","relationship":"pdb_inline_direct","evidence":{"module":526,"record_offset":54948,"parent_record_offset":53992,"ancestor_record_offset":53992,"inlinee":4496,"source_line":107,"archive":"_pdb/C/modules/00526.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_access_api","symbol":"rhapsody::rg::node_builder::NodeBuilder::access","relationship":"pdb_inline_transitive","evidence":{"module":526,"record_offset":55012,"parent_record_offset":54980,"ancestor_record_offset":53992,"inlinee":4477,"source_line":89,"archive":"_pdb/C/modules/00526.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3773669,"position_among_mapped_render_targets":1,"mapped_render_target_count":24,"previous_callsite":null,"next_callsite":3774563,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/painterly/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reset_indirect_args.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// 信息 ID: 34fb86d958943fe0df49e1ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3994e5 -> 0x9f4080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// 信息 ID: 04e7ad6ae4393287ad00662a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=526, inline_record=53992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure$0
// 信息 ID: d6c8ae5602183c4be73f87dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0 <- core::ptr::drop_in_place<bumpalo::impl$9::alloc::closure_env$0<1,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0 <- rhapsody::rg::recorder::impl$8::new_in::render_impl<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0 <- rhapsody::rg::recorder::impl$8::new_in::drop_impl<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$8::new_in::render_impl<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0>; module=1679, inline_record=14312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
