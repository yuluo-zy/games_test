// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 129e21b5bb0219e1d450c56485815d8cebdc4679fffc38b020a8c0f5d846314e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instanced_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 388
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
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":388,"record_offset":22356,"rva":5285920,"va":5373995040}
// 编译实例 RVAs: 0x50a820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t221201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e811600300","size":5,"soft_fail":false,"target_va":5374216800,"va":5373995594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t222974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e8fe660300","size":5,"soft_fail":false,"target_va":5374218416,"va":5373995437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406aa9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1703517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e85dfe1900","size":5,"soft_fail":false,"target_va":5375699376,"va":5373995854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1971590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e886151e00","size":5,"soft_fail":false,"target_va":5375967552,"va":5373995957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9353625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e899b98e00","size":5,"soft_fail":false,"target_va":5383349424,"va":5373995794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df8580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9362064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e890da8e00","size":5,"soft_fail":false,"target_va":5383357824,"va":5373995755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df9a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9367586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e822f08e00","size":5,"soft_fail":false,"target_va":5383363088,"va":5373995497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfba60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9375819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e84b108f00","size":5,"soft_fail":false,"target_va":5383371360,"va":5373995536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfc620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9378719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e89f1b8f00","size":5,"soft_fail":false,"target_va":5383374368,"va":5373995644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29597969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e811a1c301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373995386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28982985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373995040,"raw_hex":"e8c93eba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373995890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":24136,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":388,"record_offset":24328,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":21973,"source_line":82,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":24404,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":388,"record_offset":24252,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":388,"record_offset":22572,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":24164,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":21969,"source_line":613,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":388,"record_offset":22628,"parent_record_offset":22600,"ancestor_record_offset":22356,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":388,"record_offset":24060,"parent_record_offset":24028,"ancestor_record_offset":22356,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":24004,"parent_record_offset":23616,"ancestor_record_offset":22356,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":24028,"parent_record_offset":24004,"ancestor_record_offset":22356,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":23644,"parent_record_offset":23616,"ancestor_record_offset":22356,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":22356,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":23668,"parent_record_offset":23644,"ancestor_record_offset":22356,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":23784,"parent_record_offset":23720,"ancestor_record_offset":22356,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":22356,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":388,"record_offset":22600,"parent_record_offset":22572,"ancestor_record_offset":22356,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":388,"record_offset":23616,"parent_record_offset":22600,"ancestor_record_offset":22356,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":388,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":22356,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":388,"record_offset":23396,"parent_record_offset":23372,"ancestor_record_offset":22356,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":388,"record_offset":23324,"parent_record_offset":23300,"ancestor_record_offset":22356,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":388,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22356,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":388,"record_offset":22900,"parent_record_offset":22876,"ancestor_record_offset":22356,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":388,"record_offset":22840,"parent_record_offset":22816,"ancestor_record_offset":22356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":388,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":22356,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":22356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":23584,"parent_record_offset":22628,"ancestor_record_offset":22356,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":23848,"parent_record_offset":23668,"ancestor_record_offset":22356,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":24104,"parent_record_offset":22572,"ancestor_record_offset":22356,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":388,"record_offset":23952,"parent_record_offset":23928,"ancestor_record_offset":22356,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":23236,"parent_record_offset":23212,"ancestor_record_offset":22356,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":23156,"parent_record_offset":23132,"ancestor_record_offset":22356,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":23720,"parent_record_offset":23692,"ancestor_record_offset":22356,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":23756,"parent_record_offset":23720,"ancestor_record_offset":22356,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":23928,"parent_record_offset":23896,"ancestor_record_offset":22356,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":388,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":22356,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":24280,"parent_record_offset":24252,"ancestor_record_offset":22356,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":388,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":22356,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":22356,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":22356,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":23276,"parent_record_offset":23080,"ancestor_record_offset":22356,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":23188,"parent_record_offset":23104,"ancestor_record_offset":22356,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":23132,"parent_record_offset":23104,"ancestor_record_offset":22356,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":23532,"parent_record_offset":22752,"ancestor_record_offset":22356,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":23104,"parent_record_offset":23080,"ancestor_record_offset":22356,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":22728,"parent_record_offset":22704,"ancestor_record_offset":22356,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":22704,"parent_record_offset":22676,"ancestor_record_offset":22356,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":22752,"parent_record_offset":22728,"ancestor_record_offset":22356,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":22960,"parent_record_offset":22932,"ancestor_record_offset":22356,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":22988,"parent_record_offset":22960,"ancestor_record_offset":22356,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":23048,"parent_record_offset":22932,"ancestor_record_offset":22356,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":22932,"parent_record_offset":22752,"ancestor_record_offset":22356,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":23080,"parent_record_offset":22752,"ancestor_record_offset":22356,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":22792,"parent_record_offset":22752,"ancestor_record_offset":22356,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":22876,"parent_record_offset":22752,"ancestor_record_offset":22356,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":23448,"parent_record_offset":22752,"ancestor_record_offset":22356,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":24212,"parent_record_offset":24188,"ancestor_record_offset":22356,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":388,"record_offset":23896,"parent_record_offset":23872,"ancestor_record_offset":22356,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":24188,"parent_record_offset":24164,"ancestor_record_offset":22356,"inlinee":21970,"source_line":613,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":388,"record_offset":24452,"rva":5287040,"va":5373996160}
// 编译实例 RVAs: 0x50ac80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1426885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e8c5c51500","size":5,"soft_fail":false,"target_va":5375423344,"va":5373996454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1970762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e84a121e00","size":5,"soft_fail":false,"target_va":5375967552,"va":5373996785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfd1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9380748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e88c238f00","size":5,"soft_fail":false,"target_va":5383377376,"va":5373996623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28961794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e802ecb901","size":5,"soft_fail":false,"target_va":5402958528,"va":5373996729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29596813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e88d9cc301","size":5,"soft_fail":false,"target_va":5403593344,"va":5373996526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29596764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e85c9cc301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373996591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28982196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e8b43bba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373996679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1709702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996160,"raw_hex":"e886161a00","size":5,"soft_fail":false,"target_va":5375706352,"va":5373996645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373996160,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373996557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":26464,"parent_record_offset":24452,"ancestor_record_offset":24452,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":388,"record_offset":26224,"parent_record_offset":24452,"ancestor_record_offset":24452,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":26608,"parent_record_offset":24452,"ancestor_record_offset":24452,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":388,"record_offset":24660,"parent_record_offset":24452,"ancestor_record_offset":24452,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":388,"record_offset":24716,"parent_record_offset":24688,"ancestor_record_offset":24452,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":388,"record_offset":26148,"parent_record_offset":26116,"ancestor_record_offset":24452,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":26092,"parent_record_offset":25704,"ancestor_record_offset":24452,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":26116,"parent_record_offset":26092,"ancestor_record_offset":24452,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":25732,"parent_record_offset":25704,"ancestor_record_offset":24452,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":25780,"parent_record_offset":25756,"ancestor_record_offset":24452,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":25756,"parent_record_offset":25732,"ancestor_record_offset":24452,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":25872,"parent_record_offset":25808,"ancestor_record_offset":24452,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":25960,"parent_record_offset":25936,"ancestor_record_offset":24452,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":388,"record_offset":24688,"parent_record_offset":24660,"ancestor_record_offset":24452,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":388,"record_offset":25704,"parent_record_offset":24688,"ancestor_record_offset":24452,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":388,"record_offset":25436,"parent_record_offset":25412,"ancestor_record_offset":24452,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":388,"record_offset":25484,"parent_record_offset":25460,"ancestor_record_offset":24452,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":388,"record_offset":25412,"parent_record_offset":25388,"ancestor_record_offset":24452,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":388,"record_offset":24904,"parent_record_offset":24880,"ancestor_record_offset":24452,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":388,"record_offset":24988,"parent_record_offset":24964,"ancestor_record_offset":24452,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":388,"record_offset":24928,"parent_record_offset":24904,"ancestor_record_offset":24452,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":388,"record_offset":25100,"parent_record_offset":25076,"ancestor_record_offset":24452,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":25896,"parent_record_offset":25872,"ancestor_record_offset":24452,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":25672,"parent_record_offset":24716,"ancestor_record_offset":24452,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":25936,"parent_record_offset":25756,"ancestor_record_offset":24452,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":26192,"parent_record_offset":24660,"ancestor_record_offset":24452,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":388,"record_offset":26040,"parent_record_offset":26016,"ancestor_record_offset":24452,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":25324,"parent_record_offset":25300,"ancestor_record_offset":24452,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":25244,"parent_record_offset":25220,"ancestor_record_offset":24452,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":388,"record_offset":26376,"parent_record_offset":26348,"ancestor_record_offset":24452,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":388,"record_offset":26404,"parent_record_offset":26376,"ancestor_record_offset":24452,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":25808,"parent_record_offset":25780,"ancestor_record_offset":24452,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":25844,"parent_record_offset":25808,"ancestor_record_offset":24452,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":26016,"parent_record_offset":25984,"ancestor_record_offset":24452,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":26320,"parent_record_offset":26292,"ancestor_record_offset":24452,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":24452,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":24452,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":388,"record_offset":25460,"parent_record_offset":25436,"ancestor_record_offset":24452,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":388,"record_offset":26252,"parent_record_offset":26224,"ancestor_record_offset":24452,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":388,"record_offset":26516,"parent_record_offset":26492,"ancestor_record_offset":24452,"inlinee":21976,"source_line":371,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
//     evidence={"module":388,"record_offset":26492,"parent_record_offset":26464,"ancestor_record_offset":24452,"inlinee":21975,"source_line":56,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":388,"record_offset":25388,"parent_record_offset":25364,"ancestor_record_offset":24452,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":24740,"parent_record_offset":24716,"ancestor_record_offset":24452,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":24764,"parent_record_offset":24740,"ancestor_record_offset":24452,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":25364,"parent_record_offset":25168,"ancestor_record_offset":24452,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":25276,"parent_record_offset":25192,"ancestor_record_offset":24452,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":25220,"parent_record_offset":25192,"ancestor_record_offset":24452,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":25620,"parent_record_offset":24840,"ancestor_record_offset":24452,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":25192,"parent_record_offset":25168,"ancestor_record_offset":24452,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":24816,"parent_record_offset":24792,"ancestor_record_offset":24452,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":24792,"parent_record_offset":24764,"ancestor_record_offset":24452,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":24840,"parent_record_offset":24816,"ancestor_record_offset":24452,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":25048,"parent_record_offset":25020,"ancestor_record_offset":24452,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":25076,"parent_record_offset":25048,"ancestor_record_offset":24452,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":25136,"parent_record_offset":25020,"ancestor_record_offset":24452,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":25020,"parent_record_offset":24840,"ancestor_record_offset":24452,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":25168,"parent_record_offset":24840,"ancestor_record_offset":24452,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":24880,"parent_record_offset":24840,"ancestor_record_offset":24452,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":24964,"parent_record_offset":24840,"ancestor_record_offset":24452,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":25536,"parent_record_offset":24840,"ancestor_record_offset":24452,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":388,"record_offset":25984,"parent_record_offset":25960,"ancestor_record_offset":24452,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55D7
// 首条位置: {"module":388,"inlinee":21975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":388,"record_offset":26516,"parent_record_offset":26492,"ancestor_record_offset":26492,"inlinee":21976,"source_line":371,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":26492,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":26492,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":388,"record_offset":26640,"rva":5287680,"va":5373996800}
// 编译实例 RVAs: 0x50af00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t221132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8cc5f0300","size":5,"soft_fail":false,"target_va":5374218416,"va":5373997279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1426245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e845c31500","size":5,"soft_fail":false,"target_va":5375423344,"va":5373997094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1970055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8870f1e00","size":5,"soft_fail":false,"target_va":5375967552,"va":5373997492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfd7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9381539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8a3268f00","size":5,"soft_fail":false,"target_va":5383378880,"va":5373997336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28961078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e836e9b901","size":5,"soft_fail":false,"target_va":5402958528,"va":5373997445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29596179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8139ac301","size":5,"soft_fail":false,"target_va":5403593344,"va":5373997160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29596127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8df99c301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373997228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28981483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8eb38ba01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373997392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1708989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373996800,"raw_hex":"e8bd131a00","size":5,"soft_fail":false,"target_va":5375706352,"va":5373997358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373996800,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373997191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":28664,"parent_record_offset":26640,"ancestor_record_offset":26640,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":388,"record_offset":28424,"parent_record_offset":26640,"ancestor_record_offset":26640,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":28884,"parent_record_offset":26640,"ancestor_record_offset":26640,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":388,"record_offset":28808,"parent_record_offset":26640,"ancestor_record_offset":26640,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":388,"record_offset":26860,"parent_record_offset":26640,"ancestor_record_offset":26640,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":388,"record_offset":26916,"parent_record_offset":26888,"ancestor_record_offset":26640,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":388,"record_offset":28348,"parent_record_offset":28316,"ancestor_record_offset":26640,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":28292,"parent_record_offset":27904,"ancestor_record_offset":26640,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":28316,"parent_record_offset":28292,"ancestor_record_offset":26640,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":388,"record_offset":27932,"parent_record_offset":27904,"ancestor_record_offset":26640,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":27980,"parent_record_offset":27956,"ancestor_record_offset":26640,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":27956,"parent_record_offset":27932,"ancestor_record_offset":26640,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":28072,"parent_record_offset":28008,"ancestor_record_offset":26640,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":388,"record_offset":28160,"parent_record_offset":28136,"ancestor_record_offset":26640,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":388,"record_offset":26888,"parent_record_offset":26860,"ancestor_record_offset":26640,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":388,"record_offset":27904,"parent_record_offset":26888,"ancestor_record_offset":26640,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":388,"record_offset":27636,"parent_record_offset":27612,"ancestor_record_offset":26640,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":388,"record_offset":27684,"parent_record_offset":27660,"ancestor_record_offset":26640,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":388,"record_offset":27612,"parent_record_offset":27588,"ancestor_record_offset":26640,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":388,"record_offset":27104,"parent_record_offset":27080,"ancestor_record_offset":26640,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":388,"record_offset":27188,"parent_record_offset":27164,"ancestor_record_offset":26640,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":388,"record_offset":27128,"parent_record_offset":27104,"ancestor_record_offset":26640,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":388,"record_offset":27300,"parent_record_offset":27276,"ancestor_record_offset":26640,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":28096,"parent_record_offset":28072,"ancestor_record_offset":26640,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":27872,"parent_record_offset":26916,"ancestor_record_offset":26640,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":28136,"parent_record_offset":27956,"ancestor_record_offset":26640,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":28392,"parent_record_offset":26860,"ancestor_record_offset":26640,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":388,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":26640,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":27524,"parent_record_offset":27500,"ancestor_record_offset":26640,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":388,"record_offset":27444,"parent_record_offset":27420,"ancestor_record_offset":26640,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":388,"record_offset":28576,"parent_record_offset":28548,"ancestor_record_offset":26640,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":388,"record_offset":28604,"parent_record_offset":28576,"ancestor_record_offset":26640,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":388,"record_offset":28008,"parent_record_offset":27980,"ancestor_record_offset":26640,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":28044,"parent_record_offset":28008,"ancestor_record_offset":26640,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":388,"record_offset":28216,"parent_record_offset":28184,"ancestor_record_offset":26640,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":388,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":26640,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":28740,"parent_record_offset":28716,"ancestor_record_offset":26640,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":28764,"parent_record_offset":28740,"ancestor_record_offset":26640,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":388,"record_offset":27660,"parent_record_offset":27636,"ancestor_record_offset":26640,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":388,"record_offset":28452,"parent_record_offset":28424,"ancestor_record_offset":26640,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":388,"record_offset":28836,"parent_record_offset":28808,"ancestor_record_offset":26640,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":388,"record_offset":28716,"parent_record_offset":28692,"ancestor_record_offset":26640,"inlinee":21976,"source_line":371,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
//     evidence={"module":388,"record_offset":28692,"parent_record_offset":28664,"ancestor_record_offset":26640,"inlinee":21979,"source_line":69,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":388,"record_offset":27588,"parent_record_offset":27564,"ancestor_record_offset":26640,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":26940,"parent_record_offset":26916,"ancestor_record_offset":26640,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":26964,"parent_record_offset":26940,"ancestor_record_offset":26640,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":388,"record_offset":27564,"parent_record_offset":27368,"ancestor_record_offset":26640,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27476,"parent_record_offset":27392,"ancestor_record_offset":26640,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27420,"parent_record_offset":27392,"ancestor_record_offset":26640,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27820,"parent_record_offset":27040,"ancestor_record_offset":26640,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":26640,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27016,"parent_record_offset":26992,"ancestor_record_offset":26640,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":26992,"parent_record_offset":26964,"ancestor_record_offset":26640,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27040,"parent_record_offset":27016,"ancestor_record_offset":26640,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":27248,"parent_record_offset":27220,"ancestor_record_offset":26640,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":27276,"parent_record_offset":27248,"ancestor_record_offset":26640,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":27336,"parent_record_offset":27220,"ancestor_record_offset":26640,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":388,"record_offset":27220,"parent_record_offset":27040,"ancestor_record_offset":26640,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":388,"record_offset":27368,"parent_record_offset":27040,"ancestor_record_offset":26640,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":27080,"parent_record_offset":27040,"ancestor_record_offset":26640,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":27164,"parent_record_offset":27040,"ancestor_record_offset":26640,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":388,"record_offset":27736,"parent_record_offset":27040,"ancestor_record_offset":26640,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":388,"record_offset":28184,"parent_record_offset":28160,"ancestor_record_offset":26640,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55DB
// 首条位置: {"module":388,"inlinee":21979}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":388,"record_offset":28716,"parent_record_offset":28692,"ancestor_record_offset":28692,"inlinee":21976,"source_line":371,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":28740,"parent_record_offset":28716,"ancestor_record_offset":28692,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":388,"record_offset":28764,"parent_record_offset":28740,"ancestor_record_offset":28692,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00388.jsonl.gz"}
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
// 完整结构及原始类型记录: instanced_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instanced_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: instanced_roof.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":388,"record_offset":24252,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00388.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":388,"record_offset":24280,"parent_record_offset":24252,"ancestor_record_offset":22356,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":388,"record_offset":24404,"parent_record_offset":22356,"ancestor_record_offset":22356,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":388,"record_offset":28808,"parent_record_offset":26640,"ancestor_record_offset":26640,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00388.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":388,"record_offset":28836,"parent_record_offset":28808,"ancestor_record_offset":26640,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00388.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"9cfb7c252598d0bc9ae694b0","relative_path":"_instanced_roof_beams.raster.hlsl_b903e43ffb3da915.bin","sha256":"cd56c4bbb2fe11808612815f14dbb2d56a2feae404b076446265eb32cb8d3210","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"36084ca31334bf62f1085015","relative_path":"_instanced_roof_beams.raster.hlsl_f0adff76827e73ff.bin","sha256":"630e0dca51f130d49c928027a5c8267e54bb637fd184cc2ebc10ab96faa347af","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"0570e82875e79cd18b51d10b","relative_path":"_instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin","sha256":"243cd9c84899ee709cefc9e5351770ae9bb739c9c94d2c582a937a4d25367bbc","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"bd2ab6ef05a16238023daf14","relative_path":"_instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin","sha256":"8f6b028208a22a75320082d0bc10bdb411eb38826cd63e8c0950bc07cdcdf2f5","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"5d72167fd1afb2cbb6028e98","relative_path":"_roof_snow.raster.hlsl_b903e43ffb3da915.bin","sha256":"64174bdbfe1c96603ad1d3cbad662ba890e57278cfd3dbc7af8baa7c05f48f3a","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"84028b8814fc9f6b658eb2c6","relative_path":"_roof_snow.raster.hlsl_f0adff76827e73ff.bin","sha256":"e40a337cb2cb63d67f0f8a41b17c3cd165b82a26e0757e1ef75b67e81c293058","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"bb2033447dbdd6c6ba6ef132","relative_path":"_roof_tiles_gravity.cs.hlsl_cs_main_0.bin","sha256":"1a009092ca1cd2f6992eab26b39ddfd1b7655a35bd633ee901ab5702e10005e6","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10,11]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"frame_constants","storage_class":"Uniform","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[68,68,68,68,68,68,68,68,26,25,25,26,32,26,25,30,25,30,26,30,26,68,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":68,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":68,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":68,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":32,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":30,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":30,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":68,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"$Globals","storage_class":"Uniform","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"instanced_data","storage_class":"Uniform","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.StructuredBuffer.RoofSpire","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.roof_profile_mult","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"unknown","storage_class":"Output","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,68]},"decorations":[],"pointee":{"id":68,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C0","storage_class":"Output","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C1","storage_class":"Output","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C2","storage_class":"Output","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"out.var.C3","storage_class":"Output","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6,7]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,10]},"decorations":[],"pointee":{"id":10,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,10]},"decorations":[],"pointee":{"id":10,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C2","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,10]},"decorations":[],"pointee":{"id":10,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"in.var.C3","storage_class":"Input","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,8]},"decorations":[],"pointee":{"id":8,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":22,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,12]},"decorations":[],"pointee":{"id":12,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[9,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_beams.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[60,60,60,60,60,60,60,60,26,25,25,26,31,26,25,29,25,29,26,29,26,60,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":60,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":60,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":60,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":31,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":29,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":29,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":29,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"$Globals","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"color_library","storage_class":"Uniform","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":60,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":60,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":60,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":60,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":60,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":60,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":60,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":60,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":60,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":60,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":60,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":60,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"instanced_data","storage_class":"Uniform","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.StructuredBuffer.RoofSupportBeam","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_snow.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"frame_constants","storage_class":"Uniform","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[57,57,57,57,57,57,57,57,21,20,20,21,28,21,20,25,20,25,21,25,21,57,16]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":57,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":57,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":57,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":21,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":20,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":20,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":21,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":28,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":21,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":20,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":25,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":20,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":25,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":21,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":25,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":21,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":57,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":16,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[21,59,25,17,17]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":59,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"roof_pivot","type_id":25,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"is_circle","type_id":17,"decorations":[{"decoration":"Offset","operands":[92]}]},{"index":4,"name":"bool_render_shadows","type_id":17,"decorations":[{"decoration":"Offset","operands":[96]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_UV","storage_class":"Input","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,23]},"decorations":[],"pointee":{"id":23,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C3","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,9]},"decorations":[],"pointee":{"id":9,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[7,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,9]},"decorations":[],"pointee":{"id":9,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[7,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C3","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,9]},"decorations":[],"pointee":{"id":9,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[7,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,12]},"decorations":[],"pointee":{"id":12,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_snow.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"frame_constants","storage_class":"Uniform","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[57,57,57,57,57,57,57,57,21,20,20,21,28,21,20,25,20,25,21,25,21,57,16]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":57,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":57,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":57,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":21,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":20,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":20,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":21,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":28,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":21,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":20,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":25,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":20,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":25,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":21,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":25,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":21,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":57,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":16,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[21,59,25,17,17]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":59,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"roof_pivot","type_id":25,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"is_circle","type_id":17,"decorations":[{"decoration":"Offset","operands":[92]}]},{"index":4,"name":"bool_render_shadows","type_id":17,"decorations":[{"decoration":"Offset","operands":[96]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_UV","storage_class":"Input","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,23]},"decorations":[],"pointee":{"id":23,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C3","storage_class":"Output","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[20,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C3","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,5]},"decorations":[],"pointee":{"id":5,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[4,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_beams.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[60,60,60,60,60,60,60,60,26,25,25,26,31,26,25,29,25,29,26,29,26,60,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":60,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":60,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":60,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":31,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":29,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":29,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":29,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"$Globals","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"color_library","storage_class":"Uniform","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":60,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":60,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":60,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":60,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":60,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":60,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":60,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":60,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":60,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":60,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":60,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":60,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":60,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":60,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":60,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"instanced_data","storage_class":"Uniform","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.StructuredBuffer.RoofSupportBeam","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":18,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,8]},"decorations":[],"pointee":{"id":8,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":18,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,8]},"decorations":[],"pointee":{"id":8,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":19,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,10]},"decorations":[],"pointee":{"id":10,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[7,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_tiles_gravity.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"frame_constants_storage_buffer","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4]},"decorations":[],"pointee":{"id":4,"name":"type.StructuredBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[14]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":14,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":1,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"push_constants","storage_class":"PushConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,8]},"decorations":[],"pointee":{"id":8,"name":"type.ConstantBuffer.PushConstants","record":{"opcode":"OpTypeStruct","operands":[21,21,19,19,19]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"first_instance","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"instance_count","type_id":21,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"wall_height_delta","type_id":19,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":3,"name":"roof_pivot_y","type_id":19,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":4,"name":"animation_t","type_id":19,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"instanced_data","storage_class":"Uniform","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.RWStructuredBuffer.RoofShingle","record":{"opcode":"OpTypeStruct","operands":[15]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,59]},"decorations":[],"pointee":{"id":59,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[21,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10,11]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"frame_constants","storage_class":"Uniform","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[68,68,68,68,68,68,68,68,26,25,25,26,32,26,25,30,25,30,26,30,26,68,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":68,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":68,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":68,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":68,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":32,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":30,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":30,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":68,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"$Globals","storage_class":"Uniform","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"instanced_data","storage_class":"Uniform","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.StructuredBuffer.RoofSpire","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.roof_profile_mult","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"unknown","storage_class":"Output","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,68]},"decorations":[],"pointee":{"id":68,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C0","storage_class":"Output","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C1","storage_class":"Output","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C2","storage_class":"Output","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,30]},"decorations":[],"pointee":{"id":30,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"out.var.C3","storage_class":"Output","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instanced_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani
// 信息 ID: 87d1aa1d89804fc9802c4114
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader
// 信息 ID: 356d86731035500494ba340d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader::closure$0
// 信息 ID: 8f889a71932b3dd25ec0049a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader; module=388, inline_record=26492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader
// 信息 ID: 8dd447ffe50ce0bfa953d33d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader::closure$0
// 信息 ID: 13804803bc1235e8f241e08b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader; module=388, inline_record=28692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
