// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cab53b649bdfa545537b5c3a926bea929909aa76ba9425bcac7a45f6dbc1d56d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compute_garden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 498
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
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":19528,"rva":7140192,"va":5375849312}
// 编译实例 RVAs: 0x6cf360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1634247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e83910e7ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375849922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1333253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e8fba7ebff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375849728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-576154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e86635f7ff","size":5,"soft_fail":false,"target_va":5375274368,"va":5375850517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-574193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e80f3df7ff","size":5,"soft_fail":false,"target_va":5375275808,"va":5375849996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-535508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e82cd4f7ff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375850367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t127479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e8f7f10100","size":5,"soft_fail":false,"target_va":5375978624,"va":5375851140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t389527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e897f10500","size":5,"soft_fail":false,"target_va":5376239920,"va":5375850388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7334695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e827eb6f00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375849844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7378262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e856957000","size":5,"soft_fail":false,"target_va":5383228352,"va":5375850085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7379093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e895987000","size":5,"soft_fail":false,"target_va":5383229152,"va":5375850054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd9070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7379672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e8d89a7000","size":5,"soft_fail":false,"target_va":5383229552,"va":5375849875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27743416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e8b854a701","size":5,"soft_fail":false,"target_va":5403593360,"va":5375849939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27743735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e8f755a701","size":5,"soft_fail":false,"target_va":5403593392,"va":5375849652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27127863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e837f09d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375851012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t490827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e84b7d0700","size":5,"soft_fail":false,"target_va":5376340512,"va":5375849680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t369788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375849312,"raw_hex":"e87ca40500","size":5,"soft_fail":false,"target_va":5376219552,"va":5375849759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":498,"record_offset":21528,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":21840,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":21792,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":21744,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::needs_texture_processing
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":498,"record_offset":21500,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":25757,"source_line":493,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::on_texture_processed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":498,"record_offset":21584,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":25758,"source_line":497,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":498,"record_offset":19736,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":498,"record_offset":21644,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":498,"record_offset":21552,"parent_record_offset":21528,"ancestor_record_offset":19528,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":498,"record_offset":19792,"parent_record_offset":19764,"ancestor_record_offset":19528,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":498,"record_offset":21304,"parent_record_offset":21280,"ancestor_record_offset":19528,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":498,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":19528,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":498,"record_offset":21228,"parent_record_offset":21196,"ancestor_record_offset":19528,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":498,"record_offset":21172,"parent_record_offset":20780,"ancestor_record_offset":19528,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":498,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":19528,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":498,"record_offset":20812,"parent_record_offset":20780,"ancestor_record_offset":19528,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":498,"record_offset":21256,"parent_record_offset":21196,"ancestor_record_offset":19528,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":498,"record_offset":20860,"parent_record_offset":20836,"ancestor_record_offset":19528,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":498,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":19528,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":498,"record_offset":20952,"parent_record_offset":20888,"ancestor_record_offset":19528,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":498,"record_offset":21040,"parent_record_offset":21016,"ancestor_record_offset":19528,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":498,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":19528,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":498,"record_offset":21348,"parent_record_offset":20780,"ancestor_record_offset":19528,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":498,"record_offset":19764,"parent_record_offset":19736,"ancestor_record_offset":19528,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":498,"record_offset":20780,"parent_record_offset":19764,"ancestor_record_offset":19528,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":498,"record_offset":20512,"parent_record_offset":20488,"ancestor_record_offset":19528,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":498,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":19528,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":498,"record_offset":20488,"parent_record_offset":20464,"ancestor_record_offset":19528,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":498,"record_offset":21712,"parent_record_offset":21644,"ancestor_record_offset":19528,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":498,"record_offset":19980,"parent_record_offset":19956,"ancestor_record_offset":19528,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":498,"record_offset":20064,"parent_record_offset":20040,"ancestor_record_offset":19528,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":498,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":19528,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":498,"record_offset":20176,"parent_record_offset":20152,"ancestor_record_offset":19528,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":19528,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":20748,"parent_record_offset":19792,"ancestor_record_offset":19528,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":21016,"parent_record_offset":20836,"ancestor_record_offset":19528,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":21468,"parent_record_offset":19736,"ancestor_record_offset":19528,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":21120,"parent_record_offset":21096,"ancestor_record_offset":19528,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":21944,"parent_record_offset":21872,"ancestor_record_offset":19528,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":498,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":19528,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":498,"record_offset":20320,"parent_record_offset":20296,"ancestor_record_offset":19528,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":498,"record_offset":20888,"parent_record_offset":20860,"ancestor_record_offset":19528,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":498,"record_offset":20924,"parent_record_offset":20888,"ancestor_record_offset":19528,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":498,"record_offset":21096,"parent_record_offset":21064,"ancestor_record_offset":19528,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":21396,"parent_record_offset":21372,"ancestor_record_offset":19528,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":21420,"parent_record_offset":21396,"ancestor_record_offset":19528,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":498,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":19528,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":21872,"parent_record_offset":21840,"ancestor_record_offset":19528,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":498,"record_offset":21612,"parent_record_offset":21584,"ancestor_record_offset":19528,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":498,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":19528,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":498,"record_offset":19816,"parent_record_offset":19792,"ancestor_record_offset":19528,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":498,"record_offset":19840,"parent_record_offset":19816,"ancestor_record_offset":19528,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":498,"record_offset":20440,"parent_record_offset":20244,"ancestor_record_offset":19528,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":20352,"parent_record_offset":20268,"ancestor_record_offset":19528,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":20296,"parent_record_offset":20268,"ancestor_record_offset":19528,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":20696,"parent_record_offset":19916,"ancestor_record_offset":19528,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19528,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":19892,"parent_record_offset":19868,"ancestor_record_offset":19528,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":19868,"parent_record_offset":19840,"ancestor_record_offset":19528,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":19916,"parent_record_offset":19892,"ancestor_record_offset":19528,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":498,"record_offset":20124,"parent_record_offset":20096,"ancestor_record_offset":19528,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":498,"record_offset":20152,"parent_record_offset":20124,"ancestor_record_offset":19528,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":498,"record_offset":20212,"parent_record_offset":20096,"ancestor_record_offset":19528,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":498,"record_offset":20096,"parent_record_offset":19916,"ancestor_record_offset":19528,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":498,"record_offset":20244,"parent_record_offset":19916,"ancestor_record_offset":19528,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":498,"record_offset":19956,"parent_record_offset":19916,"ancestor_record_offset":19528,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":498,"record_offset":20040,"parent_record_offset":19916,"ancestor_record_offset":19528,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":498,"record_offset":20612,"parent_record_offset":19916,"ancestor_record_offset":19528,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":498,"record_offset":21064,"parent_record_offset":21040,"ancestor_record_offset":19528,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":498,"record_offset":21684,"parent_record_offset":21644,"ancestor_record_offset":19528,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":22612,"rva":7142320,"va":5375851440}
// 编译实例 RVAs: 0x6cfbb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=23
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1635988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e86c09e7ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375851663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-537580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e814ccf7ff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375852439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=27
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t123166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e81ee10100","size":5,"soft_fail":false,"target_va":5375978624,"va":5375855453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t387455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e87fe90500","size":5,"soft_fail":false,"target_va":5376239920,"va":5375852460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7333012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e894e46f00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375851527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd27b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7348913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e8b1227000","size":5,"soft_fail":false,"target_va":5383202736,"va":5375853818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd3290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7351104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e8402b7000","size":5,"soft_fail":false,"target_va":5383205520,"va":5375854411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd5350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7362338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e822577000","size":5,"soft_fail":false,"target_va":5383213904,"va":5375851561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd5540
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7362312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e808577000","size":5,"soft_fail":false,"target_va":5383214400,"va":5375852083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd5920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7363799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e8d75c7000","size":5,"soft_fail":false,"target_va":5383215392,"va":5375851588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8d50
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7377128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e8e8907000","size":5,"soft_fail":false,"target_va":5383228752,"va":5375851619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb670
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7387143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e807b87000","size":5,"soft_fail":false,"target_va":5383239280,"va":5375852132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=27
//     target_aliases=1 candidates; complete list: compute_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27741675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375851440,"raw_hex":"e8eb4da701","size":5,"soft_fail":false,"target_va":5403593360,"va":5375851680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":23232,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":24712,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_draw_indirect_cmd_buffer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":23124,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":22663,"source_line":386,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=19
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":22884,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":498,"record_offset":22804,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":23196,"parent_record_offset":23152,"ancestor_record_offset":22612,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":23336,"parent_record_offset":23264,"ancestor_record_offset":22612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":23264,"parent_record_offset":23232,"ancestor_record_offset":22612,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":22832,"parent_record_offset":22804,"ancestor_record_offset":22612,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":498,"record_offset":23152,"parent_record_offset":23124,"ancestor_record_offset":22612,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00498.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":498,"record_offset":21744,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00498.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":498,"record_offset":21792,"parent_record_offset":19528,"ancestor_record_offset":19528,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00498.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":498,"record_offset":22804,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00498.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":498,"record_offset":22832,"parent_record_offset":22804,"ancestor_record_offset":22612,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00498.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":498,"record_offset":22884,"parent_record_offset":22612,"ancestor_record_offset":22612,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00498.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":498,"record_offset":23152,"parent_record_offset":23124,"ancestor_record_offset":22612,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/00498.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138194,"position_among_mapped_render_targets":14,"mapped_render_target_count":70,"previous_callsite":3138172,"next_callsite":3138216,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138282,"position_among_mapped_render_targets":18,"mapped_render_target_count":70,"previous_callsite":3138260,"next_callsite":3138304,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compute_garden.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
// 信息 ID: 213a4437b39ca7f37bee27b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe292 -> 0x6cf360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
// 信息 ID: dc4c1f305fc476e8a5abbbb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe2ea -> 0x6cfbb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
