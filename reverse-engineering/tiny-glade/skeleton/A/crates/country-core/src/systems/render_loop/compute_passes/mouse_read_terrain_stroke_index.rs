// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b60868cd6db8780e3c3ee92651d64c822907f9dd152106221e7079ea2acabe8a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mouse_read_terrain_stroke_index.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 495
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
// 函数符号: country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":51256,"rva":7095936,"va":5375805056}
// 编译实例 RVAs: 0x6c4680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3055849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8175fd1ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375805540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3055878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8fa5ed1ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375805585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1589821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8c3bde7ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375805496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-491050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8d681f8ff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375805909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t172498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8d2a10200","size":5,"soft_fail":false,"target_va":5375978624,"va":5375806121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t433985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8419f0600","size":5,"soft_fail":false,"target_va":5376239920,"va":5375805930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7379118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8ae987000","size":5,"soft_fail":false,"target_va":5383184544,"va":5375805421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd60c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7411890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8b2187100","size":5,"soft_fail":false,"target_va":5383217344,"va":5375805449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mouse_read_terrain_stroke_index.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27787837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e83d02a801","size":5,"soft_fail":false,"target_va":5403593360,"va":5375805518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27172152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e8389d9e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375805987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27172903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375805056,"raw_hex":"e827a09e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375805972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":495,"record_offset":53808,"parent_record_offset":51256,"ancestor_record_offset":51256,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":495,"record_offset":53144,"parent_record_offset":51256,"ancestor_record_offset":51256,"inlinee":8420,"source_line":407,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":495,"record_offset":53096,"parent_record_offset":51256,"ancestor_record_offset":51256,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":495,"record_offset":53036,"parent_record_offset":51256,"ancestor_record_offset":51256,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":495,"record_offset":51472,"parent_record_offset":51256,"ancestor_record_offset":51256,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":53424,"parent_record_offset":53400,"ancestor_record_offset":51256,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":53448,"parent_record_offset":53424,"ancestor_record_offset":51256,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":53760,"parent_record_offset":53736,"ancestor_record_offset":51256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":53400,"parent_record_offset":53372,"ancestor_record_offset":51256,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":53736,"parent_record_offset":53680,"ancestor_record_offset":51256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":53708,"parent_record_offset":53680,"ancestor_record_offset":51256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":53372,"parent_record_offset":53344,"ancestor_record_offset":51256,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":53344,"parent_record_offset":53316,"ancestor_record_offset":51256,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":53520,"parent_record_offset":53344,"ancestor_record_offset":51256,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":53680,"parent_record_offset":53652,"ancestor_record_offset":51256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":53280,"parent_record_offset":53252,"ancestor_record_offset":51256,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":53252,"parent_record_offset":53224,"ancestor_record_offset":51256,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":53196,"parent_record_offset":53144,"ancestor_record_offset":51256,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":53224,"parent_record_offset":53196,"ancestor_record_offset":51256,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":53316,"parent_record_offset":53280,"ancestor_record_offset":51256,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":495,"record_offset":51528,"parent_record_offset":51500,"ancestor_record_offset":51256,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":495,"record_offset":52960,"parent_record_offset":52928,"ancestor_record_offset":51256,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":495,"record_offset":52904,"parent_record_offset":52516,"ancestor_record_offset":51256,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":495,"record_offset":52928,"parent_record_offset":52904,"ancestor_record_offset":51256,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":495,"record_offset":52544,"parent_record_offset":52516,"ancestor_record_offset":51256,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":495,"record_offset":52592,"parent_record_offset":52568,"ancestor_record_offset":51256,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":495,"record_offset":52568,"parent_record_offset":52544,"ancestor_record_offset":51256,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":495,"record_offset":52684,"parent_record_offset":52620,"ancestor_record_offset":51256,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":495,"record_offset":52772,"parent_record_offset":52748,"ancestor_record_offset":51256,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":495,"record_offset":51500,"parent_record_offset":51472,"ancestor_record_offset":51256,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":495,"record_offset":52516,"parent_record_offset":51500,"ancestor_record_offset":51256,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":495,"record_offset":52248,"parent_record_offset":52224,"ancestor_record_offset":51256,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":495,"record_offset":52296,"parent_record_offset":52272,"ancestor_record_offset":51256,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":495,"record_offset":52224,"parent_record_offset":52200,"ancestor_record_offset":51256,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":495,"record_offset":51716,"parent_record_offset":51692,"ancestor_record_offset":51256,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":495,"record_offset":51800,"parent_record_offset":51776,"ancestor_record_offset":51256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":495,"record_offset":51740,"parent_record_offset":51716,"ancestor_record_offset":51256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":495,"record_offset":51912,"parent_record_offset":51888,"ancestor_record_offset":51256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":52708,"parent_record_offset":52684,"ancestor_record_offset":51256,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":52484,"parent_record_offset":51528,"ancestor_record_offset":51256,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":52748,"parent_record_offset":52568,"ancestor_record_offset":51256,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":53004,"parent_record_offset":51472,"ancestor_record_offset":51256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":53060,"parent_record_offset":53036,"ancestor_record_offset":51256,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":495,"record_offset":52852,"parent_record_offset":52828,"ancestor_record_offset":51256,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":495,"record_offset":53556,"parent_record_offset":53280,"ancestor_record_offset":51256,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":53628,"parent_record_offset":53144,"ancestor_record_offset":51256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":495,"record_offset":52136,"parent_record_offset":52112,"ancestor_record_offset":51256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":495,"record_offset":52056,"parent_record_offset":52032,"ancestor_record_offset":51256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":53476,"parent_record_offset":53448,"ancestor_record_offset":51256,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":495,"record_offset":52620,"parent_record_offset":52592,"ancestor_record_offset":51256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":495,"record_offset":52656,"parent_record_offset":52620,"ancestor_record_offset":51256,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":495,"record_offset":52828,"parent_record_offset":52796,"ancestor_record_offset":51256,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":495,"record_offset":52272,"parent_record_offset":52248,"ancestor_record_offset":51256,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":495,"record_offset":53836,"parent_record_offset":53808,"ancestor_record_offset":51256,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":495,"record_offset":52200,"parent_record_offset":52176,"ancestor_record_offset":51256,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":495,"record_offset":51552,"parent_record_offset":51528,"ancestor_record_offset":51256,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":495,"record_offset":51576,"parent_record_offset":51552,"ancestor_record_offset":51256,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":495,"record_offset":52176,"parent_record_offset":51980,"ancestor_record_offset":51256,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":52088,"parent_record_offset":52004,"ancestor_record_offset":51256,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":52032,"parent_record_offset":52004,"ancestor_record_offset":51256,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":52432,"parent_record_offset":51652,"ancestor_record_offset":51256,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":52004,"parent_record_offset":51980,"ancestor_record_offset":51256,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":51628,"parent_record_offset":51604,"ancestor_record_offset":51256,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":51604,"parent_record_offset":51576,"ancestor_record_offset":51256,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":51652,"parent_record_offset":51628,"ancestor_record_offset":51256,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":495,"record_offset":51860,"parent_record_offset":51832,"ancestor_record_offset":51256,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":495,"record_offset":51888,"parent_record_offset":51860,"ancestor_record_offset":51256,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":495,"record_offset":51948,"parent_record_offset":51832,"ancestor_record_offset":51256,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":495,"record_offset":51832,"parent_record_offset":51652,"ancestor_record_offset":51256,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":495,"record_offset":51980,"parent_record_offset":51652,"ancestor_record_offset":51256,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":495,"record_offset":51692,"parent_record_offset":51652,"ancestor_record_offset":51256,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":495,"record_offset":51776,"parent_record_offset":51652,"ancestor_record_offset":51256,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":495,"record_offset":52348,"parent_record_offset":51652,"ancestor_record_offset":51256,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":495,"record_offset":52796,"parent_record_offset":52772,"ancestor_record_offset":51256,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00495.jsonl.gz"}
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
// 完整结构及原始类型记录: mouse_read_terrain_stroke_index.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_to_cpu_mapped_storage_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_to_cpu_mapped_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::render::ssbo::MappedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>", "constructor": "country_core::render::ssbo::impl$4::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mouse_read_terrain_stroke_index.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mouse_read_terrain_stroke_index.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":495,"record_offset":53096,"parent_record_offset":51256,"ancestor_record_offset":51256,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00495.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138084,"position_among_mapped_render_targets":9,"mapped_render_target_count":70,"previous_callsite":3138062,"next_callsite":3138106,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mouse_read_terrain_stroke_index.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index
// 信息 ID: 9457ba88a4235c1baa07bf04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe224 -> 0x6c4680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
