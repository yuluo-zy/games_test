// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c1171340e2f53b86642c1e3fe2225a85041b60941cdae74a6758210afaa3f2d9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: water_planes_draw.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 532
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
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":59248,"rva":10329024,"va":5379038144}
// 编译实例 RVAs: 0x9d9bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9422175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8a13a70ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379038282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1338415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8d193ebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379039722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-858306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e83ee7f2ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5379038925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-856384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8c0eef2ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5379038603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-453207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8a915f9ff","size":5,"soft_fail":false,"target_va":5378586208,"va":5379039410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416289d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12904690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8f2e8c400","size":5,"soft_fail":false,"target_va":5391944144,"va":5379039449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162a190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12911423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e83f03c500","size":5,"soft_fail":false,"target_va":5391950224,"va":5379038796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162d790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12925069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e88d38c500","size":5,"soft_fail":false,"target_va":5391964048,"va":5379038974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32808943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8ef9ff401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379038220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32502129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e871f1ef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379039642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-858464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8a0e6f2ff","size":5,"soft_fail":false,"target_va":5378179776,"va":5379038235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-864823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038144,"raw_hex":"e8c9cdf2ff","size":5,"soft_fail":false,"target_va":5378174784,"va":5379039602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":61648,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62028,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":532,"record_offset":60304,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":7619,"source_line":274,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::is_halloween2024
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":532,"record_offset":61204,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":27361,"source_line":219,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":61524,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":61904,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":30287,"source_line":30,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":59460,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":61440,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":59920,"parent_record_offset":59896,"ancestor_record_offset":59248,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":59968,"parent_record_offset":59944,"ancestor_record_offset":59248,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":59944,"parent_record_offset":59920,"ancestor_record_offset":59248,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":59896,"parent_record_offset":59872,"ancestor_record_offset":59248,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":59872,"parent_record_offset":59832,"ancestor_record_offset":59248,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":59832,"parent_record_offset":59808,"ancestor_record_offset":59248,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":60220,"parent_record_offset":60196,"ancestor_record_offset":59248,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":60196,"parent_record_offset":60172,"ancestor_record_offset":59248,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":60144,"parent_record_offset":60112,"ancestor_record_offset":59248,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":60088,"parent_record_offset":59552,"ancestor_record_offset":59248,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":60112,"parent_record_offset":60088,"ancestor_record_offset":59248,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":59580,"parent_record_offset":59552,"ancestor_record_offset":59248,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":60172,"parent_record_offset":60112,"ancestor_record_offset":59248,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":59628,"parent_record_offset":59604,"ancestor_record_offset":59248,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":59604,"parent_record_offset":59580,"ancestor_record_offset":59248,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":59808,"parent_record_offset":59784,"ancestor_record_offset":59248,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":59516,"parent_record_offset":59488,"ancestor_record_offset":59248,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":59488,"parent_record_offset":59460,"ancestor_record_offset":59248,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":59552,"parent_record_offset":59488,"ancestor_record_offset":59248,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":532,"record_offset":59720,"parent_record_offset":59656,"ancestor_record_offset":59248,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":62072,"parent_record_offset":62028,"ancestor_record_offset":59248,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":59744,"parent_record_offset":59720,"ancestor_record_offset":59248,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":59784,"parent_record_offset":59604,"ancestor_record_offset":59248,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":60272,"parent_record_offset":59460,"ancestor_record_offset":59248,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":60424,"parent_record_offset":60396,"ancestor_record_offset":59248,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":532,"record_offset":61400,"parent_record_offset":61376,"ancestor_record_offset":59248,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":532,"record_offset":61288,"parent_record_offset":61260,"ancestor_record_offset":59248,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":59656,"parent_record_offset":59628,"ancestor_record_offset":59248,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":61352,"parent_record_offset":61204,"ancestor_record_offset":59248,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":60036,"parent_record_offset":60012,"ancestor_record_offset":59248,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":59692,"parent_record_offset":59656,"ancestor_record_offset":59248,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":60012,"parent_record_offset":59832,"ancestor_record_offset":59248,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":532,"record_offset":61376,"parent_record_offset":61352,"ancestor_record_offset":59248,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":532,"record_offset":61232,"parent_record_offset":61204,"ancestor_record_offset":59248,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":532,"record_offset":61260,"parent_record_offset":61232,"ancestor_record_offset":59248,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":61556,"parent_record_offset":61524,"ancestor_record_offset":59248,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":532,"record_offset":60332,"parent_record_offset":60304,"ancestor_record_offset":59248,"inlinee":26885,"source_line":118,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$3::is_halloween2024::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":532,"record_offset":61320,"parent_record_offset":61232,"ancestor_record_offset":59248,"inlinee":27363,"source_line":222,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":532,"record_offset":61488,"parent_record_offset":61464,"ancestor_record_offset":59248,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":61464,"parent_record_offset":61440,"ancestor_record_offset":59248,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":62208,"rva":10331520,"va":5379040640}
// 编译实例 RVAs: 0x9da580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1339693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e8d38eebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379041000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-860294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e87adff2ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5379040913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-858531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e85de6f2ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5379040750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162b390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12914137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e8d90dc500","size":5,"soft_fail":false,"target_va":5391954832,"va":5379040690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12941619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e83379c500","size":5,"soft_fail":false,"target_va":5391982480,"va":5379040856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32806389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e8f595f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379040774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-863537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379040640,"raw_hex":"e8cfd2f2ff","size":5,"soft_fail":false,"target_va":5378177424,"va":5379040956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62572,"parent_record_offset":62208,"ancestor_record_offset":62208,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62524,"parent_record_offset":62208,"ancestor_record_offset":62208,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":62396,"parent_record_offset":62208,"ancestor_record_offset":62208,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62424,"parent_record_offset":62396,"ancestor_record_offset":62208,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":62604,"rva":10332000,"va":5379041120}
// 编译实例 RVAs: 0x9da760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1340173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e8f38cebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379041480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-860774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e89addf2ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5379041393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-859011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e87de4f2ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5379041230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162fe90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12932857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e8f956c500","size":5,"soft_fail":false,"target_va":5391974032,"va":5379041170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12941139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e85377c500","size":5,"soft_fail":false,"target_va":5391982480,"va":5379041336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32805909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e81594f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379041254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-864017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041120,"raw_hex":"e8efd0f2ff","size":5,"soft_fail":false,"target_va":5378177424,"va":5379041436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62976,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62928,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":62800,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":62828,"parent_record_offset":62800,"ancestor_record_offset":62604,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":63008,"rva":10332480,"va":5379041600}
// 编译实例 RVAs: 0x9da940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1340572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041600,"raw_hex":"e8648bebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379041879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-859490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041600,"raw_hex":"e89ee2f2ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5379041709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162bf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12916250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041600,"raw_hex":"e81a16c500","size":5,"soft_fail":false,"target_va":5391957904,"va":5379041649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12940659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041600,"raw_hex":"e87375c500","size":5,"soft_fail":false,"target_va":5391982480,"va":5379041816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32805429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041600,"raw_hex":"e83592f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379041734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-864419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041600,"raw_hex":"e85dcff2ff","size":5,"soft_fail":false,"target_va":5378177424,"va":5379041838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":63316,"parent_record_offset":63008,"ancestor_record_offset":63008,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":63188,"parent_record_offset":63008,"ancestor_record_offset":63008,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":63216,"parent_record_offset":63188,"ancestor_record_offset":63008,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
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
// 完整结构及原始类型记录: water_planes_draw.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: water_planes_draw.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: water_planes_draw.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":61524,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":532,"record_offset":61556,"parent_record_offset":61524,"ancestor_record_offset":59248,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":61648,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_lighting_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":61904,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":30287,"source_line":30,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":62028,"parent_record_offset":59248,"ancestor_record_offset":59248,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":62396,"parent_record_offset":62208,"ancestor_record_offset":62208,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":532,"record_offset":62424,"parent_record_offset":62396,"ancestor_record_offset":62208,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":62524,"parent_record_offset":62208,"ancestor_record_offset":62208,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":62800,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":532,"record_offset":62828,"parent_record_offset":62800,"ancestor_record_offset":62604,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":62928,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":63188,"parent_record_offset":63008,"ancestor_record_offset":63008,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":532,"record_offset":63216,"parent_record_offset":63188,"ancestor_record_offset":63008,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: water_planes_draw.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw
// 信息 ID: aa8711718acf516051ded012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw
// 信息 ID: 952cdf0658bb9b5bd3101a4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw
// 信息 ID: 84cc81130d2b4d067bc2f7fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
// 信息 ID: 05c8550ffd25d397bf57f7a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
