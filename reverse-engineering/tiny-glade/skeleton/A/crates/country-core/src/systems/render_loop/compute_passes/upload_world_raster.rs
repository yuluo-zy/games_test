// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 963b1441a3f09e87f1590838edf123ee25384cc02ffb31caba8e68eac780643b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: upload_world_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 136
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":136,"record_offset":1040,"rva":2514704,"va":5371223824}
// 编译实例 RVAs: 0x265f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a7680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-781015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e82915f4ff","size":5,"soft_fail":false,"target_va":5370443392,"va":5371224402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a8250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-778017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e8df20f4ff","size":5,"soft_fail":false,"target_va":5370446416,"va":5371224428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a85b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-777386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e85623f4ff","size":5,"soft_fail":false,"target_va":5370447280,"va":5371224661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403389f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t862148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e8c4270d00","size":5,"soft_fail":false,"target_va":5372086768,"va":5371224615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1226205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e8ddb51200","size":5,"soft_fail":false,"target_va":5372451200,"va":5371224990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1524997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e805451700","size":5,"soft_fail":false,"target_va":5372749712,"va":5371224710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1925687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e837621d00","size":5,"soft_fail":false,"target_va":5373149840,"va":5371224148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140661c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4176924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e81cbc3f00","size":5,"soft_fail":false,"target_va":5375401104,"va":5371224175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32368722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e852e8ed01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371224633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6305519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e8ef366000","size":5,"soft_fail":false,"target_va":5377529888,"va":5371224364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31754104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e87887e401","size":5,"soft_fail":false,"target_va":5402978880,"va":5371224771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31754069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e85587e401","size":5,"soft_fail":false,"target_va":5402978944,"va":5371224870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2988931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e8839b2d00","size":5,"soft_fail":false,"target_va":5374213616,"va":5371224680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5678671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371223824,"raw_hex":"e84fa65600","size":5,"soft_fail":false,"target_va":5376903248,"va":5371224572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":136,"record_offset":3192,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":136,"record_offset":3108,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":136,"record_offset":3808,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":3404,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":136,"record_offset":3360,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::needs_texture_upload
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":136,"record_offset":3080,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":10151,"source_line":484,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":136,"record_offset":3164,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":10152,"source_line":488,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":136,"record_offset":1316,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":1040,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":3532,"parent_record_offset":3476,"ancestor_record_offset":1040,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":136,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":1040,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":1040,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":1040,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":136,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":1040,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":136,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":1040,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":1040,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":1040,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":2808,"parent_record_offset":2776,"ancestor_record_offset":1040,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":2752,"parent_record_offset":2360,"ancestor_record_offset":1040,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1040,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":2392,"parent_record_offset":2360,"ancestor_record_offset":1040,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":2836,"parent_record_offset":2776,"ancestor_record_offset":1040,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":1040,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1040,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":2532,"parent_record_offset":2468,"ancestor_record_offset":1040,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1040,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":136,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1040,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":2928,"parent_record_offset":2360,"ancestor_record_offset":1040,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":1040,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":2360,"parent_record_offset":1344,"ancestor_record_offset":1040,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":136,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1040,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":136,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":1040,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":136,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1040,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":136,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":1040,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":136,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":1040,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":136,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1040,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":136,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1040,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":1040,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":2328,"parent_record_offset":1372,"ancestor_record_offset":1040,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":2596,"parent_record_offset":2416,"ancestor_record_offset":1040,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":1040,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":3048,"parent_record_offset":1316,"ancestor_record_offset":1040,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":136,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":1040,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":1040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":136,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1040,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":136,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1040,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":136,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":1040,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":136,"record_offset":2504,"parent_record_offset":2468,"ancestor_record_offset":1040,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":136,"record_offset":2676,"parent_record_offset":2644,"ancestor_record_offset":1040,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":136,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1040,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":136,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":1040,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":136,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1040,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":136,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1040,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":1040,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":1040,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":2020,"parent_record_offset":1824,"ancestor_record_offset":1040,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1932,"parent_record_offset":1848,"ancestor_record_offset":1040,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":1040,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":2276,"parent_record_offset":1496,"ancestor_record_offset":1040,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":1040,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1040,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":1040,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":1040,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":1040,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":1040,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":1792,"parent_record_offset":1676,"ancestor_record_offset":1040,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":1676,"parent_record_offset":1496,"ancestor_record_offset":1040,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":1824,"parent_record_offset":1496,"ancestor_record_offset":1040,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":1536,"parent_record_offset":1496,"ancestor_record_offset":1040,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":1620,"parent_record_offset":1496,"ancestor_record_offset":1040,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":2192,"parent_record_offset":1496,"ancestor_record_offset":1040,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":136,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1040,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":136,"record_offset":7448,"rva":2517552,"va":5371226672}
// 编译实例 RVAs: 0x266a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a7680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-783863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8090af4ff","size":5,"soft_fail":false,"target_va":5370443392,"va":5371227250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a7eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-781793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e81f12f4ff","size":5,"soft_fail":false,"target_va":5370445488,"va":5371227276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a85b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-780234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e83618f4ff","size":5,"soft_fail":false,"target_va":5370447280,"va":5371227509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403389f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t859300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8a41c0d00","size":5,"soft_fail":false,"target_va":5372086768,"va":5371227463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1223357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8bdaa1200","size":5,"soft_fail":false,"target_va":5372451200,"va":5371227838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1522149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8e5391700","size":5,"soft_fail":false,"target_va":5372749712,"va":5371227558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1922839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e817571d00","size":5,"soft_fail":false,"target_va":5373149840,"va":5371226996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140661c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4174076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8fcb03f00","size":5,"soft_fail":false,"target_va":5375401104,"va":5371227023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32365874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e832dded01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371227481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6302671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8cf2b6000","size":5,"soft_fail":false,"target_va":5377529888,"va":5371227212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31751256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8587ce401","size":5,"soft_fail":false,"target_va":5402978880,"va":5371227619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31751221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e8357ce401","size":5,"soft_fail":false,"target_va":5402978944,"va":5371227718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2986083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e863902d00","size":5,"soft_fail":false,"target_va":5374213616,"va":5371227528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5675823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371226672,"raw_hex":"e82f9b5600","size":5,"soft_fail":false,"target_va":5376903248,"va":5371227420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":136,"record_offset":9600,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":136,"record_offset":9516,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":136,"record_offset":10216,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":9812,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":136,"record_offset":9768,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::needs_texture_upload
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":136,"record_offset":9488,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":10151,"source_line":484,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":136,"record_offset":9572,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":10152,"source_line":488,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":136,"record_offset":7724,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":7448,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":9940,"parent_record_offset":9884,"ancestor_record_offset":7448,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":136,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":7448,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":9912,"parent_record_offset":9884,"ancestor_record_offset":7448,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":7448,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":136,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":7448,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":136,"record_offset":7780,"parent_record_offset":7752,"ancestor_record_offset":7448,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":9292,"parent_record_offset":9268,"ancestor_record_offset":7448,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":7448,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":9216,"parent_record_offset":9184,"ancestor_record_offset":7448,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":9160,"parent_record_offset":8768,"ancestor_record_offset":7448,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":9184,"parent_record_offset":9160,"ancestor_record_offset":7448,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":8800,"parent_record_offset":8768,"ancestor_record_offset":7448,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":9244,"parent_record_offset":9184,"ancestor_record_offset":7448,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":7448,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":8824,"parent_record_offset":8800,"ancestor_record_offset":7448,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":8940,"parent_record_offset":8876,"ancestor_record_offset":7448,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":7448,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":136,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":7448,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":9336,"parent_record_offset":8768,"ancestor_record_offset":7448,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":7752,"parent_record_offset":7724,"ancestor_record_offset":7448,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":8768,"parent_record_offset":7752,"ancestor_record_offset":7448,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":136,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":7448,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":136,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":7448,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":136,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":7448,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":136,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":7448,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":136,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":7448,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":136,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":7448,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":136,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":7448,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":7448,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":8736,"parent_record_offset":7780,"ancestor_record_offset":7448,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":9004,"parent_record_offset":8824,"ancestor_record_offset":7448,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":7448,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":9456,"parent_record_offset":7724,"ancestor_record_offset":7448,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":136,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":7448,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":7448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":136,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7448,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":136,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":7448,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":136,"record_offset":8876,"parent_record_offset":8848,"ancestor_record_offset":7448,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":136,"record_offset":8912,"parent_record_offset":8876,"ancestor_record_offset":7448,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":136,"record_offset":9084,"parent_record_offset":9052,"ancestor_record_offset":7448,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":136,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":7448,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":136,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":7448,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":136,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":7448,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":136,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7448,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":7448,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7448,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":8428,"parent_record_offset":8232,"ancestor_record_offset":7448,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":8340,"parent_record_offset":8256,"ancestor_record_offset":7448,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":8284,"parent_record_offset":8256,"ancestor_record_offset":7448,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":8684,"parent_record_offset":7904,"ancestor_record_offset":7448,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":7448,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":7448,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":7856,"parent_record_offset":7828,"ancestor_record_offset":7448,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":7904,"parent_record_offset":7880,"ancestor_record_offset":7448,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":7448,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":7448,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":8200,"parent_record_offset":8084,"ancestor_record_offset":7448,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":8084,"parent_record_offset":7904,"ancestor_record_offset":7448,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":8232,"parent_record_offset":7904,"ancestor_record_offset":7448,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":7944,"parent_record_offset":7904,"ancestor_record_offset":7448,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":8028,"parent_record_offset":7904,"ancestor_record_offset":7448,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":8600,"parent_record_offset":7904,"ancestor_record_offset":7448,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":136,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":7448,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":136,"record_offset":4244,"rva":2516128,"va":5371225248}
// 编译实例 RVAs: 0x2664a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a7680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-782439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8990ff4ff","size":5,"soft_fail":false,"target_va":5370443392,"va":5371225826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a8080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-779905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e87f19f4ff","size":5,"soft_fail":false,"target_va":5370445952,"va":5371225852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a85b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-778810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8c61df4ff","size":5,"soft_fail":false,"target_va":5370447280,"va":5371226085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403389f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t860724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e834220d00","size":5,"soft_fail":false,"target_va":5372086768,"va":5371226039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1224781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e84db01200","size":5,"soft_fail":false,"target_va":5372451200,"va":5371226414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1523573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8753f1700","size":5,"soft_fail":false,"target_va":5372749712,"va":5371226134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1924263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8a75c1d00","size":5,"soft_fail":false,"target_va":5373149840,"va":5371225572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140661c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4175500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e88cb63f00","size":5,"soft_fail":false,"target_va":5375401104,"va":5371225599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32367298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8c2e2ed01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371226057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6304095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e85f316000","size":5,"soft_fail":false,"target_va":5377529888,"va":5371225788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31752680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8e881e401","size":5,"soft_fail":false,"target_va":5402978880,"va":5371226195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31752645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8c581e401","size":5,"soft_fail":false,"target_va":5402978944,"va":5371226294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2987507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8f3952d00","size":5,"soft_fail":false,"target_va":5374213616,"va":5371226104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5677247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371225248,"raw_hex":"e8bfa05600","size":5,"soft_fail":false,"target_va":5376903248,"va":5371225996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":136,"record_offset":6396,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":136,"record_offset":6312,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":136,"record_offset":7012,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":6608,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":136,"record_offset":6564,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::needs_texture_upload
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":136,"record_offset":6284,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":10151,"source_line":484,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":136,"record_offset":6368,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":10152,"source_line":488,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":136,"record_offset":4520,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":4244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":6736,"parent_record_offset":6680,"ancestor_record_offset":4244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":136,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":4244,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":6708,"parent_record_offset":6680,"ancestor_record_offset":4244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":4244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":136,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":4244,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":136,"record_offset":4576,"parent_record_offset":4548,"ancestor_record_offset":4244,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":4244,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":4244,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":136,"record_offset":6012,"parent_record_offset":5980,"ancestor_record_offset":4244,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":5956,"parent_record_offset":5564,"ancestor_record_offset":4244,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":4244,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":5596,"parent_record_offset":5564,"ancestor_record_offset":4244,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":136,"record_offset":6040,"parent_record_offset":5980,"ancestor_record_offset":4244,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":4244,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":4244,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":5736,"parent_record_offset":5672,"ancestor_record_offset":4244,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":136,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4244,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":136,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":4244,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":6132,"parent_record_offset":5564,"ancestor_record_offset":4244,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":4548,"parent_record_offset":4520,"ancestor_record_offset":4244,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":136,"record_offset":5564,"parent_record_offset":4548,"ancestor_record_offset":4244,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":136,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4244,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":136,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":4244,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":136,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":4244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":136,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":4244,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":136,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4244,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":136,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":4244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":136,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4244,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":4244,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":5532,"parent_record_offset":4576,"ancestor_record_offset":4244,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":5800,"parent_record_offset":5620,"ancestor_record_offset":4244,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":4244,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":136,"record_offset":6252,"parent_record_offset":4520,"ancestor_record_offset":4244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":136,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":4244,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":4244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":136,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4244,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":136,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4244,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":136,"record_offset":5672,"parent_record_offset":5644,"ancestor_record_offset":4244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":136,"record_offset":5708,"parent_record_offset":5672,"ancestor_record_offset":4244,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":136,"record_offset":5880,"parent_record_offset":5848,"ancestor_record_offset":4244,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":136,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":4244,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":136,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":4244,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":136,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":4244,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":136,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":4244,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":4244,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":4244,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":136,"record_offset":5224,"parent_record_offset":5028,"ancestor_record_offset":4244,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":5136,"parent_record_offset":5052,"ancestor_record_offset":4244,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":4244,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":5480,"parent_record_offset":4700,"ancestor_record_offset":4244,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":4244,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":4244,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":4244,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":4244,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":4908,"parent_record_offset":4880,"ancestor_record_offset":4244,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":4244,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":4996,"parent_record_offset":4880,"ancestor_record_offset":4244,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":136,"record_offset":4880,"parent_record_offset":4700,"ancestor_record_offset":4244,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":136,"record_offset":5028,"parent_record_offset":4700,"ancestor_record_offset":4244,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":4740,"parent_record_offset":4700,"ancestor_record_offset":4244,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":4824,"parent_record_offset":4700,"ancestor_record_offset":4244,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":136,"record_offset":5396,"parent_record_offset":4700,"ancestor_record_offset":4244,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":136,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4244,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00136.jsonl.gz"}
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
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":136,"record_offset":3360,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00136.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":136,"record_offset":9768,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00136.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":136,"record_offset":6564,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00136.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3137946,"position_among_mapped_render_targets":3,"mapped_render_target_count":70,"previous_callsite":3137068,"next_callsite":3137971,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3137971,"position_among_mapped_render_targets":4,"mapped_render_target_count":70,"previous_callsite":3137946,"next_callsite":3137996,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3137996,"position_among_mapped_render_targets":5,"mapped_render_target_count":70,"previous_callsite":3137971,"next_callsite":3138018,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: upload_world_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// 信息 ID: 3cfd8cf9dd7c9b70eeab0375
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::GardenRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe1cc -> 0x265f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// 信息 ID: 9ac628d73698caba24e1d476
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::PathRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe19a -> 0x266a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// 信息 ID: f500a887d475f1fb726a40af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::WaterRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe1b3 -> 0x2664a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
