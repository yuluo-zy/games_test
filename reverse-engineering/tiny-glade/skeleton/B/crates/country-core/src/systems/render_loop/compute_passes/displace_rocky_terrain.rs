// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 739184f838a1038f42daf090ec4148b0ac75cdfe9d244cd359f928f341f4ebd0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: displace_rocky_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 545
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":3664,"rva":10743424,"va":5379452544}
// 编译实例 RVAs: 0xa3ee80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9836499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e82de869ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379452606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4371711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8014bbdff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379453194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4371742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8e24abdff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379453241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1737663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8417ce5ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379453802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1285474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e89e62ecff","size":5,"soft_fail":false,"target_va":5378167392,"va":5379452861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-400876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e814e2f9ff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379453623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1716703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8df311a00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379453644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12337992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e84843bc00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379452755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141604c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12344435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8735cbc00","size":5,"soft_fail":false,"target_va":5391797248,"va":5379452808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141605c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12348636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8dc6cbc00","size":5,"soft_fail":false,"target_va":5391801424,"va":5379452783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32394277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8254cee01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379452886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32087265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e8e19ce901","size":5,"soft_fail":false,"target_va":5411540963,"va":5379453693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32088100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379452544,"raw_hex":"e824a0e901","size":5,"soft_fail":false,"target_va":5411541776,"va":5379453671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":545,"record_offset":5640,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":27051,"source_line":460,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":545,"record_offset":4976,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":545,"record_offset":4832,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":545,"record_offset":4700,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":545,"record_offset":3864,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":3664,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":3664,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":3664,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":3664,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":5564,"parent_record_offset":5508,"ancestor_record_offset":3664,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3664,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":5176,"parent_record_offset":5148,"ancestor_record_offset":3664,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":3664,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":3664,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":3664,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3664,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":5236,"parent_record_offset":5204,"ancestor_record_offset":3664,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":3664,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":3664,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":3664,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":3664,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":5028,"parent_record_offset":4976,"ancestor_record_offset":3664,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":3664,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":3664,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":4268,"parent_record_offset":4228,"ancestor_record_offset":3664,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":5148,"parent_record_offset":5112,"ancestor_record_offset":3664,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":3664,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":545,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":3664,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":545,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":3664,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":545,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":3664,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":545,"record_offset":4484,"parent_record_offset":3956,"ancestor_record_offset":3664,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":545,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3664,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":545,"record_offset":3984,"parent_record_offset":3956,"ancestor_record_offset":3664,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":545,"record_offset":4568,"parent_record_offset":4508,"ancestor_record_offset":3664,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":545,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3664,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":545,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3664,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":545,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":3664,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":545,"record_offset":3920,"parent_record_offset":3892,"ancestor_record_offset":3664,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":545,"record_offset":3892,"parent_record_offset":3864,"ancestor_record_offset":3664,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":545,"record_offset":3956,"parent_record_offset":3892,"ancestor_record_offset":3664,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":545,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":3664,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":545,"record_offset":4120,"parent_record_offset":4060,"ancestor_record_offset":3664,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":545,"record_offset":4140,"parent_record_offset":4120,"ancestor_record_offset":3664,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":545,"record_offset":4180,"parent_record_offset":4008,"ancestor_record_offset":3664,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":545,"record_offset":4668,"parent_record_offset":3864,"ancestor_record_offset":3664,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":545,"record_offset":5364,"parent_record_offset":5112,"ancestor_record_offset":3664,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":5436,"parent_record_offset":4976,"ancestor_record_offset":3664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":3664,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":3664,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":4092,"parent_record_offset":4060,"ancestor_record_offset":3664,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":4408,"parent_record_offset":4228,"ancestor_record_offset":3664,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":545,"record_offset":5668,"parent_record_offset":5640,"ancestor_record_offset":3664,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":545,"record_offset":4732,"parent_record_offset":4700,"ancestor_record_offset":3664,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00545.jsonl.gz"}
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
// 完整结构及原始类型记录: displace_rocky_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: displace_rocky_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: displace_rocky_terrain.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":545,"record_offset":4700,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00545.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":545,"record_offset":4732,"parent_record_offset":4700,"ancestor_record_offset":3664,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00545.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":545,"record_offset":4832,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00545.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161740,"position_among_mapped_render_targets":9,"mapped_render_target_count":59,"previous_callsite":5161729,"next_callsite":5161751,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: displace_rocky_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
// 信息 ID: b4f07c742b1d8c36d0639b54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec30c -> 0xa3ee80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
