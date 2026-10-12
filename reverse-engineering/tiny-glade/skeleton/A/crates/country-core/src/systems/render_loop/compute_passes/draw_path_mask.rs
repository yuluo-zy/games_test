// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/draw_path_mask.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: deff8a21cdc8d045c27b978fcf03930bd5b7eb11d314bd2615d32b1354c495aa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_path_mask.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 459
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::draw_path_mask::draw_path_mask_compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":459,"record_offset":56996,"rva":6494336,"va":5375203456}
// 编译实例 RVAs: 0x631880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-988782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e892e9f0ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375204457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-687682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8be81f5ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375204157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t68567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8d70b0100","size":5,"soft_fail":false,"target_va":5375274368,"va":5375205796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t70522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e87a130100","size":5,"soft_fail":false,"target_va":5375275808,"va":5375205281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t109839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e80fad0100","size":5,"soft_fail":false,"target_va":5375314864,"va":5375205020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t771043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8e3c30b00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375207576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1034874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e87aca0f00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375205041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7980129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e861c47900","size":5,"soft_fail":false,"target_va":5383184544,"va":5375204410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8023995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8bb6f7a00","size":5,"soft_fail":false,"target_va":5383229152,"va":5375205152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd9070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8024424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e868717a00","size":5,"soft_fail":false,"target_va":5383229552,"va":5375205123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28388881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8112eb101","size":5,"soft_fail":false,"target_va":5403593360,"va":5375204474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_path_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28389313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8c12fb101","size":5,"soft_fail":false,"target_va":5403593392,"va":5375204074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27771441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e831c2a701","size":5,"soft_fail":false,"target_va":5402978880,"va":5375207434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1136402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e812571100","size":5,"soft_fail":false,"target_va":5376340512,"va":5375204105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1015350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375203456,"raw_hex":"e8367e0f00","size":5,"soft_fail":false,"target_va":5376219552,"va":5375204197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":459,"record_offset":60572,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":459,"record_offset":60544,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":5568,"source_line":337,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":459,"record_offset":61112,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":459,"record_offset":62440,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":459,"record_offset":61064,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":459,"record_offset":60968,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::on_texture_processed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":459,"record_offset":60628,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":25758,"source_line":497,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":459,"record_offset":60888,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":459,"record_offset":58976,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":459,"record_offset":57212,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":459,"record_offset":60688,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":459,"record_offset":60596,"parent_record_offset":60572,"ancestor_record_offset":56996,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":459,"record_offset":57268,"parent_record_offset":57240,"ancestor_record_offset":56996,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":459,"record_offset":58780,"parent_record_offset":58756,"ancestor_record_offset":56996,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":459,"record_offset":58756,"parent_record_offset":58732,"ancestor_record_offset":56996,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":459,"record_offset":58704,"parent_record_offset":58672,"ancestor_record_offset":56996,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":459,"record_offset":60412,"parent_record_offset":60024,"ancestor_record_offset":56996,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":459,"record_offset":58648,"parent_record_offset":58256,"ancestor_record_offset":56996,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":459,"record_offset":58672,"parent_record_offset":58648,"ancestor_record_offset":56996,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":459,"record_offset":58288,"parent_record_offset":58256,"ancestor_record_offset":56996,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":459,"record_offset":58732,"parent_record_offset":58672,"ancestor_record_offset":56996,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":459,"record_offset":58336,"parent_record_offset":58312,"ancestor_record_offset":56996,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":459,"record_offset":58312,"parent_record_offset":58288,"ancestor_record_offset":56996,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":459,"record_offset":58428,"parent_record_offset":58364,"ancestor_record_offset":56996,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":459,"record_offset":58516,"parent_record_offset":58492,"ancestor_record_offset":56996,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":459,"record_offset":58848,"parent_record_offset":58824,"ancestor_record_offset":56996,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":459,"record_offset":58824,"parent_record_offset":58256,"ancestor_record_offset":56996,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":459,"record_offset":59008,"parent_record_offset":58976,"ancestor_record_offset":56996,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":459,"record_offset":60024,"parent_record_offset":59008,"ancestor_record_offset":56996,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":459,"record_offset":57240,"parent_record_offset":57212,"ancestor_record_offset":56996,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":459,"record_offset":58256,"parent_record_offset":57240,"ancestor_record_offset":56996,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":459,"record_offset":57988,"parent_record_offset":57964,"ancestor_record_offset":56996,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":459,"record_offset":58036,"parent_record_offset":58012,"ancestor_record_offset":56996,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":459,"record_offset":57964,"parent_record_offset":57940,"ancestor_record_offset":56996,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":459,"record_offset":60756,"parent_record_offset":60688,"ancestor_record_offset":56996,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":459,"record_offset":57456,"parent_record_offset":57432,"ancestor_record_offset":56996,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":459,"record_offset":57540,"parent_record_offset":57516,"ancestor_record_offset":56996,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":459,"record_offset":57480,"parent_record_offset":57456,"ancestor_record_offset":56996,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":459,"record_offset":57652,"parent_record_offset":57628,"ancestor_record_offset":56996,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":459,"record_offset":58452,"parent_record_offset":58428,"ancestor_record_offset":56996,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":459,"record_offset":58224,"parent_record_offset":57268,"ancestor_record_offset":56996,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":459,"record_offset":58492,"parent_record_offset":58312,"ancestor_record_offset":56996,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":459,"record_offset":58944,"parent_record_offset":57212,"ancestor_record_offset":56996,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":459,"record_offset":58596,"parent_record_offset":58572,"ancestor_record_offset":56996,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":459,"record_offset":61216,"parent_record_offset":61144,"ancestor_record_offset":56996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":459,"record_offset":57876,"parent_record_offset":57852,"ancestor_record_offset":56996,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":459,"record_offset":57796,"parent_record_offset":57772,"ancestor_record_offset":56996,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":459,"record_offset":58364,"parent_record_offset":58336,"ancestor_record_offset":56996,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":459,"record_offset":58400,"parent_record_offset":58364,"ancestor_record_offset":56996,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":459,"record_offset":58572,"parent_record_offset":58540,"ancestor_record_offset":56996,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":459,"record_offset":58872,"parent_record_offset":58848,"ancestor_record_offset":56996,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":459,"record_offset":58896,"parent_record_offset":58872,"ancestor_record_offset":56996,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":459,"record_offset":58012,"parent_record_offset":57988,"ancestor_record_offset":56996,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":459,"record_offset":61144,"parent_record_offset":61112,"ancestor_record_offset":56996,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":459,"record_offset":60916,"parent_record_offset":60888,"ancestor_record_offset":56996,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":459,"record_offset":60656,"parent_record_offset":60628,"ancestor_record_offset":56996,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":459,"record_offset":57940,"parent_record_offset":57916,"ancestor_record_offset":56996,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":459,"record_offset":57292,"parent_record_offset":57268,"ancestor_record_offset":56996,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":459,"record_offset":57316,"parent_record_offset":57292,"ancestor_record_offset":56996,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":459,"record_offset":57916,"parent_record_offset":57720,"ancestor_record_offset":56996,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57828,"parent_record_offset":57744,"ancestor_record_offset":56996,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57772,"parent_record_offset":57744,"ancestor_record_offset":56996,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":58172,"parent_record_offset":57392,"ancestor_record_offset":56996,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57744,"parent_record_offset":57720,"ancestor_record_offset":56996,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57368,"parent_record_offset":57344,"ancestor_record_offset":56996,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57344,"parent_record_offset":57316,"ancestor_record_offset":56996,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57392,"parent_record_offset":57368,"ancestor_record_offset":56996,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":459,"record_offset":57600,"parent_record_offset":57572,"ancestor_record_offset":56996,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":459,"record_offset":57628,"parent_record_offset":57600,"ancestor_record_offset":56996,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":459,"record_offset":57688,"parent_record_offset":57572,"ancestor_record_offset":56996,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":459,"record_offset":57572,"parent_record_offset":57392,"ancestor_record_offset":56996,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":459,"record_offset":57720,"parent_record_offset":57392,"ancestor_record_offset":56996,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":459,"record_offset":57432,"parent_record_offset":57392,"ancestor_record_offset":56996,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":459,"record_offset":57516,"parent_record_offset":57392,"ancestor_record_offset":56996,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":459,"record_offset":58088,"parent_record_offset":57392,"ancestor_record_offset":56996,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":459,"record_offset":58540,"parent_record_offset":58516,"ancestor_record_offset":56996,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00459.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":459,"record_offset":60728,"parent_record_offset":60688,"ancestor_record_offset":56996,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00459.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_path_mask.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_path_mask.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/draw_path_mask.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: draw_path_mask.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::draw_path_mask::draw_path_mask_compute
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":459,"record_offset":60888,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00459.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":459,"record_offset":60916,"parent_record_offset":60888,"ancestor_record_offset":56996,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00459.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":459,"record_offset":60968,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00459.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":459,"record_offset":61064,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00459.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":459,"record_offset":62440,"parent_record_offset":56996,"ancestor_record_offset":56996,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00459.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138150,"position_among_mapped_render_targets":12,"mapped_render_target_count":70,"previous_callsite":3138128,"next_callsite":3138172,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_path_mask.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::draw_path_mask::draw_path_mask_compute
// 信息 ID: 2032cc4055ac8e4f26f97ad4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe266 -> 0x631880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
