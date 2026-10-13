// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cce6c1bdeecd368da969a924e15853b8825c6067b5a74173265194a18082ae26
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instanced_ivy.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 493
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":22212,"rva":7058512,"va":5375767632}
// 编译实例 RVAs: 0x6bb450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1551528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e85853e8ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375768323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1549753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e8475ae8ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375768164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t198918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e806090300","size":5,"soft_fail":false,"target_va":5375967552,"va":5375768629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfcc00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7607754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e8ca157400","size":5,"soft_fail":false,"target_va":5383375872,"va":5375768113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27825274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e87a94a801","size":5,"soft_fail":false,"target_va":5403593360,"va":5375768081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27210310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e846329f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375768565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-62182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767632,"raw_hex":"e81a0dffff","size":5,"soft_fail":false,"target_va":5375706352,"va":5375768529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":493,"record_offset":25352,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":493,"record_offset":25176,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::total_segment_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":493,"record_offset":23988,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":24201,"source_line":172,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":493,"record_offset":25100,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":493,"record_offset":22424,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":493,"record_offset":22480,"parent_record_offset":22452,"ancestor_record_offset":22212,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":493,"record_offset":23912,"parent_record_offset":23880,"ancestor_record_offset":22212,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":493,"record_offset":23856,"parent_record_offset":23468,"ancestor_record_offset":22212,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":493,"record_offset":23880,"parent_record_offset":23856,"ancestor_record_offset":22212,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":493,"record_offset":23496,"parent_record_offset":23468,"ancestor_record_offset":22212,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":23544,"parent_record_offset":23520,"ancestor_record_offset":22212,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":22212,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":23636,"parent_record_offset":23572,"ancestor_record_offset":22212,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":22212,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":493,"record_offset":22452,"parent_record_offset":22424,"ancestor_record_offset":22212,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":493,"record_offset":23468,"parent_record_offset":22452,"ancestor_record_offset":22212,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":493,"record_offset":23200,"parent_record_offset":23176,"ancestor_record_offset":22212,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":23248,"parent_record_offset":23224,"ancestor_record_offset":22212,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":23176,"parent_record_offset":23152,"ancestor_record_offset":22212,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":24200,"parent_record_offset":24172,"ancestor_record_offset":22212,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":22668,"parent_record_offset":22644,"ancestor_record_offset":22212,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":22752,"parent_record_offset":22728,"ancestor_record_offset":22212,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":493,"record_offset":22692,"parent_record_offset":22668,"ancestor_record_offset":22212,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":493,"record_offset":24800,"parent_record_offset":24748,"ancestor_record_offset":22212,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":493,"record_offset":22864,"parent_record_offset":22840,"ancestor_record_offset":22212,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":23660,"parent_record_offset":23636,"ancestor_record_offset":22212,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":23436,"parent_record_offset":22480,"ancestor_record_offset":22212,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":23700,"parent_record_offset":23520,"ancestor_record_offset":22212,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":23956,"parent_record_offset":22424,"ancestor_record_offset":22212,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":493,"record_offset":23804,"parent_record_offset":23780,"ancestor_record_offset":22212,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":23088,"parent_record_offset":23064,"ancestor_record_offset":22212,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":22212,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":493,"record_offset":23572,"parent_record_offset":23544,"ancestor_record_offset":22212,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":493,"record_offset":23608,"parent_record_offset":23572,"ancestor_record_offset":22212,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":493,"record_offset":23780,"parent_record_offset":23748,"ancestor_record_offset":22212,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":493,"record_offset":23224,"parent_record_offset":23200,"ancestor_record_offset":22212,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":493,"record_offset":25128,"parent_record_offset":25100,"ancestor_record_offset":22212,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":493,"record_offset":24772,"parent_record_offset":24748,"ancestor_record_offset":22212,"inlinee":23351,"source_line":50,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::impl$1::total_segment_count::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":493,"record_offset":24748,"parent_record_offset":24724,"ancestor_record_offset":22212,"inlinee":24170,"source_line":175,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":493,"record_offset":23152,"parent_record_offset":23128,"ancestor_record_offset":22212,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":22504,"parent_record_offset":22480,"ancestor_record_offset":22212,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":22528,"parent_record_offset":22504,"ancestor_record_offset":22212,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":22212,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":23128,"parent_record_offset":22932,"ancestor_record_offset":22212,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":24368,"parent_record_offset":24344,"ancestor_record_offset":22212,"inlinee":16170,"source_line":4742,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::fold::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":24724,"parent_record_offset":24416,"ancestor_record_offset":22212,"inlinee":24168,"source_line":4747,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24640,"parent_record_offset":24416,"ancestor_record_offset":22212,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::fold_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24416,"parent_record_offset":24392,"ancestor_record_offset":22212,"inlinee":24165,"source_line":3875,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24144,"parent_record_offset":24084,"ancestor_record_offset":22212,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24060,"parent_record_offset":24036,"ancestor_record_offset":22212,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24116,"parent_record_offset":24084,"ancestor_record_offset":22212,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":22212,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":24512,"parent_record_offset":24476,"ancestor_record_offset":22212,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":24548,"parent_record_offset":24512,"ancestor_record_offset":22212,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":24608,"parent_record_offset":24476,"ancestor_record_offset":22212,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":493,"record_offset":24476,"parent_record_offset":24416,"ancestor_record_offset":22212,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":493,"record_offset":24392,"parent_record_offset":24368,"ancestor_record_offset":22212,"inlinee":16171,"source_line":4151,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":23040,"parent_record_offset":22956,"ancestor_record_offset":22212,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":22984,"parent_record_offset":22956,"ancestor_record_offset":22212,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":23384,"parent_record_offset":22604,"ancestor_record_offset":22212,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":22212,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":22580,"parent_record_offset":22556,"ancestor_record_offset":22212,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":22556,"parent_record_offset":22528,"ancestor_record_offset":22212,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":22604,"parent_record_offset":22580,"ancestor_record_offset":22212,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":22812,"parent_record_offset":22784,"ancestor_record_offset":22212,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":22840,"parent_record_offset":22812,"ancestor_record_offset":22212,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":22900,"parent_record_offset":22784,"ancestor_record_offset":22212,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":22784,"parent_record_offset":22604,"ancestor_record_offset":22212,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":22932,"parent_record_offset":22604,"ancestor_record_offset":22212,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":493,"record_offset":22644,"parent_record_offset":22604,"ancestor_record_offset":22212,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":493,"record_offset":22728,"parent_record_offset":22604,"ancestor_record_offset":22212,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":493,"record_offset":23300,"parent_record_offset":22604,"ancestor_record_offset":22212,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":24172,"parent_record_offset":24144,"ancestor_record_offset":22212,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":493,"record_offset":24228,"parent_record_offset":24172,"ancestor_record_offset":22212,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":493,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":22212,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":493,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":22212,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":493,"record_offset":24344,"parent_record_offset":23988,"ancestor_record_offset":22212,"inlinee":16169,"source_line":2245,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":25384,"rva":7059840,"va":5375768960}
// 编译实例 RVAs: 0x6bb980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t43","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375768960,"raw_hex":"e92b000000","size":5,"soft_fail":false,"target_va":5375769024,"va":5375768976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":25568,"rva":7059872,"va":5375768992}
// 编译实例 RVAs: 0x6bb9a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t11","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375768992,"raw_hex":"e90b000000","size":5,"soft_fail":false,"target_va":5375769024,"va":5375769008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":25756,"rva":7059904,"va":5375769024}
// 编译实例 RVAs: 0x6bb9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1552809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e8574ee8ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375769604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1551050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e83655e8ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375769461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-346923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e8d5b4faff","size":5,"soft_fail":false,"target_va":5375423344,"va":5375770262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t196666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e83a000300","size":5,"soft_fail":false,"target_va":5375967552,"va":5375770881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7579590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e8c6a77300","size":5,"soft_fail":false,"target_va":5383349008,"va":5375769413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7580162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e802aa7300","size":5,"soft_fail":false,"target_va":5383349824,"va":5375769657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfcef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7606654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e87e117400","size":5,"soft_fail":false,"target_va":5383376624,"va":5375769965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfe380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7612168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e808277400","size":5,"soft_fail":false,"target_va":5383381888,"va":5375769715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27187752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e828da9e01","size":5,"soft_fail":false,"target_va":5402958528,"va":5375770771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27823001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e8998ba801","size":5,"soft_fail":false,"target_va":5403593344,"va":5375770338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27824270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e88e90a801","size":5,"soft_fail":false,"target_va":5403593360,"va":5375769085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27208208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e8102a9f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375770667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-64282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e8e604ffff","size":5,"soft_fail":false,"target_va":5375706352,"va":5375770629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7392049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375769024,"raw_hex":"e831cb7000","size":5,"soft_fail":false,"target_va":5383161424,"va":5375769370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375769024,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375770369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":31112,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":493,"record_offset":30872,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":493,"record_offset":31408,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":493,"record_offset":27600,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":493,"record_offset":27520,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":493,"record_offset":25956,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":493,"record_offset":26012,"parent_record_offset":25984,"ancestor_record_offset":25756,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":493,"record_offset":27444,"parent_record_offset":27412,"ancestor_record_offset":25756,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":493,"record_offset":27388,"parent_record_offset":27000,"ancestor_record_offset":25756,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":493,"record_offset":27412,"parent_record_offset":27388,"ancestor_record_offset":25756,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":493,"record_offset":27028,"parent_record_offset":27000,"ancestor_record_offset":25756,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":25756,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":27052,"parent_record_offset":27028,"ancestor_record_offset":25756,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":27168,"parent_record_offset":27104,"ancestor_record_offset":25756,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":493,"record_offset":27256,"parent_record_offset":27232,"ancestor_record_offset":25756,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":493,"record_offset":25984,"parent_record_offset":25956,"ancestor_record_offset":25756,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":493,"record_offset":27000,"parent_record_offset":25984,"ancestor_record_offset":25756,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":493,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":25756,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":26780,"parent_record_offset":26756,"ancestor_record_offset":25756,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":493,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":25756,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":26200,"parent_record_offset":26176,"ancestor_record_offset":25756,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":493,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":25756,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":493,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":25756,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":493,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":25756,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":25756,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":26968,"parent_record_offset":26012,"ancestor_record_offset":25756,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":27232,"parent_record_offset":27052,"ancestor_record_offset":25756,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":493,"record_offset":27488,"parent_record_offset":25956,"ancestor_record_offset":25756,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":493,"record_offset":27336,"parent_record_offset":27312,"ancestor_record_offset":25756,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":26620,"parent_record_offset":26596,"ancestor_record_offset":25756,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":25756,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":493,"record_offset":31024,"parent_record_offset":30996,"ancestor_record_offset":25756,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":493,"record_offset":31052,"parent_record_offset":31024,"ancestor_record_offset":25756,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":493,"record_offset":27104,"parent_record_offset":27076,"ancestor_record_offset":25756,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":493,"record_offset":27140,"parent_record_offset":27104,"ancestor_record_offset":25756,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":493,"record_offset":27312,"parent_record_offset":27280,"ancestor_record_offset":25756,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":493,"record_offset":30968,"parent_record_offset":30940,"ancestor_record_offset":25756,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":493,"record_offset":26756,"parent_record_offset":26732,"ancestor_record_offset":25756,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":493,"record_offset":30900,"parent_record_offset":30872,"ancestor_record_offset":25756,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":493,"record_offset":27548,"parent_record_offset":27520,"ancestor_record_offset":25756,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":493,"record_offset":26684,"parent_record_offset":26660,"ancestor_record_offset":25756,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":25756,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":26060,"parent_record_offset":26036,"ancestor_record_offset":25756,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":493,"record_offset":26660,"parent_record_offset":26464,"ancestor_record_offset":25756,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26572,"parent_record_offset":26488,"ancestor_record_offset":25756,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26516,"parent_record_offset":26488,"ancestor_record_offset":25756,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26916,"parent_record_offset":26136,"ancestor_record_offset":25756,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26488,"parent_record_offset":26464,"ancestor_record_offset":25756,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26112,"parent_record_offset":26088,"ancestor_record_offset":25756,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26088,"parent_record_offset":26060,"ancestor_record_offset":25756,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26136,"parent_record_offset":26112,"ancestor_record_offset":25756,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":26344,"parent_record_offset":26316,"ancestor_record_offset":25756,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":26372,"parent_record_offset":26344,"ancestor_record_offset":25756,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":26432,"parent_record_offset":26316,"ancestor_record_offset":25756,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":493,"record_offset":26316,"parent_record_offset":26136,"ancestor_record_offset":25756,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":493,"record_offset":26464,"parent_record_offset":26136,"ancestor_record_offset":25756,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":493,"record_offset":26176,"parent_record_offset":26136,"ancestor_record_offset":25756,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":493,"record_offset":26260,"parent_record_offset":26136,"ancestor_record_offset":25756,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":493,"record_offset":26832,"parent_record_offset":26136,"ancestor_record_offset":25756,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":493,"record_offset":27280,"parent_record_offset":27256,"ancestor_record_offset":25756,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00493.jsonl.gz"}
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
// 完整结构及原始类型记录: instanced_ivy.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instanced_ivy.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: instanced_ivy.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":493,"record_offset":25100,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00493.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":493,"record_offset":25128,"parent_record_offset":25100,"ancestor_record_offset":22212,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":493,"record_offset":25176,"parent_record_offset":22212,"ancestor_record_offset":22212,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":493,"record_offset":27520,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00493.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":493,"record_offset":27548,"parent_record_offset":27520,"ancestor_record_offset":25756,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":493,"record_offset":27600,"parent_record_offset":25756,"ancestor_record_offset":25756,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00493.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":7059840,"caller_aliases":{"alias_group_rva":7059840,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":7059856,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_jump_outside_procedure"}
// [父函数静态调用位置] {"caller_entry_rva":7059872,"caller_aliases":{"alias_group_rva":7059872,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":7059888,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_jump_outside_procedure"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instanced_ivy.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader
// 信息 ID: f1ac30ca6f5c6822b9011e9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader
// 信息 ID: 14edca0d716ec6aa09204bac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader
// 信息 ID: 1ac10a27a80fe8b5742f4e07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
// 信息 ID: 0039ba68517954fa100f8aa5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader @ 0x6bb990 -> 0x6bb9c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader @ 0x6bb9b0 -> 0x6bb9c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
