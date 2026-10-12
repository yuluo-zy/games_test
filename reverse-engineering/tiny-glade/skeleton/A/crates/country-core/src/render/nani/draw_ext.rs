// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/nani/draw_ext.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 16af0a16f6d653a14b537b52844299e86c25cadc4e8a9ef57b72a39dc06f205e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_ext.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 490
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
// 函数符号: country_core::render::nani::draw_ext::impl$0::nani_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":24968,"rva":6990256,"va":5375699376}
// 编译实例 RVAs: 0x6aa9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_ext.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t266878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e87e120400","size":5,"soft_fail":false,"target_va":5375967552,"va":5375700669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d33100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_ext.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6849593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e839846800","size":5,"soft_fail":false,"target_va":5382549760,"va":5375700162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d3b540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_ext.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6883479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e897086900","size":5,"soft_fail":false,"target_va":5382583616,"va":5375700132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfbd50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_ext.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7671888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e850107500","size":5,"soft_fail":false,"target_va":5383372112,"va":5375700219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: draw_ext.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27893165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e8ad9da901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375700190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27278316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e8ec3ba001","size":5,"soft_fail":false,"target_va":5402978880,"va":5375700559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e8e4210000","size":5,"soft_fail":false,"target_va":5375709040,"va":5375700359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e8941e0000","size":5,"soft_fail":false,"target_va":5375708352,"va":5375700519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375699376,"raw_hex":"e80f070000","size":5,"soft_fail":false,"target_va":5375702080,"va":5375700268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375699376,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375699803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::subset_mesh_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":28548,"parent_record_offset":24968,"ancestor_record_offset":24968,"inlinee":26142,"source_line":434,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::draw_list_index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":26692,"parent_record_offset":24968,"ancestor_record_offset":24968,"inlinee":27539,"source_line":187,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":28516,"parent_record_offset":24968,"ancestor_record_offset":24968,"inlinee":26097,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::impl$8::index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":26776,"parent_record_offset":24968,"ancestor_record_offset":24968,"inlinee":26145,"source_line":456,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":490,"record_offset":25128,"parent_record_offset":24968,"ancestor_record_offset":24968,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::rg_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":28460,"parent_record_offset":24968,"ancestor_record_offset":24968,"inlinee":7426,"source_line":32,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":490,"record_offset":25184,"parent_record_offset":25156,"ancestor_record_offset":24968,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":26616,"parent_record_offset":26584,"ancestor_record_offset":24968,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":26560,"parent_record_offset":26172,"ancestor_record_offset":24968,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":26584,"parent_record_offset":26560,"ancestor_record_offset":24968,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":26200,"parent_record_offset":26172,"ancestor_record_offset":24968,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":24968,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":24968,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":26340,"parent_record_offset":26276,"ancestor_record_offset":24968,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":26428,"parent_record_offset":26404,"ancestor_record_offset":24968,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":25156,"parent_record_offset":25128,"ancestor_record_offset":24968,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":26172,"parent_record_offset":25156,"ancestor_record_offset":24968,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":490,"record_offset":25904,"parent_record_offset":25880,"ancestor_record_offset":24968,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":25952,"parent_record_offset":25928,"ancestor_record_offset":24968,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":24968,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":490,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":24968,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":25372,"parent_record_offset":25348,"ancestor_record_offset":24968,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":25456,"parent_record_offset":25432,"ancestor_record_offset":24968,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":490,"record_offset":25396,"parent_record_offset":25372,"ancestor_record_offset":24968,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":490,"record_offset":25568,"parent_record_offset":25544,"ancestor_record_offset":24968,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":26364,"parent_record_offset":26340,"ancestor_record_offset":24968,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":26140,"parent_record_offset":25184,"ancestor_record_offset":24968,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":26404,"parent_record_offset":26224,"ancestor_record_offset":24968,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":28600,"parent_record_offset":28548,"ancestor_record_offset":24968,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":26660,"parent_record_offset":25128,"ancestor_record_offset":24968,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":490,"record_offset":26508,"parent_record_offset":26484,"ancestor_record_offset":24968,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":25792,"parent_record_offset":25768,"ancestor_record_offset":24968,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":25712,"parent_record_offset":25688,"ancestor_record_offset":24968,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":490,"record_offset":26276,"parent_record_offset":26248,"ancestor_record_offset":24968,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":26312,"parent_record_offset":26276,"ancestor_record_offset":24968,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":26484,"parent_record_offset":26452,"ancestor_record_offset":24968,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":490,"record_offset":25928,"parent_record_offset":25904,"ancestor_record_offset":24968,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubset::atlas_mesh_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":28652,"parent_record_offset":28628,"ancestor_record_offset":24968,"inlinee":26119,"source_line":488,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$2::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":26740,"parent_record_offset":26716,"ancestor_record_offset":24968,"inlinee":26128,"source_line":106,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$7::subset_mesh_count::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":490,"record_offset":28628,"parent_record_offset":28600,"ancestor_record_offset":24968,"inlinee":26117,"source_line":437,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":490,"record_offset":25856,"parent_record_offset":25832,"ancestor_record_offset":24968,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":25208,"parent_record_offset":25184,"ancestor_record_offset":24968,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":25232,"parent_record_offset":25208,"ancestor_record_offset":24968,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":25832,"parent_record_offset":25636,"ancestor_record_offset":24968,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25744,"parent_record_offset":25660,"ancestor_record_offset":24968,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25688,"parent_record_offset":25660,"ancestor_record_offset":24968,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":26088,"parent_record_offset":25308,"ancestor_record_offset":24968,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25660,"parent_record_offset":25636,"ancestor_record_offset":24968,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25284,"parent_record_offset":25260,"ancestor_record_offset":24968,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25260,"parent_record_offset":25232,"ancestor_record_offset":24968,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25308,"parent_record_offset":25284,"ancestor_record_offset":24968,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":25516,"parent_record_offset":25488,"ancestor_record_offset":24968,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":25544,"parent_record_offset":25516,"ancestor_record_offset":24968,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":25604,"parent_record_offset":25488,"ancestor_record_offset":24968,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":25488,"parent_record_offset":25308,"ancestor_record_offset":24968,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":25636,"parent_record_offset":25308,"ancestor_record_offset":24968,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":25348,"parent_record_offset":25308,"ancestor_record_offset":24968,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":25432,"parent_record_offset":25308,"ancestor_record_offset":24968,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":26004,"parent_record_offset":25308,"ancestor_record_offset":24968,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":490,"record_offset":26452,"parent_record_offset":26428,"ancestor_record_offset":24968,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00490.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_ext.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_ext.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_ext.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::render::nani::draw_ext::impl$0::nani_draw
// 信息 ID: 9ec5c883e54bfca268c72ad3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani @ 0x50ab4e -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$10,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5213df -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$40,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x521b8b -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$11,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x522e2a -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$12,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5239c8 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5256b1 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$13,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x527722 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$28,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x5295ae -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$0,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52b1f3 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$24,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52b4eb -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$29,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52b5f9 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$14,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52be30 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$9,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x52c21d -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader @ 0x5ee1b9 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_trunk @ 0x694582 -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_canopy @ 0x69501b -> 0x6aa9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
