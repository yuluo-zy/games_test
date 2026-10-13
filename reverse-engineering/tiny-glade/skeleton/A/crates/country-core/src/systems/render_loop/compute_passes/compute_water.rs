// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ede30fefcb7398100d38ef5a151c0da2656e7fb741f933d871bf1cb6a7c052a0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compute_water.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 448
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
// 函数符号: country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":34044,"rva":6315168,"va":5375024288}
// 编译实例 RVAs: 0x605ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-809214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e802a7f3ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375024889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-508229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8bb3ef8ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375024704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t248876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e82ccc0300","size":5,"soft_fail":false,"target_va":5375274368,"va":5375025487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t250843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8dbd30300","size":5,"soft_fail":false,"target_va":5375275808,"va":5375024960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t289522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8f26a0400","size":5,"soft_fail":false,"target_va":5375314864,"va":5375025337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t951335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e827840e00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375027284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1214557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e85d881200","size":5,"soft_fail":false,"target_va":5376239920,"va":5375025358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8159728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8f0817c00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375024811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8203298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8222c7d00","size":5,"soft_fail":false,"target_va":5383228352,"va":5375025049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8204129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8612f7d00","size":5,"soft_fail":false,"target_va":5383229152,"va":5375025018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd9070
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8204705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8a1317d00","size":5,"soft_fail":false,"target_va":5383229552,"va":5375024842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28568449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e881ebb301","size":5,"soft_fail":false,"target_va":5403593360,"va":5375024906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28568759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e8b7ecb301","size":5,"soft_fail":false,"target_va":5403593392,"va":5375024628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27951713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e86182aa01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375027162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1315851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e80b141400","size":5,"soft_fail":false,"target_va":5376340512,"va":5375024656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1194812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375024288,"raw_hex":"e83c3b1200","size":5,"soft_fail":false,"target_va":5376219552,"va":5375024735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":448,"record_offset":36040,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":36352,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":36304,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":36256,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::needs_texture_processing
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":448,"record_offset":36012,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":25757,"source_line":493,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::on_texture_processed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":448,"record_offset":36096,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":25758,"source_line":497,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":448,"record_offset":34248,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":448,"record_offset":36156,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":448,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":34044,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":448,"record_offset":34304,"parent_record_offset":34276,"ancestor_record_offset":34044,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":448,"record_offset":35816,"parent_record_offset":35792,"ancestor_record_offset":34044,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":448,"record_offset":35792,"parent_record_offset":35768,"ancestor_record_offset":34044,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":448,"record_offset":35740,"parent_record_offset":35708,"ancestor_record_offset":34044,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":35684,"parent_record_offset":35292,"ancestor_record_offset":34044,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":35708,"parent_record_offset":35684,"ancestor_record_offset":34044,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":35324,"parent_record_offset":35292,"ancestor_record_offset":34044,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":35768,"parent_record_offset":35708,"ancestor_record_offset":34044,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":35372,"parent_record_offset":35348,"ancestor_record_offset":34044,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":35348,"parent_record_offset":35324,"ancestor_record_offset":34044,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":35464,"parent_record_offset":35400,"ancestor_record_offset":34044,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":35552,"parent_record_offset":35528,"ancestor_record_offset":34044,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":448,"record_offset":35884,"parent_record_offset":35860,"ancestor_record_offset":34044,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":448,"record_offset":35860,"parent_record_offset":35292,"ancestor_record_offset":34044,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":448,"record_offset":34276,"parent_record_offset":34248,"ancestor_record_offset":34044,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":448,"record_offset":35292,"parent_record_offset":34276,"ancestor_record_offset":34044,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":448,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":34044,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":35072,"parent_record_offset":35048,"ancestor_record_offset":34044,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":34044,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":448,"record_offset":36224,"parent_record_offset":36156,"ancestor_record_offset":34044,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":448,"record_offset":34492,"parent_record_offset":34468,"ancestor_record_offset":34044,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":448,"record_offset":34576,"parent_record_offset":34552,"ancestor_record_offset":34044,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":448,"record_offset":34516,"parent_record_offset":34492,"ancestor_record_offset":34044,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":448,"record_offset":34688,"parent_record_offset":34664,"ancestor_record_offset":34044,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":35488,"parent_record_offset":35464,"ancestor_record_offset":34044,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":35260,"parent_record_offset":34304,"ancestor_record_offset":34044,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":35528,"parent_record_offset":35348,"ancestor_record_offset":34044,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":35980,"parent_record_offset":34248,"ancestor_record_offset":34044,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":448,"record_offset":35632,"parent_record_offset":35608,"ancestor_record_offset":34044,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":36456,"parent_record_offset":36384,"ancestor_record_offset":34044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":448,"record_offset":34912,"parent_record_offset":34888,"ancestor_record_offset":34044,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":448,"record_offset":34832,"parent_record_offset":34808,"ancestor_record_offset":34044,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":448,"record_offset":35400,"parent_record_offset":35372,"ancestor_record_offset":34044,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":35436,"parent_record_offset":35400,"ancestor_record_offset":34044,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":35608,"parent_record_offset":35576,"ancestor_record_offset":34044,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":448,"record_offset":35908,"parent_record_offset":35884,"ancestor_record_offset":34044,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":448,"record_offset":35932,"parent_record_offset":35908,"ancestor_record_offset":34044,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":448,"record_offset":35048,"parent_record_offset":35024,"ancestor_record_offset":34044,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":36384,"parent_record_offset":36352,"ancestor_record_offset":34044,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":448,"record_offset":36124,"parent_record_offset":36096,"ancestor_record_offset":34044,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":448,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":34044,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":448,"record_offset":34328,"parent_record_offset":34304,"ancestor_record_offset":34044,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":448,"record_offset":34352,"parent_record_offset":34328,"ancestor_record_offset":34044,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":448,"record_offset":34952,"parent_record_offset":34756,"ancestor_record_offset":34044,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34864,"parent_record_offset":34780,"ancestor_record_offset":34044,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34808,"parent_record_offset":34780,"ancestor_record_offset":34044,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":35208,"parent_record_offset":34428,"ancestor_record_offset":34044,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34780,"parent_record_offset":34756,"ancestor_record_offset":34044,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34404,"parent_record_offset":34380,"ancestor_record_offset":34044,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34380,"parent_record_offset":34352,"ancestor_record_offset":34044,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34428,"parent_record_offset":34404,"ancestor_record_offset":34044,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":34636,"parent_record_offset":34608,"ancestor_record_offset":34044,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":34664,"parent_record_offset":34636,"ancestor_record_offset":34044,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":34724,"parent_record_offset":34608,"ancestor_record_offset":34044,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":34608,"parent_record_offset":34428,"ancestor_record_offset":34044,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":34756,"parent_record_offset":34428,"ancestor_record_offset":34044,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":448,"record_offset":34468,"parent_record_offset":34428,"ancestor_record_offset":34044,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":448,"record_offset":34552,"parent_record_offset":34428,"ancestor_record_offset":34044,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":448,"record_offset":35124,"parent_record_offset":34428,"ancestor_record_offset":34044,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":448,"record_offset":35576,"parent_record_offset":35552,"ancestor_record_offset":34044,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":448,"record_offset":36196,"parent_record_offset":36156,"ancestor_record_offset":34044,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":37988,"rva":6318880,"va":5375028000}
// 编译实例 RVAs: 0x606b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-812860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e8c498f3ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375028535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t285609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e8a95b0400","size":5,"soft_fail":false,"target_va":5375314864,"va":5375029250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t948364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e88c780e00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375030255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1210644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e814791200","size":5,"soft_fail":false,"target_va":5376239920,"va":5375029271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8156437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e815757c00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375028102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd1cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8171792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e810b17c00","size":5,"soft_fail":false,"target_va":5383199952,"va":5375028155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd2ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8176457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e849c37c00","size":5,"soft_fail":false,"target_va":5383204592,"va":5375028130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd4bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8182665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e889db7c00","size":5,"soft_fail":false,"target_va":5383212016,"va":5375029346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd5540
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8185501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e89de67c00","size":5,"soft_fail":false,"target_va":5383214400,"va":5375028894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8d50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8200256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e840207d00","size":5,"soft_fail":false,"target_va":5383228752,"va":5375028491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd9070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8201101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e88d237d00","size":5,"soft_fail":false,"target_va":5383229552,"va":5375028446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8210332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e89c477d00","size":5,"soft_fail":false,"target_va":5383239280,"va":5375028943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: compute_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28564789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e835ddb301","size":5,"soft_fail":false,"target_va":5403593360,"va":5375028566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27948727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375028000,"raw_hex":"e8b776aa01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375030148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":40192,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_draw_indirect_cmd_buffer
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":40084,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":22663,"source_line":386,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":40036,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":39892,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":448,"record_offset":39756,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":22564,"source_line":210,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":448,"record_offset":39812,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":448,"record_offset":38188,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":448,"record_offset":38248,"parent_record_offset":38220,"ancestor_record_offset":37988,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":448,"record_offset":39680,"parent_record_offset":39648,"ancestor_record_offset":37988,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":39624,"parent_record_offset":39236,"ancestor_record_offset":37988,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":39648,"parent_record_offset":39624,"ancestor_record_offset":37988,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":448,"record_offset":39264,"parent_record_offset":39236,"ancestor_record_offset":37988,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":39312,"parent_record_offset":39288,"ancestor_record_offset":37988,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":39288,"parent_record_offset":39264,"ancestor_record_offset":37988,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":39404,"parent_record_offset":39340,"ancestor_record_offset":37988,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":448,"record_offset":39492,"parent_record_offset":39468,"ancestor_record_offset":37988,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":448,"record_offset":38220,"parent_record_offset":38188,"ancestor_record_offset":37988,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":448,"record_offset":39236,"parent_record_offset":38220,"ancestor_record_offset":37988,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":448,"record_offset":38968,"parent_record_offset":38944,"ancestor_record_offset":37988,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":37988,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":38944,"parent_record_offset":38920,"ancestor_record_offset":37988,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":448,"record_offset":38436,"parent_record_offset":38412,"ancestor_record_offset":37988,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":448,"record_offset":38520,"parent_record_offset":38496,"ancestor_record_offset":37988,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":448,"record_offset":38460,"parent_record_offset":38436,"ancestor_record_offset":37988,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":448,"record_offset":38632,"parent_record_offset":38608,"ancestor_record_offset":37988,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":40156,"parent_record_offset":40112,"ancestor_record_offset":37988,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":39428,"parent_record_offset":39404,"ancestor_record_offset":37988,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":39204,"parent_record_offset":38248,"ancestor_record_offset":37988,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":39468,"parent_record_offset":39288,"ancestor_record_offset":37988,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":39724,"parent_record_offset":38188,"ancestor_record_offset":37988,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":448,"record_offset":39572,"parent_record_offset":39548,"ancestor_record_offset":37988,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":40296,"parent_record_offset":40224,"ancestor_record_offset":37988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":448,"record_offset":38856,"parent_record_offset":38832,"ancestor_record_offset":37988,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":448,"record_offset":38776,"parent_record_offset":38752,"ancestor_record_offset":37988,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":448,"record_offset":39340,"parent_record_offset":39312,"ancestor_record_offset":37988,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":39376,"parent_record_offset":39340,"ancestor_record_offset":37988,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":39548,"parent_record_offset":39516,"ancestor_record_offset":37988,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":448,"record_offset":38992,"parent_record_offset":38968,"ancestor_record_offset":37988,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":40224,"parent_record_offset":40192,"ancestor_record_offset":37988,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":39840,"parent_record_offset":39812,"ancestor_record_offset":37988,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":448,"record_offset":40112,"parent_record_offset":40084,"ancestor_record_offset":37988,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":448,"record_offset":39780,"parent_record_offset":39756,"ancestor_record_offset":37988,"inlinee":22565,"source_line":101,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":448,"record_offset":38920,"parent_record_offset":38896,"ancestor_record_offset":37988,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":448,"record_offset":38272,"parent_record_offset":38248,"ancestor_record_offset":37988,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":448,"record_offset":38296,"parent_record_offset":38272,"ancestor_record_offset":37988,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":448,"record_offset":38896,"parent_record_offset":38700,"ancestor_record_offset":37988,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38808,"parent_record_offset":38724,"ancestor_record_offset":37988,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38752,"parent_record_offset":38724,"ancestor_record_offset":37988,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":39152,"parent_record_offset":38372,"ancestor_record_offset":37988,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38724,"parent_record_offset":38700,"ancestor_record_offset":37988,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38348,"parent_record_offset":38324,"ancestor_record_offset":37988,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38324,"parent_record_offset":38296,"ancestor_record_offset":37988,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38372,"parent_record_offset":38348,"ancestor_record_offset":37988,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":38580,"parent_record_offset":38552,"ancestor_record_offset":37988,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":38608,"parent_record_offset":38580,"ancestor_record_offset":37988,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":38668,"parent_record_offset":38552,"ancestor_record_offset":37988,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":448,"record_offset":38552,"parent_record_offset":38372,"ancestor_record_offset":37988,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":448,"record_offset":38700,"parent_record_offset":38372,"ancestor_record_offset":37988,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":448,"record_offset":38412,"parent_record_offset":38372,"ancestor_record_offset":37988,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":448,"record_offset":38496,"parent_record_offset":38372,"ancestor_record_offset":37988,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":448,"record_offset":39068,"parent_record_offset":38372,"ancestor_record_offset":37988,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":448,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":37988,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00448.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":448,"record_offset":36256,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":448,"record_offset":36304,"parent_record_offset":34044,"ancestor_record_offset":34044,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":448,"record_offset":39812,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":448,"record_offset":39840,"parent_record_offset":39812,"ancestor_record_offset":37988,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":448,"record_offset":39892,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":448,"record_offset":40036,"parent_record_offset":37988,"ancestor_record_offset":37988,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":448,"record_offset":40112,"parent_record_offset":40084,"ancestor_record_offset":37988,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00448.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138018,"position_among_mapped_render_targets":6,"mapped_render_target_count":70,"previous_callsite":3137996,"next_callsite":3138040,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138216,"position_among_mapped_render_targets":15,"mapped_render_target_count":70,"previous_callsite":3138194,"next_callsite":3138238,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compute_water.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
// 信息 ID: 08a4ffa869ce3b80dfc64438
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe1e2 -> 0x605ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
// 信息 ID: 26fc35a97842a5d68401ca79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe2a8 -> 0x606b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
