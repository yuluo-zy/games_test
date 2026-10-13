// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0df6d5dc1b2e228f720ecb78cbc94f38fc9fbb82a1f3dad93c6d729f61f28e84
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: displace_rocky_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 421
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":5324,"rva":5899232,"va":5374608352}
// 编译实例 RVAs: 0x5a03e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-393115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e86500faff","size":5,"soft_fail":false,"target_va":5374215680,"va":5374608790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t705518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e8eec30a00","size":5,"soft_fail":false,"target_va":5375314864,"va":5374609341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1369116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e81ce41400","size":5,"soft_fail":false,"target_va":5375978624,"va":5374609503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1630553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e859e11800","size":5,"soft_fail":false,"target_va":5376239920,"va":5374609362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8575842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e862db8200","size":5,"soft_fail":false,"target_va":5383184544,"va":5374608697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd1760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8589833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e809128300","size":5,"soft_fail":false,"target_va":5383198560,"va":5374608722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd3800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8598164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e894328300","size":5,"soft_fail":false,"target_va":5383206912,"va":5374608743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: displace_rocky_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28984537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e8d944ba01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374608818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28369484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374608352,"raw_hex":"e84ce2b001","size":5,"soft_fail":false,"target_va":5402978880,"va":5374609391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":421,"record_offset":7308,"parent_record_offset":5324,"ancestor_record_offset":5324,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":421,"record_offset":7164,"parent_record_offset":5324,"ancestor_record_offset":5324,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":421,"record_offset":7084,"parent_record_offset":5324,"ancestor_record_offset":5324,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":421,"record_offset":5520,"parent_record_offset":5324,"ancestor_record_offset":5324,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":421,"record_offset":5576,"parent_record_offset":5548,"ancestor_record_offset":5324,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":421,"record_offset":7008,"parent_record_offset":6976,"ancestor_record_offset":5324,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":421,"record_offset":6952,"parent_record_offset":6564,"ancestor_record_offset":5324,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":421,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":5324,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":421,"record_offset":6592,"parent_record_offset":6564,"ancestor_record_offset":5324,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":421,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":5324,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":421,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":5324,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":421,"record_offset":6732,"parent_record_offset":6668,"ancestor_record_offset":5324,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":421,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":5324,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":421,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":5324,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":421,"record_offset":6564,"parent_record_offset":5548,"ancestor_record_offset":5324,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":421,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":5324,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":421,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":5324,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":421,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5324,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":421,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":5324,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":421,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":5324,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":421,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5324,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":421,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":5324,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":5324,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":6532,"parent_record_offset":5576,"ancestor_record_offset":5324,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":6796,"parent_record_offset":6616,"ancestor_record_offset":5324,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":7052,"parent_record_offset":5520,"ancestor_record_offset":5324,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":421,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5324,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":421,"record_offset":7408,"parent_record_offset":7336,"ancestor_record_offset":5324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":421,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":5324,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":421,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5324,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":6668,"parent_record_offset":6640,"ancestor_record_offset":5324,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":6704,"parent_record_offset":6668,"ancestor_record_offset":5324,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":6876,"parent_record_offset":6844,"ancestor_record_offset":5324,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":421,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":5324,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":421,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":5324,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":421,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":5324,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":421,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":5324,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":5324,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":5324,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":6224,"parent_record_offset":6028,"ancestor_record_offset":5324,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":6136,"parent_record_offset":6052,"ancestor_record_offset":5324,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":5324,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":6480,"parent_record_offset":5700,"ancestor_record_offset":5324,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":5324,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":5324,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":5324,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":5324,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":421,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":5324,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":421,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":5324,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":421,"record_offset":5996,"parent_record_offset":5880,"ancestor_record_offset":5324,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":421,"record_offset":5880,"parent_record_offset":5700,"ancestor_record_offset":5324,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":421,"record_offset":6028,"parent_record_offset":5700,"ancestor_record_offset":5324,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":421,"record_offset":5740,"parent_record_offset":5700,"ancestor_record_offset":5324,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":421,"record_offset":5824,"parent_record_offset":5700,"ancestor_record_offset":5324,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":421,"record_offset":6396,"parent_record_offset":5700,"ancestor_record_offset":5324,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":421,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":5324,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00421.jsonl.gz"}
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
// 完整结构及原始类型记录: displace_rocky_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: displace_rocky_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/displace_rocky_terrain.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: displace_rocky_terrain.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":421,"record_offset":7084,"parent_record_offset":5324,"ancestor_record_offset":5324,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00421.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":421,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":5324,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00421.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":421,"record_offset":7164,"parent_record_offset":5324,"ancestor_record_offset":5324,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00421.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138370,"position_among_mapped_render_targets":22,"mapped_render_target_count":70,"previous_callsite":3138348,"next_callsite":3138392,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/stairs.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: displace_rocky_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
// 信息 ID: 83201d54ff01e1597fe05257
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe342 -> 0x5a03e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
