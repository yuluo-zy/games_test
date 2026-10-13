// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6c1ca637d0e0dff7ca11699701262f9d6c021187081a426ffc36a952ed97f3a4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reset_indirect_args.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 503, 1655
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
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":69128,"rva":9480304,"va":5378189424}
// 编译实例 RVAs: 0x90a870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=4
//     target_aliases=532 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9236774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8da0e73ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5378190497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8573388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8342e7dff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378189495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8034216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8586885ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5378192339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-580546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e83e24f7ff","size":5,"soft_fail":false,"target_va":5377611920,"va":5378192461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dca90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t859596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8cc1d0d00","size":5,"soft_fail":false,"target_va":5379050128,"va":5378190527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dcfd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t860509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e85d210d00","size":5,"soft_fail":false,"target_va":5379051472,"va":5378190958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2924930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e882a12c00","size":5,"soft_fail":false,"target_va":5381114688,"va":5378189753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2979118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e82e752d00","size":5,"soft_fail":false,"target_va":5381170352,"va":5378191229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412cb1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10225710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e82e089c00","size":5,"soft_fail":false,"target_va":5388415456,"va":5378189741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412d2170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10254315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8eb779c00","size":5,"soft_fail":false,"target_va":5388444016,"va":5378189696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412d5ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10268988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e83cb19c00","size":5,"soft_fail":false,"target_va":5388458704,"va":5378189711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412d6130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10270605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e88db79c00","size":5,"soft_fail":false,"target_va":5388460336,"va":5378189726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: reset_indirect_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33657030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8c6900102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378190133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33350652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8fce3fc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378191119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33352237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e82deafc01","size":5,"soft_fail":false,"target_va":5411543264,"va":5378191022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33301363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e87323fc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5378191000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33303693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e88d2cfc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5378191102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3633724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e83c723700","size":5,"soft_fail":false,"target_va":5381823600,"va":5378189871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3633549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e88d713700","size":5,"soft_fail":false,"target_va":5381824016,"va":5378190462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3112142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378189424,"raw_hex":"e8ce7c2f00","size":5,"soft_fail":false,"target_va":5381302080,"va":5378189933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":69340,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":503,"record_offset":72668,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":73128,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":73184,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::pass_resource_access::ssbo_to_rg_handle
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":503,"record_offset":70892,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":28405,"source_line":203,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":503,"record_offset":71064,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":503,"record_offset":71220,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":503,"record_offset":71276,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":26614,"source_line":150,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":503,"record_offset":72888,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":503,"record_offset":71004,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":9697,"source_line":24,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":69764,"parent_record_offset":69740,"ancestor_record_offset":69128,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":69812,"parent_record_offset":69788,"ancestor_record_offset":69128,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":69788,"parent_record_offset":69764,"ancestor_record_offset":69128,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":69740,"parent_record_offset":69716,"ancestor_record_offset":69128,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":69716,"parent_record_offset":69676,"ancestor_record_offset":69128,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":69676,"parent_record_offset":69652,"ancestor_record_offset":69128,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":70064,"parent_record_offset":70040,"ancestor_record_offset":69128,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":70040,"parent_record_offset":70016,"ancestor_record_offset":69128,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":69988,"parent_record_offset":69956,"ancestor_record_offset":69128,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":69932,"parent_record_offset":69404,"ancestor_record_offset":69128,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":69956,"parent_record_offset":69932,"ancestor_record_offset":69128,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":69432,"parent_record_offset":69404,"ancestor_record_offset":69128,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":70016,"parent_record_offset":69956,"ancestor_record_offset":69128,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":69480,"parent_record_offset":69456,"ancestor_record_offset":69128,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":69456,"parent_record_offset":69432,"ancestor_record_offset":69128,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":69652,"parent_record_offset":69628,"ancestor_record_offset":69128,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":69368,"parent_record_offset":69340,"ancestor_record_offset":69128,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":69404,"parent_record_offset":69340,"ancestor_record_offset":69128,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":503,"record_offset":71244,"parent_record_offset":71220,"ancestor_record_offset":69128,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":71032,"parent_record_offset":71004,"ancestor_record_offset":69128,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":71360,"parent_record_offset":71324,"ancestor_record_offset":69128,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":503,"record_offset":71468,"parent_record_offset":71440,"ancestor_record_offset":69128,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":503,"record_offset":69568,"parent_record_offset":69508,"ancestor_record_offset":69128,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":69588,"parent_record_offset":69568,"ancestor_record_offset":69128,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":69628,"parent_record_offset":69456,"ancestor_record_offset":69128,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":70144,"parent_record_offset":70116,"ancestor_record_offset":69128,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":71392,"parent_record_offset":71276,"ancestor_record_offset":69128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":71492,"parent_record_offset":71468,"ancestor_record_offset":69128,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":71324,"parent_record_offset":71276,"ancestor_record_offset":69128,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":69508,"parent_record_offset":69480,"ancestor_record_offset":69128,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":69880,"parent_record_offset":69856,"ancestor_record_offset":69128,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":69540,"parent_record_offset":69508,"ancestor_record_offset":69128,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":69856,"parent_record_offset":69676,"ancestor_record_offset":69128,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":72816,"parent_record_offset":72752,"ancestor_record_offset":69128,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":71128,"parent_record_offset":71092,"ancestor_record_offset":69128,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":72840,"parent_record_offset":72816,"ancestor_record_offset":69128,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":71152,"parent_record_offset":71128,"ancestor_record_offset":69128,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":503,"record_offset":72724,"parent_record_offset":72696,"ancestor_record_offset":69128,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":503,"record_offset":71092,"parent_record_offset":71064,"ancestor_record_offset":69128,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":503,"record_offset":72752,"parent_record_offset":72724,"ancestor_record_offset":69128,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":503,"record_offset":71184,"parent_record_offset":71092,"ancestor_record_offset":69128,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":503,"record_offset":72788,"parent_record_offset":72752,"ancestor_record_offset":69128,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":503,"record_offset":71440,"parent_record_offset":71416,"ancestor_record_offset":69128,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":503,"record_offset":72916,"parent_record_offset":72888,"ancestor_record_offset":69128,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12580
// 首条位置: {"module":1655,"inlinee":75136}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":45420,"parent_record_offset":44852,"ancestor_record_offset":44852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::track_unwrap
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":45124,"parent_record_offset":44852,"ancestor_record_offset":44852,"inlinee":74648,"source_line":424,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Ref::raw
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1655,"record_offset":45012,"parent_record_offset":44852,"ancestor_record_offset":44852,"inlinee":28197,"source_line":344,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1655,"record_offset":45268,"parent_record_offset":45240,"ancestor_record_offset":44852,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":45216,"parent_record_offset":45124,"ancestor_record_offset":44852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":45292,"parent_record_offset":45268,"ancestor_record_offset":44852,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":45184,"parent_record_offset":45160,"ancestor_record_offset":44852,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$22::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":45160,"parent_record_offset":45124,"ancestor_record_offset":44852,"inlinee":74584,"source_line":397,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":45240,"parent_record_offset":45216,"ancestor_record_offset":44852,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/01655.jsonl.gz"}
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
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":72888,"parent_record_offset":69128,"ancestor_record_offset":69128,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1450749,"position_among_mapped_render_targets":1,"mapped_render_target_count":25,"previous_callsite":null,"next_callsite":1451502,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/painterly/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reset_indirect_args.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
// 信息 ID: 85467d8e73aee6ae3ba908ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1622fd -> 0x90a870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0
// 信息 ID: 679cd97b892fcc8d5b30ae1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0 <- alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=44852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
