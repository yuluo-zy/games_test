// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/taa.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a99c73a0c4848b465602d121c8d445cc1271a4dabbc389136e0738c753732baf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: taa.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 451
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
// 函数符号: country_core::systems::render_loop::compute_passes::taa::calculate_taa
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":42356,"rva":6373680,"va":5375082800}
// 编译实例 RVAs: 0x614130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405acbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-423842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e85e88f9ff","size":5,"soft_fail":false,"target_va":5374659536,"va":5375083373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t191065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e859ea0200","size":5,"soft_fail":false,"target_va":5375274368,"va":5375083298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t192263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e807ef0200","size":5,"soft_fail":false,"target_va":5375275808,"va":5375083540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t230837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e8b5850300","size":5,"soft_fail":false,"target_va":5375314864,"va":5375084022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t894324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e874a50d00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375084295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1155865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e819a31100","size":5,"soft_fail":false,"target_va":5376239920,"va":5375084050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8101356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e8ec9d7b00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375083183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8145106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e8d2487c00","size":5,"soft_fail":false,"target_va":5383228352,"va":5375083241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8145448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e8284a7c00","size":5,"soft_fail":false,"target_va":5383229152,"va":5375083699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: taa.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28510029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e84d07b301","size":5,"soft_fail":false,"target_va":5403593360,"va":5375083326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27894757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e8e5a3a901","size":5,"soft_fail":false,"target_va":5402978880,"va":5375084118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1258037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375082800,"raw_hex":"e835321300","size":5,"soft_fail":false,"target_va":5376341760,"va":5375083718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":44360,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":44236,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":44092,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":451,"record_offset":42528,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":451,"record_offset":44332,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":451,"record_offset":42584,"parent_record_offset":42556,"ancestor_record_offset":42356,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":451,"record_offset":44016,"parent_record_offset":43984,"ancestor_record_offset":42356,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":451,"record_offset":43960,"parent_record_offset":43572,"ancestor_record_offset":42356,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":451,"record_offset":43984,"parent_record_offset":43960,"ancestor_record_offset":42356,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":451,"record_offset":43600,"parent_record_offset":43572,"ancestor_record_offset":42356,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":43648,"parent_record_offset":43624,"ancestor_record_offset":42356,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":43624,"parent_record_offset":43600,"ancestor_record_offset":42356,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":43740,"parent_record_offset":43676,"ancestor_record_offset":42356,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":43828,"parent_record_offset":43804,"ancestor_record_offset":42356,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":451,"record_offset":42556,"parent_record_offset":42528,"ancestor_record_offset":42356,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":451,"record_offset":43572,"parent_record_offset":42556,"ancestor_record_offset":42356,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":451,"record_offset":43304,"parent_record_offset":43280,"ancestor_record_offset":42356,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":43352,"parent_record_offset":43328,"ancestor_record_offset":42356,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":43280,"parent_record_offset":43256,"ancestor_record_offset":42356,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":451,"record_offset":42772,"parent_record_offset":42748,"ancestor_record_offset":42356,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":451,"record_offset":42856,"parent_record_offset":42832,"ancestor_record_offset":42356,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":42796,"parent_record_offset":42772,"ancestor_record_offset":42356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":451,"record_offset":42968,"parent_record_offset":42944,"ancestor_record_offset":42356,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":43764,"parent_record_offset":43740,"ancestor_record_offset":42356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":43540,"parent_record_offset":42584,"ancestor_record_offset":42356,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":43804,"parent_record_offset":43624,"ancestor_record_offset":42356,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":44060,"parent_record_offset":42528,"ancestor_record_offset":42356,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":43908,"parent_record_offset":43884,"ancestor_record_offset":42356,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":44452,"parent_record_offset":44388,"ancestor_record_offset":42356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":451,"record_offset":43192,"parent_record_offset":43168,"ancestor_record_offset":42356,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":451,"record_offset":43112,"parent_record_offset":43088,"ancestor_record_offset":42356,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":451,"record_offset":43676,"parent_record_offset":43648,"ancestor_record_offset":42356,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":451,"record_offset":43712,"parent_record_offset":43676,"ancestor_record_offset":42356,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":451,"record_offset":43884,"parent_record_offset":43852,"ancestor_record_offset":42356,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":451,"record_offset":43328,"parent_record_offset":43304,"ancestor_record_offset":42356,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":44388,"parent_record_offset":44360,"ancestor_record_offset":42356,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":451,"record_offset":43256,"parent_record_offset":43232,"ancestor_record_offset":42356,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":451,"record_offset":42608,"parent_record_offset":42584,"ancestor_record_offset":42356,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":451,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":42356,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":451,"record_offset":43232,"parent_record_offset":43036,"ancestor_record_offset":42356,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":43144,"parent_record_offset":43060,"ancestor_record_offset":42356,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":43088,"parent_record_offset":43060,"ancestor_record_offset":42356,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":43488,"parent_record_offset":42708,"ancestor_record_offset":42356,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":43060,"parent_record_offset":43036,"ancestor_record_offset":42356,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":42684,"parent_record_offset":42660,"ancestor_record_offset":42356,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":42660,"parent_record_offset":42632,"ancestor_record_offset":42356,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":42708,"parent_record_offset":42684,"ancestor_record_offset":42356,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":42916,"parent_record_offset":42888,"ancestor_record_offset":42356,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":42944,"parent_record_offset":42916,"ancestor_record_offset":42356,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":43004,"parent_record_offset":42888,"ancestor_record_offset":42356,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":42888,"parent_record_offset":42708,"ancestor_record_offset":42356,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":43036,"parent_record_offset":42708,"ancestor_record_offset":42356,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":451,"record_offset":42748,"parent_record_offset":42708,"ancestor_record_offset":42356,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":451,"record_offset":42832,"parent_record_offset":42708,"ancestor_record_offset":42356,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":451,"record_offset":43404,"parent_record_offset":42708,"ancestor_record_offset":42356,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":451,"record_offset":43852,"parent_record_offset":43828,"ancestor_record_offset":42356,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00451.jsonl.gz"}
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
// 完整结构及原始类型记录: taa.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: taa.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/taa.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: taa.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::taa::calculate_taa
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":451,"record_offset":44092,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00451.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":451,"record_offset":44236,"parent_record_offset":42356,"ancestor_record_offset":42356,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00451.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"68ec69d2886b38f74c2b1a7a","relative_path":"_taa.cs.hlsl_cs_main_0.bin","sha256":"c554db655bb9169329d15105923fe85583d66d7dc2b58ce72a72e453abe8a709","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"taa","prefix_candidates":["taa","taa","tgsr"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _taa.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"frame_constants","storage_class":"Uniform","type":{"id":84,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,7]},"decorations":[],"pointee":{"id":7,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[66,66,66,66,66,66,66,66,26,25,25,26,81,26,25,39,25,39,26,39,26,66,22]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":66,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":81,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":39,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":39,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":66,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":22,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"$Globals","storage_class":"Uniform","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[81,81,81,81,81,81,26,25,26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"color_tex_dims","type_id":81,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"history_tex_dims","type_id":81,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"reprojection_tex_dims","type_id":81,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"history_output_tex_dims","type_id":81,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"output_tex_dims","type_id":81,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"temp_debug_tex_dims","type_id":81,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"bypass_neighborhood_clamping","type_id":26,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"sharpen_amount","type_id":25,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":8,"name":"history_valid","type_id":26,"decorations":[{"decoration":"Offset","operands":[56]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"color_tex","storage_class":"UniformConstant","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,10]},"decorations":[],"pointee":{"id":10,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"history_tex","storage_class":"UniformConstant","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,10]},"decorations":[],"pointee":{"id":10,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,10]},"decorations":[],"pointee":{"id":10,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"history_output_tex","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,16]},"decorations":[],"pointee":{"id":16,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"output_tex","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,16]},"decorations":[],"pointee":{"id":16,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,91]},"decorations":[],"pointee":{"id":91,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[26,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,91]},"decorations":[],"pointee":{"id":91,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[26,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3141597,"position_among_mapped_render_targets":52,"mapped_render_target_count":70,"previous_callsite":3141193,"next_callsite":3141689,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: taa.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::taa::calculate_taa
// 信息 ID: 752a44f310292997a3cc34d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fefdd -> 0x614130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
