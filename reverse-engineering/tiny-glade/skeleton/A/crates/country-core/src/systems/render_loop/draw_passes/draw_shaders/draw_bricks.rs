// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 57f0ba6a6ace8640f5a73076c68fbf8a55a63c06646dfdbd7963552362c5d76b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_bricks.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 485
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
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":35416,"rva":6902608,"va":5375611728}
// 编译实例 RVAs: 0x695350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1396414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e842b1eaff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375613209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-189182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8021dfdff","size":5,"soft_fail":false,"target_va":5375423344,"va":5375612521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t102207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e83f8f0100","size":5,"soft_fail":false,"target_va":5375715360,"va":5375613148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406aeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t102902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8f6910100","size":5,"soft_fail":false,"target_va":5375716032,"va":5375613125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t353910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e876660500","size":5,"soft_fail":false,"target_va":5375967552,"va":5375613637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7736121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8390b7600","size":5,"soft_fail":false,"target_va":5383349424,"va":5375613298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfc620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7761104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8d06c7600","size":5,"soft_fail":false,"target_va":5383374368,"va":5375613259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfc910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7762375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8c7717600","size":5,"soft_fail":false,"target_va":5383375120,"va":5375612740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27344952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e83840a101","size":5,"soft_fail":false,"target_va":5402958528,"va":5375613571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27980742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8c6f3aa01","size":5,"soft_fail":false,"target_va":5403593344,"va":5375612597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27981302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8f6f5aa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375612053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27365426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e83290a101","size":5,"soft_fail":false,"target_va":5402978880,"va":5375613449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27365914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e81a92a101","size":5,"soft_fail":false,"target_va":5402979344,"va":5375613425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t90059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8cb5f0100","size":5,"soft_fail":false,"target_va":5375702768,"va":5375612704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t97767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8e77d0100","size":5,"soft_fail":false,"target_va":5375709840,"va":5375612068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t94192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375611728,"raw_hex":"e8f06f0100","size":5,"soft_fail":false,"target_va":5375707584,"va":5375613387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375611728,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375612628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":35876,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":22528,"source_line":691,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":35668,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":22526,"source_line":299,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":37688,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":37516,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":35840,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":485,"record_offset":39204,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::RenderViewIndex::is_shadow_view
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":485,"record_offset":35640,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":26229,"source_line":66,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":485,"record_offset":41024,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":21973,"source_line":82,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":41100,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":485,"record_offset":39480,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":27352,"source_line":577,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":37600,"parent_record_offset":35416,"ancestor_record_offset":35416,"inlinee":21969,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":37012,"parent_record_offset":36972,"ancestor_record_offset":35416,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":35916,"parent_record_offset":35876,"ancestor_record_offset":35416,"inlinee":22529,"source_line":713,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":37716,"parent_record_offset":37688,"ancestor_record_offset":35416,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":485,"record_offset":35692,"parent_record_offset":35668,"ancestor_record_offset":35416,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":37476,"parent_record_offset":37452,"ancestor_record_offset":35416,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":39160,"parent_record_offset":39128,"ancestor_record_offset":35416,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":39104,"parent_record_offset":38716,"ancestor_record_offset":35416,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":39128,"parent_record_offset":39104,"ancestor_record_offset":35416,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":38744,"parent_record_offset":38716,"ancestor_record_offset":35416,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":37136,"parent_record_offset":37108,"ancestor_record_offset":35416,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":37108,"parent_record_offset":36972,"ancestor_record_offset":35416,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":38792,"parent_record_offset":38768,"ancestor_record_offset":35416,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":38768,"parent_record_offset":38744,"ancestor_record_offset":35416,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":38884,"parent_record_offset":38820,"ancestor_record_offset":35416,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":37228,"parent_record_offset":37164,"ancestor_record_offset":35416,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":38972,"parent_record_offset":38948,"ancestor_record_offset":35416,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":37316,"parent_record_offset":37292,"ancestor_record_offset":35416,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":37452,"parent_record_offset":36900,"ancestor_record_offset":35416,"inlinee":22535,"source_line":383,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$7::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":36972,"parent_record_offset":36940,"ancestor_record_offset":35416,"inlinee":22534,"source_line":904,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":36940,"parent_record_offset":36900,"ancestor_record_offset":35416,"inlinee":22532,"source_line":910,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":38716,"parent_record_offset":37688,"ancestor_record_offset":35416,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":36900,"parent_record_offset":35876,"ancestor_record_offset":35416,"inlinee":22531,"source_line":942,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":36660,"parent_record_offset":36636,"ancestor_record_offset":35416,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":35800,"parent_record_offset":35776,"ancestor_record_offset":35416,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":36708,"parent_record_offset":36684,"ancestor_record_offset":35416,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":36636,"parent_record_offset":36612,"ancestor_record_offset":35416,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":36124,"parent_record_offset":36100,"ancestor_record_offset":35416,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":36208,"parent_record_offset":36184,"ancestor_record_offset":35416,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":485,"record_offset":36148,"parent_record_offset":36124,"ancestor_record_offset":35416,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":35416,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":35776,"parent_record_offset":35692,"ancestor_record_offset":35416,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":37252,"parent_record_offset":37228,"ancestor_record_offset":35416,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":38684,"parent_record_offset":37716,"ancestor_record_offset":35416,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":37292,"parent_record_offset":37108,"ancestor_record_offset":35416,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":37540,"parent_record_offset":37516,"ancestor_record_offset":35416,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":37564,"parent_record_offset":37540,"ancestor_record_offset":35416,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":37396,"parent_record_offset":37372,"ancestor_record_offset":35416,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":36548,"parent_record_offset":36524,"ancestor_record_offset":35416,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":36468,"parent_record_offset":36444,"ancestor_record_offset":35416,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":485,"record_offset":39356,"parent_record_offset":39328,"ancestor_record_offset":35416,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":485,"record_offset":39384,"parent_record_offset":39356,"ancestor_record_offset":35416,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":35720,"parent_record_offset":35692,"ancestor_record_offset":35416,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":37040,"parent_record_offset":37012,"ancestor_record_offset":35416,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":35744,"parent_record_offset":35720,"ancestor_record_offset":35416,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":37372,"parent_record_offset":37340,"ancestor_record_offset":35416,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":37068,"parent_record_offset":37040,"ancestor_record_offset":35416,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":485,"record_offset":39300,"parent_record_offset":39272,"ancestor_record_offset":35416,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":485,"record_offset":36684,"parent_record_offset":36660,"ancestor_record_offset":35416,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":485,"record_offset":39232,"parent_record_offset":39204,"ancestor_record_offset":35416,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":41052,"parent_record_offset":41024,"ancestor_record_offset":35416,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":485,"record_offset":36612,"parent_record_offset":36588,"ancestor_record_offset":35416,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":35416,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":35972,"parent_record_offset":35944,"ancestor_record_offset":35416,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":36588,"parent_record_offset":36388,"ancestor_record_offset":35416,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36500,"parent_record_offset":36416,"ancestor_record_offset":35416,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36444,"parent_record_offset":36416,"ancestor_record_offset":35416,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36844,"parent_record_offset":36052,"ancestor_record_offset":35416,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36416,"parent_record_offset":36388,"ancestor_record_offset":35416,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":35416,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36004,"parent_record_offset":35972,"ancestor_record_offset":35416,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36052,"parent_record_offset":36028,"ancestor_record_offset":35416,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":36268,"parent_record_offset":36240,"ancestor_record_offset":35416,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":36296,"parent_record_offset":36268,"ancestor_record_offset":35416,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":36356,"parent_record_offset":36240,"ancestor_record_offset":35416,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":36240,"parent_record_offset":36052,"ancestor_record_offset":35416,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":36388,"parent_record_offset":36052,"ancestor_record_offset":35416,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":36100,"parent_record_offset":36052,"ancestor_record_offset":35416,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":36184,"parent_record_offset":36052,"ancestor_record_offset":35416,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":36760,"parent_record_offset":36052,"ancestor_record_offset":35416,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":485,"record_offset":37648,"parent_record_offset":37624,"ancestor_record_offset":35416,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":37340,"parent_record_offset":37316,"ancestor_record_offset":35416,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":37624,"parent_record_offset":37600,"ancestor_record_offset":35416,"inlinee":21970,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":41216,"rva":6904704,"va":5375613824}
// 编译实例 RVAs: 0x695b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1398571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8d5a8eaff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375615366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-191243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8f514fdff","size":5,"soft_fail":false,"target_va":5375423344,"va":5375614582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8d2860100","size":5,"soft_fail":false,"target_va":5375715360,"va":5375615305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406aeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e889890100","size":5,"soft_fail":false,"target_va":5375716032,"va":5375615282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t351700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8d45d0500","size":5,"soft_fail":false,"target_va":5375967552,"va":5375615847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7733957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8c5027600","size":5,"soft_fail":false,"target_va":5383349424,"va":5375615462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfc620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7758940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e85c647600","size":5,"soft_fail":false,"target_va":5383374368,"va":5375615423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfc910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7760314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8ba697600","size":5,"soft_fail":false,"target_va":5383375120,"va":5375614801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27342776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8b837a101","size":5,"soft_fail":false,"target_va":5402958528,"va":5375615747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27978681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8b9ebaa01","size":5,"soft_fail":false,"target_va":5403593344,"va":5375614658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27978629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e885ebaa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375614726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27363256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8b887a101","size":5,"soft_fail":false,"target_va":5402978880,"va":5375615619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27363346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e81288a101","size":5,"soft_fail":false,"target_va":5402979152,"va":5375615801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27363744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8a089a101","size":5,"soft_fail":false,"target_va":5402979344,"va":5375615595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t87998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8be570100","size":5,"soft_fail":false,"target_va":5375702768,"va":5375614765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t94946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e8e2720100","size":5,"soft_fail":false,"target_va":5375709840,"va":5375614889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t92024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375613824,"raw_hex":"e878670100","size":5,"soft_fail":false,"target_va":5375707584,"va":5375615555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375613824,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375614689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":41684,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":22528,"source_line":691,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":41476,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":22526,"source_line":299,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":43516,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":43324,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":41648,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":485,"record_offset":45032,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::RenderViewIndex::is_shadow_view
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":485,"record_offset":41440,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":26229,"source_line":66,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":485,"record_offset":46852,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":21973,"source_line":82,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":46928,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":485,"record_offset":45308,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":27352,"source_line":577,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":43432,"parent_record_offset":41216,"ancestor_record_offset":41216,"inlinee":21969,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":42820,"parent_record_offset":42780,"ancestor_record_offset":41216,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":41724,"parent_record_offset":41684,"ancestor_record_offset":41216,"inlinee":22529,"source_line":713,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":485,"record_offset":43544,"parent_record_offset":43516,"ancestor_record_offset":41216,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":485,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":41216,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":43284,"parent_record_offset":43260,"ancestor_record_offset":41216,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":44988,"parent_record_offset":44956,"ancestor_record_offset":41216,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":44932,"parent_record_offset":44544,"ancestor_record_offset":41216,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":44956,"parent_record_offset":44932,"ancestor_record_offset":41216,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":44572,"parent_record_offset":44544,"ancestor_record_offset":41216,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":42944,"parent_record_offset":42916,"ancestor_record_offset":41216,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":42916,"parent_record_offset":42780,"ancestor_record_offset":41216,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":44620,"parent_record_offset":44596,"ancestor_record_offset":41216,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":44596,"parent_record_offset":44572,"ancestor_record_offset":41216,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":44712,"parent_record_offset":44648,"ancestor_record_offset":41216,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":43036,"parent_record_offset":42972,"ancestor_record_offset":41216,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":44800,"parent_record_offset":44776,"ancestor_record_offset":41216,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":43124,"parent_record_offset":43100,"ancestor_record_offset":41216,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":43260,"parent_record_offset":42708,"ancestor_record_offset":41216,"inlinee":22535,"source_line":383,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$7::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":485,"record_offset":42780,"parent_record_offset":42748,"ancestor_record_offset":41216,"inlinee":22534,"source_line":904,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":42748,"parent_record_offset":42708,"ancestor_record_offset":41216,"inlinee":22532,"source_line":910,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":44544,"parent_record_offset":43516,"ancestor_record_offset":41216,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":42708,"parent_record_offset":41684,"ancestor_record_offset":41216,"inlinee":22531,"source_line":942,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":42468,"parent_record_offset":42444,"ancestor_record_offset":41216,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":41608,"parent_record_offset":41584,"ancestor_record_offset":41216,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":42516,"parent_record_offset":42492,"ancestor_record_offset":41216,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":42444,"parent_record_offset":42420,"ancestor_record_offset":41216,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":41932,"parent_record_offset":41908,"ancestor_record_offset":41216,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":485,"record_offset":42016,"parent_record_offset":41992,"ancestor_record_offset":41216,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":485,"record_offset":41956,"parent_record_offset":41932,"ancestor_record_offset":41216,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":42128,"parent_record_offset":42104,"ancestor_record_offset":41216,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":485,"record_offset":41584,"parent_record_offset":41500,"ancestor_record_offset":41216,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":43060,"parent_record_offset":43036,"ancestor_record_offset":41216,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":44512,"parent_record_offset":43544,"ancestor_record_offset":41216,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":43100,"parent_record_offset":42916,"ancestor_record_offset":41216,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":43356,"parent_record_offset":43324,"ancestor_record_offset":41216,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":43388,"parent_record_offset":43356,"ancestor_record_offset":41216,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":43204,"parent_record_offset":43180,"ancestor_record_offset":41216,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":42356,"parent_record_offset":42332,"ancestor_record_offset":41216,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":485,"record_offset":42276,"parent_record_offset":42252,"ancestor_record_offset":41216,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":485,"record_offset":45184,"parent_record_offset":45156,"ancestor_record_offset":41216,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":485,"record_offset":45212,"parent_record_offset":45184,"ancestor_record_offset":41216,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":41528,"parent_record_offset":41500,"ancestor_record_offset":41216,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":42848,"parent_record_offset":42820,"ancestor_record_offset":41216,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":41552,"parent_record_offset":41528,"ancestor_record_offset":41216,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":43180,"parent_record_offset":43148,"ancestor_record_offset":41216,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":42876,"parent_record_offset":42848,"ancestor_record_offset":41216,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":485,"record_offset":45128,"parent_record_offset":45100,"ancestor_record_offset":41216,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":485,"record_offset":42492,"parent_record_offset":42468,"ancestor_record_offset":41216,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":485,"record_offset":45060,"parent_record_offset":45032,"ancestor_record_offset":41216,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":485,"record_offset":46880,"parent_record_offset":46852,"ancestor_record_offset":41216,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":485,"record_offset":42420,"parent_record_offset":42396,"ancestor_record_offset":41216,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":41752,"parent_record_offset":41724,"ancestor_record_offset":41216,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":41780,"parent_record_offset":41752,"ancestor_record_offset":41216,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":485,"record_offset":42396,"parent_record_offset":42196,"ancestor_record_offset":41216,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":42308,"parent_record_offset":42224,"ancestor_record_offset":41216,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":42252,"parent_record_offset":42224,"ancestor_record_offset":41216,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":42652,"parent_record_offset":41860,"ancestor_record_offset":41216,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":42224,"parent_record_offset":42196,"ancestor_record_offset":41216,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":41836,"parent_record_offset":41812,"ancestor_record_offset":41216,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":41812,"parent_record_offset":41780,"ancestor_record_offset":41216,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":41860,"parent_record_offset":41836,"ancestor_record_offset":41216,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":42076,"parent_record_offset":42048,"ancestor_record_offset":41216,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":42104,"parent_record_offset":42076,"ancestor_record_offset":41216,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":42164,"parent_record_offset":42048,"ancestor_record_offset":41216,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":485,"record_offset":42048,"parent_record_offset":41860,"ancestor_record_offset":41216,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":485,"record_offset":42196,"parent_record_offset":41860,"ancestor_record_offset":41216,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":41908,"parent_record_offset":41860,"ancestor_record_offset":41216,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":41992,"parent_record_offset":41860,"ancestor_record_offset":41216,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":485,"record_offset":42568,"parent_record_offset":41860,"ancestor_record_offset":41216,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":485,"record_offset":43480,"parent_record_offset":43456,"ancestor_record_offset":41216,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":43148,"parent_record_offset":43124,"ancestor_record_offset":41216,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":485,"record_offset":43456,"parent_record_offset":43432,"ancestor_record_offset":41216,"inlinee":21970,"source_line":613,"archive":"_pdb/A/modules/00485.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_bricks.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_bricks.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: draw_bricks.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":485,"record_offset":41052,"parent_record_offset":41024,"ancestor_record_offset":35416,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":485,"record_offset":46880,"parent_record_offset":46852,"ancestor_record_offset":41216,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00485.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_bricks.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>
// 信息 ID: dd1907e289526a5cdb6bad94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks -> ["0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>
// 信息 ID: 0707074480ec8fb3d2f05a4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks -> ["1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
